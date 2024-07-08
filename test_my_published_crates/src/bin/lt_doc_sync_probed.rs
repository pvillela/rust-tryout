use latency_trace::{summary_stats, LatencyTrace};
use std::{thread, time::Duration};
use tracing::{instrument, trace_span};

#[instrument(level = "trace")]
fn f() {
    for _ in 0..1000 {
        trace_span!("loop_body").in_scope(|| {
            // Simulated work
            thread::sleep(Duration::from_micros(1200));

            g();
        });
    }
}

#[instrument(level = "trace")]
fn g() {
    // Simulated work
    thread::sleep(Duration::from_micros(800));
}

fn main() {
    let probed = LatencyTrace::activated_default()
        .unwrap()
        .measure_latencies_probed(f)
        .unwrap();

    // Let the function run for some time before probing latencies.
    thread::sleep(Duration::from_micros(48000));

    let latencies1 = probed.probe_latencies();
    let latencies2 = probed.wait_and_report();

    println!("\nlatencies1 in microseconds");
    for (span_group, stats) in latencies1.map_values(summary_stats) {
        println!("  * {:?}, {:?}", span_group, stats);
    }

    println!("\nlatencies2 in microseconds");
    for (span_group, stats) in latencies2.map_values(summary_stats) {
        println!("  * {:?}, {:?}", span_group, stats);
    }
}
