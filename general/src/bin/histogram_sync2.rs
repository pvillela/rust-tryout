//! Example usage of [hdrhistogram::sync::SyncHistogram].
//! Modifies `histogram_sync` by removing the global static histogram and having thread-local
//! recorders initialized dynamically from an in-scope histogram.

use hdrhistogram::{
    sync::{Recorder, SyncHistogram},
    Histogram,
};
use std::{cell::RefCell, thread};

fn with_recorder(hist: &SyncHistogram<u64>, f: fn(&mut Recorder<u64>) -> ()) {
    thread_local! {
        static RECORDER: RefCell<Option<Recorder<u64>>> = RefCell::new(None);
    }

    RECORDER.with(|r| {
        let mut x = r.borrow_mut();
        let y = x.as_mut();
        if y.is_none() {
            *x = Some(hist.recorder());
        } else {
            println!("******* y is_some");
        }
        f(&mut x.as_mut().unwrap());
    });
}

fn main() {
    let hist = Histogram::<u64>::new_with_bounds(1, 9, 1).unwrap();
    let mut hist: SyncHistogram<u64> = hist.into();

    {
        let mut recorder1 = hist.recorder();
        {
            let mut recorder2 = hist.recorder();
            recorder1.record_n(3, 3).unwrap();
            recorder2.record_n(7, 3).unwrap();
        }
        drop(recorder1);

        // refresh() blockx until all existing recorders are dropped.
        hist.refresh();

        println!("mean={}", hist.mean());
        println!("histogram={:?}", hist);
    }

    {
        thread::scope(|s| {
            s.spawn(|| {
                with_recorder(&hist, |r| r.record(1).unwrap());
            });
        });

        thread::scope(|s| {
            s.spawn(|| {
                // with_recorder(|r| r.record_n(4, 2).unwrap());
                with_recorder(&hist, |r| {
                    r.record(4).unwrap();
                });
                with_recorder(&hist, |r| {
                    r.record(4).unwrap();
                });
            });
        });

        thread::scope(|s| {
            s.spawn(|| {
                // with_recorder(|r| r.record_n(9, 3).unwrap());
                with_recorder(&hist, |r| {
                    r.record(9).unwrap();
                });
                with_recorder(&hist, |r| {
                    r.record(9).unwrap();
                });
                with_recorder(&hist, |r| {
                    r.record(9).unwrap();
                });
            });
        });

        // refresh() blockx until all existing recorders are dropped.
        hist.refresh();

        println!("mean={}", hist.mean());
        println!("histogram={:?}", hist);
    }

    {
        {
            let mut recorder1 = hist.recorder();
            let mut recorder2 = hist.recorder();
            recorder1.record_n(6, 3).unwrap();
            recorder2.record_n(8, 3).unwrap();
        }

        // refresh() blockx until all existing recorders are dropped.
        hist.refresh();

        println!("mean={}", hist.mean());
        println!("histogram={:?}", hist);
    }
}
