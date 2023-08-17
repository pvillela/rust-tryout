//! This captures both total and sync timings:
//! - total timings include suspend time and are based on span creation and closing;
//! - active timings exclude suspend time and are based on span entry and exit.

use env_logger;
use hdrhistogram::{
    sync::{Recorder, SyncHistogram},
    Histogram,
};
use log;
use std::{
    cell::RefCell,
    collections::HashMap,
    env::set_var,
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

//=================
// Types

/// Globally collected information for a callsite.
#[derive(Debug)]
pub struct CallsiteTiming {
    pub callsite_str: String,
    pub span_name: String,
    pub total_time: SyncHistogram<u64>,
    pub active_time: SyncHistogram<u64>,
}

/// Timings by callsite.
type Timings = HashMap<Identifier, CallsiteTiming>;

/// Callsite parents.
/// Separate from [Timings] to avoid locking issues caused by [SyncHistogram].refresh.
type Parents = HashMap<Identifier, Option<Identifier>>;

/// Thread-local information collected for a callsite.
struct LocalCallsiteTiming {
    total_time: Recorder<u64>,
    active_time: Recorder<u64>,
}

/// Information about a span stored in the registry.
#[derive(Debug)]
struct SpanTiming {
    created_at: Instant,
    entered_at: Instant,
    acc_active_time: u64,
    parent_callsite: Option<Identifier>,
}

/// Provides access a [Timings] containing the latencies collected for different span callsites.
#[derive(Clone)]
pub struct Latencies {
    timings: Arc<RwLock<Timings>>,
    parents: Arc<RwLock<Parents>>,
}

//=================
// Thread-locals

thread_local! {
    static LOCAL_PARENT_INFO: RefCell<Parents> = RefCell::new(HashMap::new());
}

thread_local! {
    static LOCAL_CALLSITE_INFO: RefCell<HashMap<Identifier, LocalCallsiteTiming>> = RefCell::new(HashMap::new());
}

//=================
// impls

impl Latencies {
    pub fn new() -> Latencies {
        let timings = RwLock::new(HashMap::new());
        let parents = RwLock::new(HashMap::new());
        Latencies {
            timings: Arc::new(timings),
            parents: Arc::new(parents),
        }
    }

    fn refresh(&self) {
        for (_, v) in self.timings.write().unwrap().iter_mut() {
            v.total_time.refresh();
            v.active_time.refresh();
        }
    }

    pub fn with(
        &self,
        f: impl FnOnce(&HashMap<Identifier, CallsiteTiming>, &HashMap<Identifier, Option<Identifier>>),
    ) {
        f(
            self.timings.read().unwrap().deref(),
            self.parents.read().unwrap().deref(),
        );
    }

    pub fn print_mean_timings(&self) {
        self.with(|timings, parents| {
            println!("\nMean timing values by span:");

            for (callsite, v) in timings.iter() {
                let mean_total_time = v.total_time.mean();
                let mean_active_time = v.active_time.mean();
                let total_time_count = v.total_time.len();
                let active_time_count = v.active_time.len();
                let parent = parents.get(callsite).unwrap();
                println!(
                    "  callsite={:?}, parent={:?}, callsite_str={}, span_name={}, mean_total_time={}μs, total_time_count={}, mean_active_time={}μs, active_time_count={}",
                    callsite, parent, v.callsite_str, v.span_name, mean_total_time, total_time_count, mean_active_time,active_time_count
                );
            }
        });
    }

    fn update_parent_info(&self, callsite: &Identifier, parent: &Option<Identifier>) {
        log::debug!(
            "entered `update_parent_info`for callsite id {:?} on thread {:?}",
            callsite,
            thread::current().id(),
        );
        LOCAL_PARENT_INFO.with(|parents_cell| {
            let mut parents = parents_cell.borrow_mut();
            if parents.contains_key(callsite) {
                // Both local and global parents info are good for this callsite.
                return;
            }

            // Update local parents
            {
                parents.insert(callsite.clone(), parent.clone());
            }

            // Update global parents
            {
                log::debug!(
                    "`update_parent_info`getting read lock for callsite id {:?} on thread {:?}",
                    callsite,
                    thread::current().id()
                );
                let parents = self.parents.as_ref().read().unwrap();
                if !parents.contains_key(callsite) {
                    drop(parents); // need to get write lock below;
                    log::debug!(
                        "`update_parent_info`getting write lock for callsite id {:?} on thread {:?}",
                        callsite,
                        thread::current().id()
                    );
                    let mut parents = self.parents.as_ref().write().unwrap();
                    log::debug!(
                        "`update_parent_info`got write lock for callsite id {:?} on thread {:?}",
                        callsite,
                        thread::current().id()
                    );
                    // Situation may have changed while waiting for write lock
                    if parents.contains_key(callsite) {
                        return;
                    }
                    parents.insert(callsite.clone(), parent.clone());
                }
            }
        });
    }

    fn with_local_callsite_info(
        &self,
        callsite: &Identifier,
        f: impl Fn(&mut LocalCallsiteTiming) -> (),
    ) {
        LOCAL_CALLSITE_INFO.with(|local_info| {
            let mut callsite_recorders = local_info.borrow_mut();
            let mut local_info = callsite_recorders
                .entry(callsite.clone())
                .or_insert_with(|| {
                    log::debug!(
                        "***** thread-loacal CallsiteRecorder created for callsite={:?} on thread={:?}",
                        callsite,
                        thread::current().id()
                    );

                    let callsite_timings = self.timings.read().unwrap();
                    let callsite_timing = callsite_timings.get(&callsite).unwrap();

                    LocalCallsiteTiming {
                        total_time: callsite_timing.total_time.recorder(),
                        active_time: callsite_timing.active_time.recorder(),
                    }
                });

            f(&mut local_info);
            log::debug!(
                "***** exiting with_local_callsite_info for callsite={:?} on thread={:?}",
                callsite,
                thread::current().id()
            );
});
    }
}

impl<S> Layer<S> for Latencies
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        log::debug!("`register_callsite` entered");
        if !meta.is_span() {
            return Interest::never();
        }

        let meta_name = meta.name();
        let callsite = meta.callsite();
        let callsite_str = format!("{}-{}", meta.module_path().unwrap(), meta.line().unwrap());
        let interest = Interest::always();

        let mut timings = self.timings.write().unwrap();

        let mut hist = Histogram::<u64>::new_with_bounds(1, 60 * 1000, 1).unwrap();
        hist.auto(true);
        let hist2 = hist.clone();
        let hist: SyncHistogram<u64> = hist.into();
        let hist2: SyncHistogram<u64> = hist2.into();

        timings.insert(
            callsite.clone(),
            CallsiteTiming {
                callsite_str: callsite_str.to_owned(),
                span_name: meta_name.to_owned(),
                total_time: hist,
                active_time: hist2,
            },
        );

        log::debug!(
            "`register_callsite` executed with id={:?}, meta_name={}",
            callsite,
            meta_name
        );

        interest
    }

    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        log::debug!("entered `on_new_span`");
        let span = ctx.span(id).unwrap();
        let parent_span = span.parent();
        let parent_callsite = parent_span.map(|span_ref| span_ref.metadata().callsite());

        span.extensions_mut().insert(SpanTiming {
            created_at: Instant::now(),
            entered_at: Instant::now(),
            acc_active_time: 0,
            parent_callsite,
        });
        log::debug!("`on_new_span` executed with id={:?}", id);
    }

    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        log::debug!("entered `on_enter` wth span Id {:?}", id);
        let span = ctx.span(id).unwrap();
        let mut ext = span.extensions_mut();
        let span_timing = ext.get_mut::<SpanTiming>().unwrap();
        span_timing.entered_at = Instant::now();
        log::debug!("`on_enter` executed with id={:?}", id);
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        log::debug!("entered `on_exit` wth span Id {:?}", id);
        let span = ctx.span(id).unwrap();
        let mut ext = span.extensions_mut();
        let span_timing = ext.get_mut::<SpanTiming>().unwrap();
        span_timing.acc_active_time += (Instant::now() - span_timing.entered_at).as_micros() as u64;
        log::debug!("`on_exit` executed for span id {:?}", id);
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        log::debug!("entered `on_close` wth span Id {:?}", id);

        let span = ctx.span(&id).unwrap();
        let callsite = span.metadata().callsite();
        let ext = span.extensions();
        let span_timing = ext.get::<SpanTiming>().unwrap();

        self.with_local_callsite_info(&callsite, |r| {
            r.total_time
                .record((Instant::now() - span_timing.created_at).as_micros() as u64)
                .unwrap();
            r.active_time.record(span_timing.acc_active_time).unwrap();
        });

        log::debug!(
            "`on_close` completed call to with_local_callsite_info for span id {:?}",
            id
        );

        self.update_parent_info(&callsite, &span_timing.parent_callsite);

        log::debug!("`on_close` executed for span id {:?}", id);
    }
}

