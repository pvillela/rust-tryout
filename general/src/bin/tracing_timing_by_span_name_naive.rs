//! Based on `tracing_counter_by_span_name_naive` and `tracing_timing_original`.
//! Naive because it does not use [tracing_subscriber::Registry] and instead uses a naive storage
//! approach based on [std::sync::RwLock].
//!
//! This captures both total and sync timings:
//! - total timings include suspend time and are based on span creation and closing;
//! - active timings exclude suspend time and are based on span entry and exit.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        RwLock,
    },
    thread,
    time::{Duration, Instant},
};
use tracing::{
    callsite::Identifier,
    dispatcher::set_global_default,
    info, span,
    subscriber::{Interest, Subscriber},
    warn, Dispatch, Event, Id, Level, Metadata,
};

/// Keeps track of counts by callsite.
type TimingBySpan = RwLock<HashMap<Identifier, CallsiteTiming>>;

#[derive(Debug)]
struct CallsiteTiming {
    meta_name: String,
    acc_total_time: AtomicU64,
    acc_active_time: AtomicU64,
    count: AtomicU64,
}

struct SpanStartTime {
    callsite: Identifier,
    created_at: Instant,
    entered_at: Instant,
}

/// Collects counts emitted by application spans and events.
pub struct TimingCollector {
    next_id: AtomicUsize,
    timing_by_span: TimingBySpan,
    span_start_times: RwLock<HashMap<Id, SpanStartTime>>,
}

impl TimingCollector {
    pub fn new_dispatch() -> Dispatch {
        let timing_by_span = RwLock::new(HashMap::new());
        let span_start_times = RwLock::new(HashMap::new());
        let collector = TimingCollector {
            next_id: AtomicUsize::new(1),
            timing_by_span,
            span_start_times,
        };
        Dispatch::new(collector)
    }

    pub fn print_timing(&self) {
        for (_, v) in self.timing_by_span.read().unwrap().iter() {
            let acc_total_time = v.acc_total_time.load(Ordering::Acquire);
            let acc_active_time = v.acc_active_time.load(Ordering::Acquire);
            let count = v.count.load(Ordering::Acquire);
            let mean_total_time = if count > 0 { acc_total_time / count } else { 0 };
            let mean_active_time = if count > 0 {
                acc_active_time / count
            } else {
                0
            };
            println!(
                "  name={}, acc_total_time={}μs, acc_active_time={}μs, count={}, mean_total_time={}μs, mean_active_time={}μs",
                v.meta_name, acc_total_time, acc_active_time, count, mean_total_time, mean_active_time
            );
        }
    }
}

impl Subscriber for TimingCollector {
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        //println!("`register_callsite` entered");

        let meta_name = meta.name();
        let callsite = meta.callsite();
        let interest = Interest::always();

        let mut map = self.timing_by_span.write().unwrap();
        map.insert(
            callsite.clone(),
            CallsiteTiming {
                meta_name: meta_name.to_owned(),
                acc_total_time: AtomicU64::new(0),
                acc_active_time: AtomicU64::new(0),
                count: AtomicU64::new(0),
            },
        );

        //println!(
        //     "`register_callsite` executed with id={:?}, meta_name={}",
        //     callsite, meta_name
        // );

        interest
    }

    fn new_span(&self, new_span: &span::Attributes<'_>) -> Id {
        //println!("`new_span` entered");
        let callsite = new_span.metadata().callsite();
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let id = Id::from_u64(id as u64);

        let mut start_times = self.span_start_times.write().unwrap();
        start_times.insert(
            id.clone(),
            SpanStartTime {
                callsite: callsite.clone(),
                created_at: Instant::now(),
                entered_at: Instant::now(),
            },
        );

        let timings = self.timing_by_span.read().unwrap();
        let timing = timings.get(&callsite).unwrap();
        timing.count.fetch_add(1, Ordering::AcqRel);

        //println!("`new_span` executed with id={:?}", id);
        id
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {
        // unimplemented
    }

    fn record(&self, _: &Id, _values: &span::Record<'_>) {
        //println!("`record` entered");
    }

    fn event(&self, event: &Event<'_>) {
        let _name = event.metadata().name();
        //println!("`event` executed for name: {}", name)
    }

    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        // metadata.fields().iter().any(|f| f.name().contains("count"))
        true
    }

    fn enter(&self, id: &Id) {
        //println!("entered `enter` wth span Id {:?}", id);
        let mut start_times = self.span_start_times.write().unwrap();
        let start_time = &mut start_times.get_mut(id).unwrap().entered_at;
        *start_time = Instant::now();
        //println!("`enter` executed with id={:?}", id);
    }

    fn exit(&self, id: &Id) {
        //println!("entered `exit` wth span Id {:?}", id);
        let start_times = self.span_start_times.write().unwrap();
        let SpanStartTime {
            callsite,
            created_at: _,
            entered_at,
        } = start_times.get(id).unwrap();

        let timings = self.timing_by_span.read().unwrap();
        let timing = timings.get(&callsite).unwrap();
        timing.acc_active_time.fetch_add(
            (Instant::now() - *entered_at).as_micros() as u64,
            Ordering::AcqRel,
        );
        //println!("`try_close` executed for span id {:?}", id);
    }

    fn try_close(&self, id: Id) -> bool {
        //println!("entered `try_close` wth span Id {:?}", id);
        let mut start_times = self.span_start_times.write().unwrap();
        let SpanStartTime {
            callsite,
            created_at,
            entered_at: _,
        } = start_times.remove(&id).unwrap();

        let timings = self.timing_by_span.read().unwrap();
        let timing = timings.get(&callsite).unwrap();
        timing.acc_total_time.fetch_add(
            (Instant::now() - created_at).as_micros() as u64,
            Ordering::AcqRel,
        );
        //println!("`try_close` executed for span id {:?}", id);
        true
    }
}

fn main() {
    let dispatch = TimingCollector::new_dispatch();
    set_global_default(dispatch.clone()).unwrap();

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

    let collector: &TimingCollector = dispatch.downcast_ref().unwrap();
    collector.print_timing();
}
