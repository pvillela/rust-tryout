//! Compares different forms of `busy_work` functions in terms of their deviation from the target latency.
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
    let target_effort_mul = calibrate_busy_work(busy_work_mul, target_latency);
    let target_effort_div = calibrate_busy_work(busy_work_div, target_latency);

    println!("target_latency_nanos={target_latency_nanos}");
    println!(
        "target_effort_sha={target_effort_sha}, target_effort_mul={target_effort_mul}, target_effort_div={target_effort_div}"
    );

    const N: usize = 10;

    let mut sum2dev_sha = 0.;
    let mut sum2dev_mul = 0.;
    let mut sum2dev_div = 0.;
    for _ in 0..N {
        let latency_sha_nanos = latency(|| busy_work_sha(target_effort_sha)).as_nanos() as f64;
        let latency_mul_nanos = latency(|| busy_work_mul(target_effort_mul)).as_nanos() as f64;
        let latency_div_nanos = latency(|| busy_work_div(target_effort_div)).as_nanos() as f64;
        sum2dev_sha += (latency_sha_nanos - target_latency_nanos).powi(2);
        sum2dev_mul += (latency_mul_nanos - target_latency_nanos).powi(2);
        sum2dev_div += (latency_div_nanos - target_latency_nanos).powi(2);

        println!(
            "latency_sha_nanos={latency_sha_nanos}, latency_mul_nanos={latency_mul_nanos}, latency_div_nanos={latency_div_nanos}"
        );
    }

    let stdev_sha = (sum2dev_sha / N as f64).sqrt();
    let rel_stdev_sha = stdev_sha / target_latency_nanos;
    let stdev_mul = (sum2dev_mul / N as f64).sqrt();
    let rel_stdev_mul = stdev_mul / target_latency_nanos;
    let stdev_div = (sum2dev_div / N as f64).sqrt();
    let rel_stdev_div = stdev_div / target_latency_nanos;

    println!("stdev_sha={stdev_sha}, stdev_mul={stdev_mul}, stdev_div={stdev_div}",);
    println!(
        "rel_stdev_sha={rel_stdev_sha}, rel_stdev_mul={rel_stdev_mul}, rel_stdev_div={rel_stdev_div}",
    );
}

/// Invokes `f` once and returns its latency.
#[inline(always)]
pub fn latency(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

#[inline(always)]
fn swap_neighbour(arr: &mut [Duration], i: usize) {
    if arr[i + 1] < arr[i] {
        let temp = arr[i];
        arr[i] = arr[i + 1];
        arr[i + 1] = temp;
    }
}

/// Invokes `f` 3 times and returns the median latency.
#[inline(always)]
pub fn latency_m(f: impl Fn()) -> Duration {
    let mut lats = [latency(&f), latency(&f), latency(&f)];
    for k in (0..2).rev() {
        for i in 0..k {
            swap_neighbour(&mut lats, i);
        }
    }
    lats[1]
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
pub fn busy_work_mul(effort: u32) {
    let extent = black_box(effort);
    let mut v: u64;
    for _ in 0..extent {
        v = black_box(u64::MAX).wrapping_mul(black_box(black_box(u64::MAX)));
        black_box(v);
    }
}

/// Function that does a significant amount of computation to support validation of benchmarking frameworks.
/// `effort` is the number of iterations that determines the amount of work performed.
pub fn busy_work_div(effort: u32) {
    const K: f64 = 104729.; // 10,000th prime number
    const M: f64 = 104723.; // 9,999th prime number
    // 1st 17 decimal digits of pi.
    const F: f64 = 0.141_592_653_589_793_23;
    let extent = black_box(effort);
    let mut vf = F;
    for _ in 0..extent {
        vf = black_box(((M + vf) / K + F) / 2.); // always in interval (0.07, 1)
    }
    black_box(vf);
}

/// Function that does a significant amount of computation to support validation of benchmarking frameworks.
/// `effort` is the number of iterations that determines the amount of work performed.
pub fn busy_work_exp(effort: u32) {
    const M: u64 = 7;
    let extent = black_box(effort);
    let mut v = M as f64;
    for _ in 0..extent {
        let ve = v.exp();
        let vei = ve.floor();
        let vef = ve - vei;
        let vem = vei as u64 % M + 1;
        v = vem as f64 + vef;
    }
    black_box(v);
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
    let latency = latency_m(|| busy_work(calibration_effort));
    (target_latency.as_nanos() * calibration_effort as u128 / latency.as_nanos()) as u32
}