//=================
// functions

/// Measures latencies of spans in `f`.
/// May only be called once per process and will panic if called more than once.
pub fn measure_latencies(f: impl FnOnce() -> () + Send + 'static) -> Latencies {
    let latencies = Latencies::new();
    Registry::default().with(latencies.clone()).init();
    thread::spawn(f).join().unwrap();
    latencies.refresh();
    latencies
}

/// Measures latencies of spans in async function `f` running on the [tokio] runtime.
/// May only be called once per process and will panic if called more than once.
pub fn measure_latencies_tokio<F>(f: impl FnOnce() -> F + Send + 'static) -> Latencies
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

//=================
// Examples

mod example {
    use super::*;

    #[instrument(level = "trace")]
    pub async fn f() {
        let mut foo: u64 = 1;

        for _ in 0..4 {
            log::debug!("Before my_great_span");

            async {
                thread::sleep(Duration::from_millis(3));
                tokio::time::sleep(Duration::from_millis(100)).await;
                foo += 1;
                info!(yak_shaved = true, yak_count = 2, "hi from inside my span");
                log::debug!("Before my_other_span");
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
}

fn main() {
    use example::f;

    set_var("RUST_LOG", "debug");

    let latencies = measure_latencies_tokio(|| async {
        // Set env_logger only if `tracing_subsriber` hasn't pulled in `tracing_log` and already set a logger.
        // Otherwise, setting a second logger would panic.
        _ = env_logger::try_init();

        let h1 = tokio::spawn(f());
        let h2 = tokio::spawn(f());
        _ = h1.await;
        _ = h2.await;
    });

    latencies.print_mean_timings();

    latencies.with(|timings, parents| {
    println!("\nMedian timings by span:");
    for (callsite, v) in timings.iter() {
        let median_total_time = v.total_time.value_at_quantile(0.5);
        let median_active_time = v.active_time.value_at_quantile(0.5);
        let total_time_count = v.total_time.len();
        let active_time_count = v.active_time.len();
        let parent = parents.get(callsite).unwrap();
        println!(
            "  callsite_id={:?}, parent_callsite={:?}, callsite_str={}, span_name={}, median_total_time={}μs, total_time_count={}, median_active_time={}μs, active_time_count={}",
            callsite, parent, v.callsite_str, v.span_name, median_total_time, total_time_count, median_active_time,active_time_count
        );
    }});
}
