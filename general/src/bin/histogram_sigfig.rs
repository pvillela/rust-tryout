//! Demonstration of how the `sigfig` constructor argument impacts values recorded in [`hdrhistogram::Histogram`].
//! - Shows how different values are aliased depending on the `sigfig` used.
//! - Shows how statistics such as the mean can be distorted.
//! - Shows how the `sigfig` impacts the capacity of a non-auto histogram.
//!
//! Examples use [`Histogram::new_with_max`], which sets the `low` value to `1`.
//! Note that a higher `low` value changes the resolution of recorded values in a somewhat unintuitive way.

use std::error::Error;

use hdrhistogram::Histogram;

type Timing = Histogram<u64>;

/// Shows how different values are aliased depending on the `sigfig` used.
fn data_aliasing() -> Result<(), Box<dyn Error>> {
    println!("*** low_sigfig example ***");

    const MAX_VALUE: u64 = 50;

    let mut hist_a = Timing::new_with_max(MAX_VALUE, 1)?;
    let mut hist_b = Timing::new_with_max(MAX_VALUE, 2)?;

    for i in 1..=MAX_VALUE {
        hist_a.record(i)?;
        hist_b.record(i)?;
    }

    let mut iter_a = hist_a.iter_recorded();
    let mut iter_b = hist_b.iter_recorded();
    let (mut item_a_opt, mut item_b_opt) = (iter_a.next(), iter_b.next());

    while item_a_opt.is_some() || item_b_opt.is_some() {
        let (value_a, count_a) = match &mut item_a_opt {
            Some(item) => {
                let (value_a, count_a) = (item.value_iterated_to(), item.count_at_value());
                item_a_opt = iter_a.next();
                (value_a, count_a)
            }
            None => (0, 0),
        };

        let (value_b, count_b) = match &mut item_b_opt {
            Some(item) => {
                let (value_b, count_b) = (item.value_iterated_to(), item.count_at_value());
                item_b_opt = iter_b.next();
                (value_b, count_b)
            }
            None => (0, 0),
        };

        println!(
            "value_a={}, count_a={}, value_b={}, count_b={}",
            value_a, count_a, value_b, count_b
        );
    }

    println!("Distortion of means:");
    {
        let real_mean = (1. + MAX_VALUE as f64) / 2.;
        let hist_a_mean = hist_a.mean();
        let hist_b_mean = hist_b.mean();
        println!("real_mean={real_mean}, hist_a_mean={hist_a_mean}, hist_b_mean={hist_b_mean}");
    }

    Ok(())
}

fn print_hist_config_stats(name: &str, hist: &Timing, last_recorded_value: u64) {
    let distinct = hist.distinct_values();
    let len = hist.len();
    let recorded_count = hist.iter_recorded().count();
    let low = hist.low();
    let high = hist.high();
    let min = hist.min();
    let max = hist.max();
    let mean = hist.mean();
    let median = hist.value_at_quantile(0.5);
    let stdev = hist.stdev();
    let auto = hist.is_auto_resize();
    let empty = hist.is_empty();
    println!(
        "{name}: distinct={distinct}, len={len}, recorded_count={recorded_count}, last_recorded_value={last_recorded_value}, low={low}, high={high}, min={min}, max={max}, mean={mean}, median={median}, stdev={stdev}, auto={auto}, empty={empty}"
    );
}

/// Shows how the `sigfig` impacts the capacity of a non-auto histogram.
fn high_sigfig() {
    println!("*** high_sigfig example ***");

    const MAX_VALUE: u64 = 10_000_000;
    const STEP: u64 = 10;

    let create_and_stress_histogram = |high: u64, sigfig: u8| {
        let mut hist = Timing::new_with_max(high, sigfig).unwrap();
        let name = format!("hist_z_{high}_{sigfig}");
        let mut last_recorded_value = 0;
        for i in 1..=MAX_VALUE / STEP {
            let res = hist.record(i * STEP);
            if res.is_err() {
                for j in ((i - 1) * STEP + 1)..=(i * STEP) {
                    let res = hist.record(j);
                    if res.is_err() {
                        last_recorded_value = j - 1;
                        break;
                    }
                }
                break;
            }
        }
        print_hist_config_stats(&name, &hist, last_recorded_value);
    };

    create_and_stress_histogram(2, 2);
    create_and_stress_histogram(2, 5);
    create_and_stress_histogram(10, 2);
    create_and_stress_histogram(10, 5);
    create_and_stress_histogram(100, 2);
    create_and_stress_histogram(100, 5);
    create_and_stress_histogram(1_000, 2);
    create_and_stress_histogram(1_000, 5);
    create_and_stress_histogram(10_000, 2);
    create_and_stress_histogram(10_000, 5);
    create_and_stress_histogram(100_000, 2);
    create_and_stress_histogram(100_000, 5);
    create_and_stress_histogram(1_000_000, 2);
    create_and_stress_histogram(1_000_000, 5);
}

fn main() {
    data_aliasing().unwrap();
    println!();
    high_sigfig();
}
