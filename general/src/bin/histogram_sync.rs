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
    thread::scope(|s| {
        s.spawn(|| {
            with_recorder(|r| r.record(1).unwrap());
        });
    });

    thread::scope(|s| {
        s.spawn(|| {
            with_recorder(|r| {
                r.record(4).unwrap();
                r.record(4).unwrap();
            });
        });
    });

    thread::scope(|s| {
        s.spawn(|| {
            with_recorder(|r| r.record(9).unwrap());
            with_recorder(|r| r.record(9).unwrap());
            with_recorder(|r| r.record(9).unwrap());
        });
    });

    get_histogram().write().unwrap().refresh();

    println!("mean={}", get_histogram().read().unwrap().mean());
    println!("histogram={:?}", get_histogram());
}
