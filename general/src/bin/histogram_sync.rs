//! Example usage of [hdrhistogram::sync::SyncHistogram].

use std::{
    cell::RefCell,
    sync::{OnceLock, RwLock},
    thread,
};

use hdrhistogram::{
    sync::{Recorder, SyncHistogram},
    Histogram,
};

fn get_histogram() -> &'static RwLock<SyncHistogram<u64>> {
    static HIST_LOCK: OnceLock<RwLock<SyncHistogram<u64>>> = OnceLock::new();
    HIST_LOCK.get_or_init(|| {
        let hist = Histogram::<u64>::new_with_bounds(1, 9, 1).unwrap();
        let sync_hist: SyncHistogram<u64> = hist.into();
        RwLock::new(sync_hist)
    })
}

fn with_recorder(f: fn(&mut Recorder<u64>) -> ()) {
    thread_local! {
        static RECORDER: RefCell<Recorder<u64>> = RefCell::new(get_histogram().read().unwrap().recorder());
    }

    RECORDER.with(|r| {
        f(&mut r.borrow_mut());
    });
}

fn main() {
    {
        let mut hist = get_histogram().write().unwrap();
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
                with_recorder(|r| r.record(1).unwrap());
            });
        });

        thread::scope(|s| {
            s.spawn(|| {
                with_recorder(|r| {
                    r.record_n(4, 2).unwrap();
                });
            });
        });

        thread::scope(|s| {
            s.spawn(|| {
                with_recorder(|r| r.record_n(9, 3).unwrap());
            });
        });

        // refresh() blockx until all existing recorders are dropped.
        get_histogram().write().unwrap().refresh();

        println!("mean={}", get_histogram().read().unwrap().mean());
        println!("histogram={:?}", get_histogram());
    }

    {
        let mut hist = get_histogram().write().unwrap();
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
