//! Demonstration of how the `sigfic` constructor argument impacts values recorded in [`hdrhistogram::Histogram`].
//! Examples use [`Histogram::new_with_max`], which sets the `low` value to `1`.
//! Note that a higher `low` value changes the resolution of recorded values in a somewhat unintuitive way.

use std::error::Error;

use hdrhistogram::Histogram;

type Timing = Histogram<u64>;

fn main() -> Result<(), Box<dyn Error>> {
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
            Some(ref item) => {
                let (value_a, count_a) = (item.value_iterated_to(), item.count_at_value());
                item_a_opt = iter_a.next();
                (value_a, count_a)
            }
            None => (0, 0),
        };

        let (value_b, count_b) = match &mut item_b_opt {
            Some(ref item) => {
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

    Ok(())
}
