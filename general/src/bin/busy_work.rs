//! Compares different forms of `busy_work` functions in terms of their deviation from the target latency.
//!
//! Execute it by running:
//! ```
//! cargo run -r --bin busy_work
//! ```

use general::fwk::comb_sort::comb_sort;
use sha2::{Digest, Sha256};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

fn main() {
    let target_latency = Duration::from_nanos(2000);
    let target_latency_nanos = target_latency.as_nanos() as f64;

    let target_effort_sha = calibrate_busy_work(busy_work_sha, target_latency);
    let target_effort_umul = calibrate_busy_work(busy_work_umul, target_latency);
    let target_effort_fmul = calibrate_busy_work(busy_work_fmul, target_latency);

    println!("target_latency_nanos={target_latency_nanos}");
    println!(
        "target_effort_sha={target_effort_sha}, target_effort_umul={target_effort_umul}, target_effort_fmul={target_effort_fmul}"
    );

    const N: usize = 10;

    let mut sum2dev_sha = 0.;
    let mut sum2dev_umul = 0.;
    let mut sum2dev_fmul = 0.;
    for _ in 0..N {
        let latency_sha_nanos = latency(|| busy_work_sha(target_effort_sha)).as_nanos() as f64;
        let latency_umul_nanos = latency(|| busy_work_umul(target_effort_umul)).as_nanos() as f64;
        let latency_fmul_nanos = latency(|| busy_work_fmul(target_effort_fmul)).as_nanos() as f64;
        sum2dev_sha += (latency_sha_nanos - target_latency_nanos).powi(2);
        sum2dev_umul += (latency_umul_nanos - target_latency_nanos).powi(2);
        sum2dev_fmul += (latency_fmul_nanos - target_latency_nanos).powi(2);

        println!(
            "latency_sha_nanos={latency_sha_nanos}, latency_umul_nanos={latency_umul_nanos}, latency_fmul_nanos={latency_fmul_nanos}"
        );
    }

    let stdev_sha = (sum2dev_sha / N as f64).sqrt();
    let rel_stdev_sha = stdev_sha / target_latency_nanos;
    let stdev_umul = (sum2dev_umul / N as f64).sqrt();
    let rel_stdev_umul = stdev_umul / target_latency_nanos;
    let stdev_fmul = (sum2dev_fmul / N as f64).sqrt();
    let rel_stdev_fmul = stdev_fmul / target_latency_nanos;

    println!("stdev_sha={stdev_sha}, stdev_umul={stdev_umul}, stdev_fmul={stdev_fmul}",);
    println!(
        "rel_stdev_sha={rel_stdev_sha}, rel_stdev_umul={rel_stdev_umul}, rel_stdev_fmul={rel_stdev_fmul}",
    );
}

/// Invokes `f` once and returns its latency.
#[inline(always)]
pub fn latency(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now().duration_since(start)
}

/// Invokes `f` `R` times and returns the median latency.
#[inline(always)]
fn latency_m<const R: usize>(f: impl Fn()) -> Duration {
    if R <= 1 {
        return latency(&f);
    }

    let mut lats = [Duration::new(0, 0); R];

    for i in 0..R {
        lats[i] = latency(&f);
    }

    comb_sort(&mut lats);

    if R % 2 == 1 {
        lats[R / 2]
    } else {
        let m1 = lats[R / 2 - 1].as_nanos();
        let m2 = lats[R / 2].as_nanos();
        let m = (m1 + m2) / 2;
        Duration::from_nanos(m as u64)
    }
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
pub fn busy_work_umul(effort: u32) {
    let extent = black_box(effort);
    let mut v: u64;
    for _ in 0..extent {
        v = black_box(u64::MAX).wrapping_mul(black_box(black_box(u64::MAX)));
        black_box(v);
    }
}

/// Function that does a significant amount of computation to support validation of benchmarking frameworks.
/// `effort` is the number of iterations that determines the amount of work performed.
pub fn busy_work_fmul(effort: u32) {
    const F: f64 = 0.5;
    let extent = black_box(effort);
    let mut vf = F;
    for _ in 0..extent {
        vf = black_box(((1. + vf) * (1. + vf)).fract());
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

/// Returns an estimate of the number of iterations required for `busy_work` to have latency `target_latency`.
///
/// Calls [`calibrate_busy_work_x`] with predefined default `calibration_effort` and `R` values.
pub fn calibrate_busy_work(busy_work: fn(u32), target_latency: Duration) -> u32 {
    const CALIBRATION_EFFORT: u32 = 100_000;
    const R: usize = 0;
    calibrate_busy_work_x::<R>(busy_work, target_latency, CALIBRATION_EFFORT)
}

/// Returns an estimate of the number of iterations required for `busy_work` to have latency `target_latency`.
///
/// # Generic parameters:
/// - `R`: the number of times the calibration is run. The median calibration is returned. An extremely high value
///   for `R` will cause a stack overflow.
///
/// # Arguments
/// - `busy_work`: function to be calibrated.
/// - `target_latency`: target latency.
/// - `calibration_effort`: the number of iterations executed during calibration.
pub fn calibrate_busy_work_x<const R: usize>(
    busy_work: fn(u32),
    target_latency: Duration,
    calibration_effort: u32,
) -> u32 {
    let latency = latency_m::<R>(|| busy_work(calibration_effort));
    (target_latency.as_nanos() * calibration_effort as u128 / latency.as_nanos()) as u32
}
