//! Based on `tracing_counter_by_span_name_naive` and `tracing_timing_original`.
//! Naive because it does not use [tracing_subscriber::Registry] and instead uses a naive storage
//! approach based on [std::sync::RwLock].

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};
use tracing::{
    info, span,
    subscriber::{Interest, Subscriber},
    warn, Event, Id, Level, Metadata,
};
use tracing_core::callsite::Identifier;

/// Keeps track of counts by callsite.
type TimingBySpan = RwLock<HashMap<Identifier, SpanCallsiteTiming>>;

#[derive(Debug)]
struct SpanCallsiteTiming {
    meta_name: String,
    acc_time: AtomicU64,
    count: AtomicU64,
}

struct SpanEntryTime {
    callsite: Identifier,
    started_at: Instant,
}

/// Collects counts emitted by application spans and events.
pub struct TimingCollector {
    next_id: AtomicUsize,
    timing_by_span: Arc<TimingBySpan>,
    span_entry_times: RwLock<HashMap<Id, SpanEntryTime>>,
}

/// Provides external visibility to counts collected by [CountsCollector].
pub struct TimingHandle(Arc<TimingBySpan>);

impl TimingCollector {
    fn new() -> (Self, TimingHandle) {
        let timing_by_span = Arc::new(RwLock::new(HashMap::new()));
        let span_entry_times = RwLock::new(HashMap::new());
        let handle = TimingHandle(timing_by_span.clone());
        let collector = TimingCollector {
            next_id: AtomicUsize::new(1),
            timing_by_span,
            span_entry_times,
        };
        (collector, handle)
    }
}

impl TimingHandle {
    fn print_timing(&self) {
        for (_, v) in self.0.read().unwrap().iter() {
            let acc_time = v.acc_time.load(Ordering::Acquire);
            let count = v.count.load(Ordering::Acquire);
            let mean = if count > 0 { acc_time / count } else { 0 };
            println!(
                "  name={}, acc_time={}μs, count={}, mean={}μs",
                v.meta_name, acc_time, count, mean
            );
        }
    }
}

impl Subscriber for TimingCollector {
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        println!("`register_callsite` entered");

        let meta_name = meta.name();
        let callsite = meta.callsite();
        let interest = Interest::always();

        let mut map = self.timing_by_span.write().unwrap();
        map.entry(callsite.clone())
            .or_insert_with(|| SpanCallsiteTiming {
                meta_name: meta_name.to_owned(),
                acc_time: AtomicU64::new(0),
                count: AtomicU64::new(0),
            });

        println!(
            "`register_callsite` executed with id={:?}, meta_name={}",
            callsite, meta_name
        );

        interest
    }

    fn new_span(&self, new_span: &span::Attributes<'_>) -> Id {
        println!("`new_span` entered");
        let callsite = new_span.metadata().callsite();
        let id = self.next_id.fetch_add(1, Ordering::AcqRel);
        let id = Id::from_u64(id as u64);
        let mut map = self.span_entry_times.write().unwrap();
        map.insert(
            id.clone(),
            SpanEntryTime {
                callsite,
                started_at: Instant::now(),
            },
        );
        println!("`new_span` executed with id={:?}", id);
        id
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {
        // unimplemented
    }

    fn record(&self, _: &Id, _values: &span::Record<'_>) {
        println!("`record` entered");
    }

    fn event(&self, event: &Event<'_>) {
        let name = event.metadata().name();
        println!("`event` executed for name: {}", name)
    }

    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        // metadata.fields().iter().any(|f| f.name().contains("count"))
        true
    }

    fn enter(&self, _span: &Id) {
        println!("`enter` entered");
    }

    fn exit(&self, span: &Id) {
        let mut map1 = self.span_entry_times.write().unwrap();
        let SpanEntryTime {
            callsite,
            started_at,
        } = map1.remove(span).unwrap();
        let map2 = self.timing_by_span.read().unwrap();
        let timing = map2.get(&callsite).unwrap();
        timing.acc_time.fetch_add(
            (Instant::now() - started_at).as_micros() as u64,
            Ordering::AcqRel,
        );
        timing.count.fetch_add(1, Ordering::AcqRel);
        println!("`exit` executed for span id {:?}", span);
    }
}

fn main() {
    let (collector, handle) = TimingCollector::new();

    tracing::subscriber::set_global_default(collector).unwrap();

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

    handle.print_timing();
}
