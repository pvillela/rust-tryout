//! Example of latency measurement for a simple async function.

use std::time::Duration;
use tracing::{dispatcher, instrument, trace, trace_span, Dispatch, Instrument};
use tracing_timing::{Builder, Histogram, TimingSubscriber};

#[instrument(level = "trace")]
async fn f() {
    for _ in 0..1000 {
        async {
            trace_span!("empty").in_scope(|| {
                // Empty span used to show some of the tracing overhead.
                trace!("end_empty");
            });

            // Simulated work
            tokio::time::sleep(Duration::from_micros(6000)).await;

            g().await;
            trace!("end_loop_body");
        }
        .instrument(trace_span!("loop_body"))
        .await
    }
    trace!("end_f");
}

#[instrument(level = "trace")]
async fn g() {
    // Simulated work
    tokio::time::sleep(Duration::from_micros(4000)).await;
    trace!("end_g");
}

fn main() {
    let s = Builder::default().build(|| Histogram::new_with_bounds(10_000, 1_000_000, 3).unwrap());

    let d = Dispatch::new(s);
    let d2 = d.clone();
    std::thread::spawn(move || {
        // let fast = Normal::<f64>::new(100_000.0, 50_000.0).unwrap();
        // let slow = Normal::<f64>::new(500_000.0, 50_000.0).unwrap();
        dispatcher::with_default(&d2, || {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    f().await;
                })
        })
    });
    std::thread::sleep(std::time::Duration::from_secs(5));

    d.downcast_ref::<TimingSubscriber>()
        .unwrap()
        .with_histograms(|hs| {
            for (span_group, hs) in hs {
                for (event_group, h) in hs {
                    // make sure we see the latest samples:
                    h.refresh();
                    // print the median:
                    println!(
                        "{} -> {}: {}ns",
                        span_group,
                        event_group,
                        h.value_at_quantile(0.5)
                    )
                }
            }
        });
}
