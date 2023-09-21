//! Example of latency measurement for a simple async function.

use rand_distr::Normal;
use std::time::Duration;
use tracing::{dispatcher, instrument, trace, trace_span, Dispatch, Instrument};
use tracing_timing::{Builder, Histogram, TimingSubscriber};

#[instrument(level = "trace")]
async fn f() {
    for _ in 0..1000 {
        async {
            trace_span!("empty").in_scope(|| {
                // Empty span used to show some of the tracing overhead.
            });

            // Simulated work
            tokio::time::sleep(Duration::from_micros(6000)).await;

            g().await;
        }
        .instrument(trace_span!("loop_body"))
        .await
    }
}

#[instrument(level = "trace")]
async fn g() {
    // Simulated work
    tokio::time::sleep(Duration::from_micros(4000)).await;
}

fn main() {
    let s = Builder::default().build(|| Histogram::new_with_bounds(10_000, 1_000_000, 3).unwrap());
    let sid = s.downcaster();

    let d = Dispatch::new(s);
    let d2 = d.clone();
    std::thread::spawn(move || {
        use rand::prelude::*;
        let mut rng = thread_rng();
        // let fast = Normal::<f64>::new(100_000.0, 50_000.0).unwrap();
        // let slow = Normal::<f64>::new(500_000.0, 50_000.0).unwrap();
        dispatcher::with_default(&d2, || {
            for _ in 0..100000 {
                let fast = std::time::Duration::from_nanos(100_000 * 10);
                let slow = std::time::Duration::from_nanos(500_000 * 10);
                trace_span!("request").in_scope(|| {
                    std::thread::sleep(fast); // emulate some work
                    trace!("fast");
                    std::thread::sleep(slow); // emulate some more work
                    trace!("slow");
                });
            }
        })
    });
    std::thread::sleep(std::time::Duration::from_secs(15));

    d.downcast_ref::<TimingSubscriber>()
        .unwrap()
        .with_histograms(|hs| {
            for (span_group, hs) in hs {
                for (event_group, h) in hs {
                    // make sure we see the latest samples:
                    h.refresh();
                    // print the median:
                    println!(
                        "{} -> {}: {}ns; count={}",
                        span_group,
                        event_group,
                        h.value_at_quantile(0.5),
                        h.len()
                    )
                }
            }
        });
}
