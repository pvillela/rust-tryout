//! This captures both total and sync timings:
//! - total timings include suspend time and are based on span creation and closing;
//! - active timings exclude suspend time and are based on span entry and exit.

use hdrhistogram::{
    sync::{Recorder, SyncHistogram},
    Histogram,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    future::Future,
    ops::Deref,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use tracing::{
    callsite::Identifier,
    info, instrument,
    subscriber::{Interest, Subscriber},
    warn, Id, Instrument, Metadata,
};
use tracing_core::span::Attributes;
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer, Registry,
};

#[derive(Debug)]
pub struct CallsiteTiming {
    span_name: String,
    total_time: SyncHistogram<u64>,
    active_time: SyncHistogram<u64>,
}

struct CallsiteRecorder {
    total_time: Recorder<u64>,
    active_time: Recorder<u64>,
}

#[derive(Debug)]
struct SpanTiming {
    // callsite: Identifier,
    created_at: Instant,
    entered_at: Instant,
    acc_active_time: u64,
}

/// Collects counts emitted by application spans and events.
#[derive(Debug)]
struct Timings {
    callsite_timings: RwLock<HashMap<Identifier, CallsiteTiming>>,
}

pub struct Latencies(Arc<Timings>);

impl Clone for Latencies {
    fn clone(&self) -> Self {
        Latencies(self.0.clone())
    }
}

impl Latencies {
    pub fn new() -> Latencies {
        let timing_by_span = RwLock::new(HashMap::new());
        let timings = Timings {
            callsite_timings: timing_by_span,
        };
        Latencies(Arc::new(timings))
    }

    pub fn read<'a>(&'a self) -> impl Deref<Target = HashMap<Identifier, CallsiteTiming>> + 'a {
        for (_, v) in self.0.callsite_timings.write().unwrap().iter_mut() {
            v.total_time.refresh();
            v.active_time.refresh();
        }
        self.0.callsite_timings.read().unwrap()
    }

    pub fn print_mean_timings(&self) {
        let timings = self.read();
        println!("\nMean timing values by span:");
        for (_, v) in timings.iter() {
            let mean_total_time = v.total_time.mean();
            let mean_active_time = v.active_time.mean();
            let total_time_count = v.total_time.len();
            let active_time_count = v.active_time.len();
            println!(
                "  span_name={}, mean_total_time={}μs, total_time_count={}, mean_active_time={}μs, active_time_count={}",
                v.span_name, mean_total_time, total_time_count, mean_active_time,active_time_count
            );
        }
    }

    fn with_recorder(&self, id: &Identifier, f: impl Fn(&mut CallsiteRecorder) -> ()) {
        thread_local! {
            static RECORDERS: RefCell<HashMap<Identifier, CallsiteRecorder>> = RefCell::new(HashMap::new());
        }

        RECORDERS.with(|m| {
            let mut callsite_recorders = m.borrow_mut();
            let mut recorder = callsite_recorders.entry(id.clone()).or_insert_with(|| {
                println!(
                    "***** thread-loacal CallsiteRecorder created for callsite={:?} on thread={:?}",
                    id,
                    thread::current().id()
                );

                let callsite_timings = self.0.callsite_timings.read().unwrap();
                let callsite_timing = callsite_timings.get(&id).unwrap();

                CallsiteRecorder {
                    total_time: callsite_timing.total_time.recorder(),
                    active_time: callsite_timing.active_time.recorder(),
                }
            });

            f(&mut recorder);
        });
    }
}

