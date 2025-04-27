//! Demonstration of the effects of the `reset()` and `clear()` methods of [`hdrhistogram::Histogram`].
//!
//! Based on the output, the `clear()` method does not seem to do what the documentation says -- it just
//! preserves the min and max values, not any other statistics.
//!
//! Both methods preserve the capacity of the histogram.

use hdrhistogram::Histogram;
use std::error::Error;

type Timing = Histogram<u64>;

fn print_hist_config_stats(name: &str, hist: &Timing) {
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
    println!("{name}: distinct={distinct}, len={len}, recorded_count={recorded_count}, low={low}, high={high}, min={min}, max={max}, mean={mean}, median={median}, stdev={stdev}, auto={auto}, empty={empty}");
}

fn main() -> Result<(), Box<dyn Error>> {
    const MAX_VALUE: u64 = 10_000_000;
    const STEP: u64 = 2;

    fn stress_histogram(name: &str, hist: &mut Timing) -> u64 {
        println!("*** stress_histogram({name}) ***");
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
        print_hist_config_stats(name, &hist);
        println!("{name}: last_recorded_value={last_recorded_value}");
        last_recorded_value
    }

    let mut hist_a = Timing::new_with_max(2, 2)?;
    {
        print_hist_config_stats("hist_a<0>", &hist_a);
        let last = stress_histogram("hist_a<1>", &mut hist_a);
        println!("*** expand hist_a and add next value ***");
        hist_a.auto(true);
        hist_a.record(last + 1)?;
        hist_a.auto(false);
        print_hist_config_stats("hist_a<2>", &hist_a);
    }

    let mut hist_b: Timing = hist_a.clone();
    {
        println!("*** clone hist_a to hist_b ***");
        print_hist_config_stats("hist_b<2>", &hist_a);
    }

    {
        stress_histogram("hist_a<3>", &mut hist_a);
        stress_histogram("hist_b<3>", &mut hist_b);
    }

    {
        println!("*** clear hist_a ***");
        hist_a.clear();
        print_hist_config_stats("hist_a<4>", &hist_a);
        println!("*** record 10 to hist_a ***");
        hist_a.record(10)?;
        print_hist_config_stats("hist_a<5>", &hist_a);
        stress_histogram("hist_a<6>", &mut hist_a);
    }

    {
        println!("*** reset hist_b ***");
        hist_b.reset();
        print_hist_config_stats("hist_b<4>", &hist_b);
        println!("*** record 10 to hist_b ***");
        hist_b.record(10)?;
        print_hist_config_stats("hist_b<5>", &hist_b);
        stress_histogram("hist_b<6>", &mut hist_b);
    }

    Ok(())
}
