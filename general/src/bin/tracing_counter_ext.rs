//! Updated from https://github.com/tokio-rs/tracing/blob/master/examples/examples/counters.rs,
//! which contains references to an older version of the `tracing` crate.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock, RwLockReadGuard,
    },
};
use tracing::{
    field::{Field, Visit},
    info, span,
    subscriber::{Interest, Subscriber},
    warn, Event, Id, Level, Metadata,
};
use tracing_core::span::Current;

/// Keeps track of counts by field name.
type Counts = Arc<RwLock<HashMap<String, AtomicUsize>>>;

/// Collects counts emitted by application spans and events.
pub struct CountsCollector {
    next_id: AtomicUsize,
    counts: Counts,
}

/// Provides external visibility to counts collected by [CountsCollector].
pub struct CountsHandle(Counts);

/// Required for implementation of [CountsCollector] as a [Subscriber] to perform accumulation of counts.
struct CountsVisitor<'a> {
    counts: RwLockReadGuard<'a, HashMap<String, AtomicUsize>>,
}

impl<'a> Visit for CountsVisitor<'a> {
    fn record_i64(&mut self, field: &Field, value: i64) {
        if let Some(counter) = self.counts.get(field.name()) {
            if value > 0 {
                counter.fetch_add(value as usize, Ordering::Release);
            } else {
                counter.fetch_sub(-value as usize, Ordering::Release);
            }
        };
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if let Some(counter) = self.counts.get(field.name()) {
            counter.fetch_add(value as usize, Ordering::Release);
        };
    }

    fn record_bool(&mut self, _: &Field, _: bool) {}
    fn record_str(&mut self, _: &Field, _: &str) {}
    fn record_debug(&mut self, _: &Field, _: &dyn fmt::Debug) {}
}

impl CountsCollector {
    fn new() -> (Self, CountsHandle) {
        let counts = Arc::new(RwLock::new(HashMap::new()));
        let handle = CountsHandle(counts.clone());
        let collector = CountsCollector {
            next_id: AtomicUsize::new(1),
            counts,
        };
        (collector, handle)
    }

    fn visitor(&self) -> CountsVisitor<'_> {
        CountsVisitor {
            counts: self.counts.read().unwrap(),
        }
    }
}

impl CountsHandle {
    fn print_counts(&self) {
        for (k, v) in self.0.read().unwrap().iter() {
            println!("{}: {}", k, v.load(Ordering::Acquire));
        }
    }
}

impl Subscriber for CountsCollector {
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        let mut interest = Interest::never();
        for key in meta.fields() {
            let name = key.name();
            if name.contains("count") {
                self.counts
                    .write()
                    .unwrap()
                    .entry(name.to_owned())
                    .or_insert_with(|| AtomicUsize::new(0));
                interest = Interest::always();
            }
        }
        interest
    }

    fn new_span(&self, new_span: &span::Attributes<'_>) -> Id {
        new_span.record(&mut self.visitor());
        let id = self.next_id.fetch_add(1, Ordering::AcqRel);
        println!("`new_span` executed with id={}", id);
        Id::from_u64(id as u64)
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {
        // unimplemented
    }

    fn record(&self, _: &Id, values: &span::Record<'_>) {
        values.record(&mut self.visitor())
    }

    fn event(&self, event: &Event<'_>) {
        event.record(&mut self.visitor())
    }

    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.fields().iter().any(|f| f.name().contains("count"))
    }

    fn enter(&self, _span: &Id) {
        println!("`enter` executed");
    }
    fn exit(&self, _span: &Id) {}
    fn current_span(&self) -> Current {
        Current::none()
    }
}

fn main() {
    let (collector, handle) = CountsCollector::new();

    tracing::subscriber::set_global_default(collector).unwrap();

    let mut foo: u64 = 1;

    for _ in 0..2 {
        println!("Before top-level span! macro");
        span!(Level::TRACE, "my_great_span", foo_count = &foo).in_scope(|| {
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
                warn!(yak_shaved = false, yak_count = -1, "failed to shave yak");
            });
        });
    }

    handle.print_counts();
}