impl<S> Layer<S> for Latencies
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        //println!("`register_callsite` entered");
        if !meta.is_span() {
            return Interest::never();
        }

        let meta_name = meta.name();
        let callsite = meta.callsite();
        let interest = Interest::always();

        let mut map = self.0.callsite_timings.write().unwrap();

        let mut hist = Histogram::<u64>::new_with_bounds(1, 60 * 1000, 1).unwrap();
        hist.auto(true);
        let hist2 = hist.clone();
        let hist: SyncHistogram<u64> = hist.into();
        let hist2: SyncHistogram<u64> = hist2.into();

        map.insert(
            callsite.clone(),
            CallsiteTiming {
                span_name: meta_name.to_owned(),
                total_time: hist,
                active_time: hist2,
            },
        );

        //println!(
        //     "`register_callsite` executed with id={:?}, meta_name={}",
        //     callsite, meta_name
        // );

        interest
    }

    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        //println!("`new_span` entered");
        let span = ctx.span(id).unwrap();
        span.extensions_mut().insert(SpanTiming {
            created_at: Instant::now(),
            entered_at: Instant::now(),
            acc_active_time: 0,
        });
        //println!("`new_span` executed with id={:?}", id);
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        //println!("entered `enter` wth span Id {:?}", id);
        let span = ctx.span(id).unwrap();
        let mut ext = span.extensions_mut();
        let span_timing = ext.get_mut::<SpanTiming>().unwrap();
        span_timing.entered_at = Instant::now();
        //println!("`enter` executed with id={:?}", id);
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        //println!("entered `exit` wth span Id {:?}", id);
        let span = ctx.span(id).unwrap();
        let mut ext = span.extensions_mut();
        let span_timing = ext.get_mut::<SpanTiming>().unwrap();
        span_timing.acc_active_time += (Instant::now() - span_timing.entered_at).as_micros() as u64;
        //println!("`try_close` executed for span id {:?}", id);
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        //println!("entered `try_close` wth span Id {:?}", id);

        let span = ctx.span(&id).unwrap();
        let callsite = span.metadata().callsite();
        let ext = span.extensions();
        let span_timing = ext.get::<SpanTiming>().unwrap();

        self.with_recorder(&callsite, |r| {
            r.total_time
                .record((Instant::now() - span_timing.created_at).as_micros() as u64)
                .unwrap();
            r.active_time.record(span_timing.acc_active_time).unwrap();
        });

        //println!("`try_close` executed for span id {:?}", id);
    }
}

/// Measures latencies of spans in `f`.
/// May only be called once per process and will panic if called more than once.
pub fn measure_latencies(f: impl FnOnce() -> () + Send) -> Latencies {
    let latencies = Latencies::new();

    Registry::default().with(latencies.clone()).init();

    thread::scope(|s| {
        s.spawn(f);
    });

    latencies
}

/// Measures latencies of spans in async function `f` running on the [tokio] runtime.
/// May only be called once per process and will panic if called more than once.
pub fn measure_latencies_tokio<F>(f: impl FnOnce() -> F + Send) -> Latencies
where
    F: Future<Output = ()> + Send,
{
    measure_latencies(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                f().await;
            });
    })
}

#[instrument(level = "trace")]
async fn f() {
    let mut foo: u64 = 1;

    for _ in 0..4 {
        println!("Before my_great_span");

        async {
            thread::sleep(Duration::from_millis(3));
            tokio::time::sleep(Duration::from_millis(100)).await;
            foo += 1;
            info!(yak_shaved = true, yak_count = 2, "hi from inside my span");
            println!("Before my_other_span");
            async {
                thread::sleep(Duration::from_millis(2));
                tokio::time::sleep(Duration::from_millis(25)).await;
                warn!(yak_shaved = false, yak_count = -1, "failed to shave yak");
            }
            .instrument(tracing::trace_span!("my_other_span"))
            .await;
        }
        .instrument(tracing::trace_span!("my_great_span"))
        .await
    }
}

fn main() {
    let latencies = measure_latencies_tokio(|| async {
        f().await;
        f().await;
    });

    latencies.print_mean_timings();

    let timings = latencies.read();
    println!("\nMedian timings by span:");
    for (_, v) in timings.iter() {
        let median_total_time = v.total_time.value_at_quantile(0.5);
        let median_active_time = v.active_time.value_at_quantile(0.5);
        let total_time_count = v.total_time.len();
        let active_time_count = v.active_time.len();
        println!(
            "  span_name={}, median_total_time={}μs, total_time_count={}, median_active_time={}μs, active_time_count={}",
            v.span_name, median_total_time, total_time_count, median_active_time,active_time_count
        );
    }
}
