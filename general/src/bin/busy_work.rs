//! Compares two forms of `busy_work` functions in terms of their deviation from the target latency.
//!
//! Execute it by running:
//! ```
//! cargo run -r --bin busy_work
//! ```

use sha2::{Digest, Sha256};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

fn main() {
    let target_latency = Duration::from_nanos(2000);
    let target_latency_nanos = target_latency.as_nanos() as f64;
    let target_effort_sha = calibrate_busy_work(busy_work_sha, target_latency);
    let target_effort_simple = calibrate_busy_work(busy_work_simple, target_latency);

    println!("target_latency_nanos={}", target_latency.as_nanos());
    println!("target_effort_sha={}", target_effort_sha);
    println!("target_effort_simple={}", target_effort_simple);

    const N: usize = 10;

    let mut sum2dev_sha = 0.;
    let mut sum2dev_simple = 0.;
    for _ in 0..N {
        let latency_sha_nanos = latency(|| busy_work_sha(target_effort_sha)).as_nanos() as f64;
        let latency_simple_nanos =
            latency(|| busy_work_simple(target_effort_simple)).as_nanos() as f64;
        sum2dev_sha += (latency_sha_nanos - target_latency_nanos).powi(2);
        sum2dev_simple += (latency_simple_nanos - target_latency_nanos).powi(2);

        println!(
            "latency_sha_nanos={}, latency_simple_nanos={}",
            latency_sha_nanos, latency_simple_nanos
        );
    }

    let stdev_sha = (sum2dev_sha / N as f64).sqrt();
    let rel_stdev_sha = stdev_sha / target_latency_nanos;
    let stdev_simple = (sum2dev_simple / N as f64).sqrt();
    let rel_stdev_simple = stdev_simple / target_latency_nanos;

    println!("stdev_sha={stdev_sha}, stdev_simple={stdev_simple}",);
    println!("rel_stdev_sha={rel_stdev_sha}, rel_stdev_simple={rel_stdev_simple}",);
}

/// Invokes `f` once and returns its latency.
#[inline(always)]
pub fn latency(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

/// Function that does a significant amount of computation to support validation of benchmarking frameworks.
/// `effort` is the number of iterations that determines the amount of work performed.
pub fn busy_work_sha(effort: u32) {
    let extent = black_box(effort);
    let seed = black_box(0_u64);
    let buf = seed.to_be_bytes();
    let mut hasher = Sha256::new();
    for _ in 0..extent {
        hasher.update(buf);
    }
    let hash = hasher.finalize();
    black_box(hash);
}

/// Function that does a significant amount of computation to support validation of benchmarking frameworks.
/// `effort` is the number of iterations that determines the amount of work performed.
pub fn busy_work_simple(effort: u32) {
    let extent = black_box(effort);
    let mut v: u64;
    for _ in 0..extent {
        v = black_box(u64::MAX).wrapping_mul(black_box(black_box(u64::MAX)));
        black_box(v);
    }
}

/// Returns an estimate of the number of iterations required for `busy_work` to have a latency
/// of `target_micros`.
///
/// Calls [`calibrate_busy_work_x`] with a predefined `calibration_effort`;
pub fn calibrate_busy_work(busy_work: fn(u32), target_latency: Duration) -> u32 {
    const CALIBRATION_EFFORT: u32 = 200_000;
    calibrate_busy_work_x(busy_work, target_latency, CALIBRATION_EFFORT)
}

/// Returns an estimate of the number of iterations required for `busy_work` to have a latency
/// of `target_micros`. `calibration_effort` is the number of iterations executed during calibration.
pub fn calibrate_busy_work_x(
    busy_work: fn(u32),
    target_latency: Duration,
    calibration_effort: u32,
) -> u32 {
    let latency = latency(|| busy_work(calibration_effort));
    (target_latency.as_nanos() * calibration_effort as u128 / latency.as_nanos()) as u32
}
