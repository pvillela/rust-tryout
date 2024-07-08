//! Demonstration of errore returned by methods of [`hdrhistogram::Histogram`].

use hdrhistogram::Histogram;

type Timing = Histogram<u64>;

fn main() {
    // CreationError
    {
        println!("***** CreationError");

        let low_is_zero = Timing::new_with_bounds(0, 10, 1);
        println!("{low_is_zero:?}");

        // LowExceedsMax error doesn't occur in practice because it is subsumed by HighLessThanTwiceLow.
        let low_exceeds_max = Timing::new_with_bounds(21, 20, 1);
        println!("{low_exceeds_max:?}");

        let high_less_than_twice_low = Timing::new_with_bounds(10, 15, 1);
        println!("{high_less_than_twice_low:?}");

        let sig_fig_exceeds_max = Timing::new_with_bounds(1, 10, 6);
        println!("{sig_fig_exceeds_max:?}");

        let cannot_represent_sig_fig_beyond_low =
            Timing::new_with_bounds(2_u64.pow(60), 2_u64.pow(62), 5);
        println!("{cannot_represent_sig_fig_beyond_low:?}");

        // UsizeTypeTooSmall can't happen when usize is 64 bits.
        let usize_type_too_small = Timing::new_with_bounds(1, u64::MAX, 5);
        println!(
            "usize_type_too_small.is_err()={}",
            usize_type_too_small.is_err()
        );
    }

    // RecordError
    {
        println!("\n***** RecordError");

        let mut hist = Timing::new_with_bounds(1, 10, 3).unwrap();
        println!("hist.is_auto_resize()={}", hist.is_auto_resize());
        let value_out_of_range_resize_disabled = hist.record(10000);
        println!("{value_out_of_range_resize_disabled:?}");

        hist.auto(true);
        let value_out_of_range_resize_disabled = hist.record(10000);
        println!(
            "value_out_of_range_resize_disabled.is_err()={}",
            value_out_of_range_resize_disabled.is_err()
        );

        // ResizeFailedUsizeTypeTooSmall can't happen when usize is 64 bits.
    }

    // AdditionError
    {
        println!("\n***** AdditionError");

        let mut hist = Timing::new_with_bounds(1, 10, 3).unwrap();
        println!("hist.is_auto_resize()={}", hist.is_auto_resize());
        println!(
            "hist.high()={}, hist.sigfig()={}",
            hist.high(),
            hist.sigfig()
        );

        let mut hist2 = Timing::new_with_bounds(1, u64::MAX, 5).unwrap();
        hist2.record(u64::MAX).unwrap();

        let other_addend_value_exceeds_range = hist.add(hist2.clone());
        println!("{other_addend_value_exceeds_range:?}");

        hist.auto(true);
        let other_addend_value_exceeds_range = hist.add(hist2);
        println!(
            "other_addend_value_exceeds_range.is_err()={}",
            other_addend_value_exceeds_range.is_err()
        );
        println!(
            "hist.high()={}, hist.sigfig()={}",
            hist.high(),
            hist.sigfig()
        );

        // ResizeFailedUsizeTypeTooSmall can't happen when usize is 64 bits.
    }
}
