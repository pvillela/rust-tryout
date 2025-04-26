//! Demonstration of the effects of the `reset()` and `clear()` methods of [`hdrhistogram::Histogram`].

use hdrhistogram::Histogram;
use std::error::Error;

type Timing = Histogram<u64>;

fn print_hist_config_stats(name: &str, hist: &Timing) {
    let low = hist.low();
    let high = hist.high();
    let len = hist.len();
    let min = hist.min();
    let max = hist.max();
    let mean = hist.mean();
    let median = hist.value_at_quantile(0.5);
    let stdev = hist.stdev();
    let auto = hist.is_auto_resize();
    let empty = hist.is_empty();
    println!("{name}: low={low}, high={high}, len={len}, min={min}, max={max}, mean={mean}, median={median}, stdev={stdev}, auto={auto}, empty={empty}");
}

fn main() -> Result<(), Box<dyn Error>> {
    const MAX_VALUE: u64 = 10_000;

    let mut hist_a = Timing::new(5)?;
    let mut hist_b: Timing = Histogram::new_from(&hist_a);

    print_hist_config_stats("hist_a new", &hist_a);
    print_hist_config_stats("hist_b new", &hist_a);

    for i in 1..=MAX_VALUE {
        hist_a.record(i)?;
        hist_b.record(i)?;
    }

    print_hist_config_stats("hist_a recorded", &hist_a);
    print_hist_config_stats("hist_b recorded", &hist_a);

    let mut hist_c: Timing = Histogram::new_from(&hist_a);
    print_hist_config_stats("hist_c", &hist_c);

    hist_a.clear();
    print_hist_config_stats("hist_a cleared", &hist_a);

    hist_b.reset();
    print_hist_config_stats("hist_b reset", &hist_b);

    hist_a.auto(false);
    print_hist_config_stats("hist_a no auto", &hist_a);

    hist_b.auto(false);
    print_hist_config_stats("hist_b no auto", &hist_b);

    for i in 1..=MAX_VALUE {
        hist_a.record(i)?;
    }
    print_hist_config_stats("hist_a no auto re-recorded", &hist_a);

    for i in 1..=MAX_VALUE {
        hist_b.record(i)?;
    }
    print_hist_config_stats("hist_b no auto re-recorded", &hist_b);

    for i in 1..=MAX_VALUE {
        hist_c.record(i)?;
    }
    print_hist_config_stats("hist_c recorded", &hist_c);

    let a_excess = hist_a.record(MAX_VALUE * 100);
    println!("a_excess={a_excess:?}");

    let b_excess = hist_b.record(MAX_VALUE * 100);
    println!("b_excess={b_excess:?}");

    let c_excess = hist_c.record(MAX_VALUE * 100);
    println!("c_excess={c_excess:?}");
    print_hist_config_stats("hist_c with excess", &hist_c);

    Ok(())
}
