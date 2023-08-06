//! Based on `tracing_counter_by_span_name_naive` and `tracing_timing_original`.
//! Naive because it does not use [tracing_subscriber::Registry] and instead uses a naive storage
//! approach based on [std::sync::RwLock].
//!
//! This captures both total and sync timings:
//! - total timings include suspend time and are based on span creation and closing;
//! - sync timings exclude suspend time and are based on span entry and exit.

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
    warn, Event, Id, Metadata,
};
use tracing_core::{callsite::Identifier, Level};

/// Keeps track of counts by callsite.
type TimingBySpan = RwLock<HashMap<Identifier, SpanCallsiteTiming>>;

#[derive(Debug)]
struct SpanCallsiteTiming {
    meta_name: String,
    acc_total_time: AtomicU64,
    acc_active_time: AtomicU64,
    count: AtomicU64,
}

struct SpanStartTime {
    callsite: Identifier,
    created_at: RwLock<Instant>,
    entered_at: RwLock<Instant>,
}

/// Collects counts emitted by application spans and events.
pub struct TimingCollector {
    next_id: AtomicUsize,
    timing_by_span: Arc<TimingBySpan>,
    span_start_times: RwLock<HashMap<Id, SpanStartTime>>,
}

/// Provides external visibility to counts collected by [CountsCollector].
pub struct TimingHandle(Arc<TimingBySpan>);

impl TimingCollector {
    pub fn new() -> (Self, TimingHandle) {
        let timing_by_span = Arc::new(RwLock::new(HashMap::new()));
        let span_start_times = RwLock::new(HashMap::new());
        let handle = TimingHandle(timing_by_span.clone());
        let collector = TimingCollector {
            next_id: AtomicUsize::new(1),
            timing_by_span,
            span_start_times,
        };
        (collector, handle)
    }
}

impl TimingHandle {
    pub fn print_timing(&self) {
        for (_, v) in self.0.read().unwrap().iter() {
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
                "  name={}, acc_time={}μs, acc_sync_time={}μs, count={}, mean_time={}μs, mean_sync_time={}μs",
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
            SpanCallsiteTiming {
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
                created_at: RwLock::new(Instant::now()),
                entered_at: RwLock::new(Instant::now()),
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
        let start_times = self.span_start_times.write().unwrap();
        let mut start_time = start_times.get(id).unwrap().entered_at.write().unwrap();
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
            (Instant::now() - *entered_at.read().unwrap()).as_micros() as u64,
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
            (Instant::now() - *created_at.read().unwrap()).as_micros() as u64,
            Ordering::AcqRel,
        );
        //println!("`try_close` executed for span id {:?}", id);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};
    use tracing::{info, span, warn, Level};

    #[test]
    fn test_all() {
        let (collector, handle) = TimingCollector::new();

        tracing::subscriber::set_global_default(collector).unwrap();

        let mut foo: u64 = 1;

        for _ in 0..2 {
            //println!("Before top-level span! macro");
            span!(Level::TRACE, "my_great_span", foo_count = &foo).in_scope(|| {
                thread::sleep(Duration::from_millis(100));
                foo += 1;
                info!(yak_shaved = true, yak_count = 2, "hi from inside my span");
                //println!("Before lower-level span! macro");
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
