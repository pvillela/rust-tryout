//! Demonstration of values and iteration through [`hdrhistogram::Histogram`].
//!
//! ```
//! cargo run --package general --bin histogram_values_and_iter --all-features
//! ```

use hdrhistogram::Histogram;

fn record_and_show(sigfig: u8, data_pump: fn(&mut Histogram<u64>) -> String) {
    let mut hist = Histogram::<u64>::new(sigfig).unwrap();
    let name = data_pump(&mut hist);
    println!("*** sigfig={sigfig}, data_pump={name}");

    let count = hist.len();
    let mean = hist.mean();
    let min = hist.min();
    let p50 = hist.value_at_quantile(0.5);
    let median_equiv = hist.median_equivalent(p50);
    let max = hist.max();
    let counted_values: Vec<(u64, u64)> = hist
        .iter_recorded()
        .map(|r| (r.value_iterated_to(), r.count_at_value()))
        .collect();

    println!(
        "count={count}, mean={mean}, min={min}, p50={p50}, median_equiv={median_equiv}, max={max}"
    );
    println!("counted_values={counted_values:?}");
    println!("hist={hist:?}");
}

fn data_pump_1(hist: &mut Histogram<u64>) -> String {
    hist.record_n(0, 5).unwrap();
    hist.record(1).unwrap();
    hist.record(2).unwrap();
    hist.record(3).unwrap();
    hist.record(4).unwrap();
    hist.record(5).unwrap();
    hist.record(6).unwrap();
    hist.record(7).unwrap();
    hist.record(8).unwrap();
    hist.record(9).unwrap();
    hist.record_n(10, 10).unwrap();
    hist.record(11).unwrap();
    hist.record(12).unwrap();
    hist.record(13).unwrap();
    hist.record(14).unwrap();
    hist.record(15).unwrap();
    hist.record(16).unwrap();
    hist.record(17).unwrap();
    hist.record(18).unwrap();
    hist.record(19).unwrap();
    hist.record_n(20, 5).unwrap();

    "data_pump_1".to_owned()
}
fn data_pump_2(hist: &mut Histogram<u64>) -> String {
    hist.record(10).unwrap();
    hist.record(12).unwrap();

    "data_pump_2".to_owned()
}

fn main() {
    {
        let sigfig = 0;
        let data_pump = data_pump_1;
        record_and_show(sigfig, data_pump);
    }

    println!();

    {
        let sigfig = 1;
        let data_pump = data_pump_1;
        record_and_show(sigfig, data_pump);
    }

    println!();
    {
        let sigfig = 0;
        let data_pump = data_pump_2;
        record_and_show(sigfig, data_pump);
    }

    println!();

    {
        let sigfig = 1;
        let data_pump = data_pump_2;
        record_and_show(sigfig, data_pump);
    }
}
