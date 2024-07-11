//! Demonstrates impact of [`std::hint::black_box`]. Run this example with `cargo run -r --bin black_box`.
//!
//! For the closures created from function [`work`], the compiler optimizes away the code unless [`black_box`]
//! is called on the returned value.
//!
//! For the closures created from function [`real_work`], the compiler apparently cannot optimizes away the code,
//! so calling [`black_box`] on the returned value has no impact on latency.

use sha2::{Digest, Sha256};
use std::{hint::black_box, time::Instant};

/// Returns first command line argument if it exists.
fn cmd_line_args() -> Option<u64> {
    let mut args = std::env::args();

    let res = args
        .nth(1)?
        .parse::<u64>()
        .expect("argument must be integer");

    Some(res)
}

pub fn work(extent: u64) -> u64 {
    let mut x = 1;
    for i in 0..extent {
        x *= i;
    }
    x
}

pub fn real_work(extent: u64) -> Vec<u8> {
    let buf = extent.to_be_bytes();
    let mut hasher = Sha256::new();
    for _ in 0..extent {
        hasher.update(buf);
    }
    hasher.finalize().as_slice().into()
}

fn main() {
    let extent = cmd_line_args().unwrap_or(10_000_000);

    let work1 = || work(extent);
    let work2 = || black_box(work(extent));

    let real_work1 = || real_work(extent);
    let real_work2 = || black_box(real_work(extent));

    {
        let start = Instant::now();
        let elapsed = Instant::now().duration_since(start);
        println!("Warm-up elapsed time: {elapsed:?}");
    }

    {
        {
            let start = Instant::now();
            work1();
            let elapsed = Instant::now().duration_since(start);
            println!("Latency for work1: {elapsed:?}");
        }

        {
            let start = Instant::now();
            black_box(work1)();
            let elapsed = Instant::now().duration_since(start);
            println!("Latency for black_box(work1): {elapsed:?}");
        }

        {
            let start = Instant::now();
            work2();
            let elapsed = Instant::now().duration_since(start);
            println!("Latency for work2: {elapsed:?}");
        }
    }

    {
        {
            let start = Instant::now();
            real_work1();
            let elapsed = Instant::now().duration_since(start);
            println!("Latency for real_work1: {elapsed:?}");
        }

        {
            let start = Instant::now();
            black_box(real_work1)();
            let elapsed = Instant::now().duration_since(start);
            println!("Latency for black_box(real_work1): {elapsed:?}");
        }

        {
            let start = Instant::now();
            real_work2();
            let elapsed = Instant::now().duration_since(start);
            println!("Latency for real_work2: {elapsed:?}");
        }
    }
}
