//! Based on `tracing_counter_by_span_name_naive` and `tracing_timing_original`.
//! Naive because it does not use [tracing_subscriber::Registry] and instead uses a naive storage
//! approach based on [std::sync::RwLock].
//!
//! This captures both total and sync timings:
//! - total timings include suspend time and are based on span creation and closing;
//! - active timings exclude suspend time and are based on span entry and exit.

use hdrhistogram::{sync::SyncHistogram, Histogram};
use std::{
    collections::HashMap,
    sync::RwLock,
    thread,
    time::{Duration, Instant},
};
use tracing::{
    callsite::Identifier,
    info, span,
    subscriber::{Interest, Subscriber},
    warn, Id, Level, Metadata,
};
use tracing_core::span::Attributes;
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    util::SubscriberInitExt,
    Layer,
};

#[derive(Debug)]
struct CallsiteTiming {
    meta_name: String,
    total_time: SyncHistogram<u64>,
    active_time: SyncHistogram<u64>,
}

struct SpanStartTime {
    callsite: Identifier,
    created_at: Instant,
    entered_at: Instant,
}

/// Collects counts emitted by application spans and events.
pub struct Timings {
    callsite_timing: RwLock<HashMap<Identifier, CallsiteTiming>>,
    span_start_times: RwLock<HashMap<Id, SpanStartTime>>,
}

impl Timings {
    pub fn new_leaked_layer() -> &'static Self {
        let timing_by_span = RwLock::new(HashMap::new());
        let span_start_times = RwLock::new(HashMap::new());
        let timings = Self {
            callsite_timing: timing_by_span,
            span_start_times,
        };
        Box::leak(Box::new(timings))
    }

    pub fn print_mean_timing(&self) {
        for (_, v) in self.callsite_timing.read().unwrap().iter() {
            let mean_total_time = v.total_time.mean();
            let mean_active_time = v.active_time.mean();
            println!(
                "  name={}, mean_total_time={}μs, mean_active_time={}μs",
                v.meta_name, mean_total_time, mean_active_time
            );
        }
    }
}

impl<S: Subscriber> Layer<S> for &'static Timings {
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        //println!("`register_callsite` entered");

        let meta_name = meta.name();
        let callsite = meta.callsite();
        let interest = Interest::always();

        let mut map = self.callsite_timing.write().unwrap();

        let mut hist = Histogram::<u64>::new_with_bounds(1, 60 * 1000, 1).unwrap();
        hist.auto(true);
        let hist2 = hist.clone();
        let hist: SyncHistogram<u64> = hist.into();
        let hist2: SyncHistogram<u64> = hist2.into();

        map.insert(
            callsite.clone(),
            CallsiteTiming {
                meta_name: meta_name.to_owned(),
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

    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, _ctx: Context<'_, S>) {
        //println!("`new_span` entered");
        let callsite = attrs.metadata().callsite();

        let mut start_times = self.span_start_times.write().unwrap();
        start_times.insert(
            id.clone(),
            SpanStartTime {
                callsite: callsite.clone(),
                created_at: Instant::now(),
                entered_at: Instant::now(),
            },
        );

        //println!("`new_span` executed with id={:?}", id);
    }

    fn on_enter(&self, id: &Id, _ctx: Context<'_, S>) {
        //println!("entered `enter` wth span Id {:?}", id);
        let mut start_times = self.span_start_times.write().unwrap();
        let start_time = &mut start_times.get_mut(id).unwrap().entered_at;
        *start_time = Instant::now();
        //println!("`enter` executed with id={:?}", id);
    }

    fn on_exit(&self, id: &Id, _ctx: Context<'_, S>) {
        //println!("entered `exit` wth span Id {:?}", id);
        let start_times = self.span_start_times.write().unwrap();
        let SpanStartTime {
            callsite,
            created_at: _,
            entered_at,
        } = start_times.get(id).unwrap();

        let mut timings = self.callsite_timing.write().unwrap();
        let timing = timings.get_mut(&callsite).unwrap();
        timing
            .active_time
            .record((Instant::now() - *entered_at).as_micros() as u64)
            .unwrap();
        //println!("`try_close` executed for span id {:?}", id);
    }

    fn on_close(&self, id: Id, _ctx: Context<'_, S>) {
        //println!("entered `try_close` wth span Id {:?}", id);
        let mut start_times = self.span_start_times.write().unwrap();
        let SpanStartTime {
            callsite,
            created_at,
            entered_at: _,
        } = start_times.remove(&id).unwrap();

        let mut timings = self.callsite_timing.write().unwrap();
        let timing = timings.get_mut(&callsite).unwrap();
        timing
            .total_time
            .record((Instant::now() - created_at).as_micros() as u64)
            .unwrap();
        //println!("`try_close` executed for span id {:?}", id);
    }
}

fn main() {
    let timing_layer = Timings::new_leaked_layer();
    tracing_subscriber::registry::Registry::default()
        .with(timing_layer)
        .init();

    let mut foo: u64 = 1;

    for _ in 0..2 {
        println!("Before top-level span! macro");
        span!(Level::TRACE, "my_great_span", foo_count = &foo).in_scope(|| {
            thread::sleep(Duration::from_millis(100));
            foo += 1;
            info!(yak_shaved = true, yak_count = 2, "hi from inside my span");
            println!("Before lower-level span! macro");
            span!(
                Level::TRACE,
                "my other span",
                foo_count = &foo,
                baz_count = 5
            )
            .in_scope(|| {
                thread::sleep(Duration::from_millis(25));
                warn!(yak_shaved = false, yak_count = -1, "failed to shave yak");
            });
        });
    }

    timing_layer.print_mean_timing();
}
