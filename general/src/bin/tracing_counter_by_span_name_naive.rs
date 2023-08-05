//! Modification of module `tracing_counter_refactored` to capture separate counts by span.
//! Naive because it does not use [tracing_subscriber::Registry] and instead uses a naive storage
//! approach based on [std::sync::RwLock].

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, AtomicUsize, Ordering},
        Arc, RwLock, RwLockReadGuard,
    },
};
use tracing::{
    field::{Field, Visit},
    info, span,
    subscriber::{Interest, Subscriber},
    warn, Event, Id, Level, Metadata,
};
use tracing_core::callsite::Identifier;

/// Keeps track of counts by field name.
type FieldCounts = RwLock<HashMap<String, AtomicI64>>;

/// Keeps track of counts by callsite.
type CountsBySpan = RwLock<HashMap<Identifier, SpanCounts>>;

#[derive(Debug)]
struct SpanCounts {
    meta_name: String,
    field_counts: FieldCounts,
}

/// Collects counts emitted by application spans and events.
pub struct CountsCollector {
    next_id: AtomicUsize,
    counts_by_span: Arc<CountsBySpan>,
}

/// Provides external visibility to counts collected by [CountsCollector].
pub struct CountsHandle(Arc<CountsBySpan>);

/// Required for implementation of [CountsCollector] as a [Subscriber] to perform accumulation of counts.
struct FieldCountsVisitor<'a> {
    callsite: Identifier,
    counts_by_span: RwLockReadGuard<'a, HashMap<Identifier, SpanCounts>>, //RwLockReadGuard<'a, HashMap<Identifier, FieldCounts>>,
}

impl<'a> Visit for FieldCountsVisitor<'a> {
    fn record_i64(&mut self, field: &Field, value: i64) {
        if let Some(counter) = self
            .counts_by_span
            .get(&self.callsite)
            .unwrap()
            .field_counts
            .read()
            .unwrap()
            .get(field.name())
        {
            counter.fetch_add(value, Ordering::Release);
        };
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if let Some(counter) = self
            .counts_by_span
            .get(&self.callsite)
            .unwrap()
            .field_counts
            .read()
            .unwrap()
            .get(field.name())
        {
            counter.fetch_add(value as i64, Ordering::Release);
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
            counts_by_span: counts,
        };
        (collector, handle)
    }

    fn visitor(&self, callsite: Identifier) -> FieldCountsVisitor {
        let x = self.counts_by_span.read().unwrap();
        FieldCountsVisitor {
            callsite,
            counts_by_span: x,
        }
    }
}

impl CountsHandle {
    fn print_counts(&self) {
        for (_, v) in self.0.read().unwrap().iter() {
            println!("meta_name={}", v.meta_name);
            for (k1, v1) in v.field_counts.read().unwrap().iter() {
                println!("  field={}, value={}", k1, v1.load(Ordering::Acquire));
            }
            // println!("{:?}: {:?}", k, v);
        }
    }
}

impl Subscriber for CountsCollector {
    fn register_callsite(&self, meta: &Metadata<'_>) -> Interest {
        println!("`register_callsite` entered");

        let meta_name = meta.name();
        let callsite = meta.callsite();

        let mut interest = Interest::never();
        for key in meta.fields() {
            let callsite = callsite.clone();
            let name = key.name();
            if name.contains("count") {
                let mut x1 = self.counts_by_span.write().unwrap();
                let x2 = x1.entry(callsite.clone()).or_insert_with(|| SpanCounts {
                    // callsite,
                    meta_name: meta_name.to_owned(),
                    field_counts: RwLock::new(HashMap::new()),
                });
                x2.field_counts
                    .write()
                    .unwrap()
                    .entry(name.to_owned())
                    .or_insert_with(|| AtomicI64::new(0));
                interest = Interest::always();
            }
        }

        println!(
            "`register_callsite` executed with id={:?}, meta_name={}",
            callsite, meta_name
        );

        interest
    }

    fn new_span(&self, new_span: &span::Attributes<'_>) -> Id {
        println!("`new_span` entered");
        let callsite = new_span.metadata().callsite();
        new_span.record(&mut self.visitor(callsite));
        let id = self.next_id.fetch_add(1, Ordering::AcqRel);
        println!("`new_span` executed with id={}", id);
        Id::from_u64(id as u64)
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {
        // unimplemented
    }

    fn record(&self, _: &Id, _values: &span::Record<'_>) {
        println!("`record` entered");
    }

    fn event(&self, event: &Event<'_>) {
        let callsite = event.metadata().callsite();
        event.record(&mut self.visitor(callsite))
    }

    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        // metadata.fields().iter().any(|f| f.name().contains("count"))
        true
    }

    fn enter(&self, _span: &Id) {
        println!("`enter` entered");
    }

    fn exit(&self, _span: &Id) {}
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
