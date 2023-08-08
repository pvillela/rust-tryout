//! From https://stackoverflow.com/a/74711569/445619.

use std::time::{Duration, Instant};
use tracing::span::{Attributes, Id};
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

struct Timing {
    started_at: Instant,
}

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        println!("`on_new_span` entered");
        let span = ctx.span(id).unwrap();

        span.extensions_mut().insert(Timing {
            started_at: Instant::now(),
        });
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        println!("`on_close` entered");
        let span = ctx.span(&id).unwrap();

        let started_at = span.extensions().get::<Timing>().unwrap().started_at;

        println!(
            "span {} took {}",
            span.metadata().name(),
            (Instant::now() - started_at).as_micros(),
        );
    }
}

#[tracing::instrument]
fn test(n: u64) {
    std::thread::sleep(Duration::from_millis(n));
}

fn main() {
    tracing_subscriber::registry::Registry::default()
        .with(CustomLayer)
        .init();

    test(10);
    test(20);
    test(30);
}
