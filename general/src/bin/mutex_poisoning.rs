//! Demonstrates conditions for Mutex poisoning.

use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let mutex_a = Mutex::new(0);
    let guard_a = mutex_a.lock().unwrap();

    // Panicking while borrowing `MutexGuard` does NOT poinson the mutex.
    {
        thread::scope(|s| {
            let h_a1 = s.spawn(|| {
                assert_eq!(guard_a.deref(), &1, "h_a1");
            });
            let _ = h_a1.join();

            let h_a2 = s.spawn(|| {
                println!("*** h_a2: guard value: {guard_a:?}");
            });
            let _ = h_a2.join();
        });
        drop(guard_a);
        let guard = mutex_a.lock().unwrap();
        println!("*** mutex is not poisoned: {guard:?}");
    }

    // Panicking while owing a `MutexGuard` itself poinsons the mutex.
    {
        let mutex_b = Arc::new(Mutex::new(0));

        thread::scope(|s| {
            let mutex_b1 = mutex_b.clone();
            let h_b1 = s.spawn(move || {
                let guard_b = mutex_b1.lock().unwrap();
                assert_eq!(guard_b.deref(), &1, "h_b1");
            });
            let _ = h_b1.join();

            let h_b2 = s.spawn(move || {
                let guard_b = mutex_b.lock().unwrap();
                println!("*** h_b2: guard value: {guard_b:?}");
            });
            let _ = h_b2.join();
        });
    }
}
