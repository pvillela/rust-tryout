//! Demonstrates the use of the [`thread_local`] crate with static variables.
//! This use case is useful when the spawning of threads is not under our control.
//! That is the case, for example, with tracing instrumentation libraries, where the spawning of threads is under
//! the control of the code being instrumented and the scope within which the instrumentation code executes
//! may be deeply nested in the instrumented code.
//!
//! Keep in mind, however, that my `thread-local-collect` library may be a simpler solution for this use case.

use std::{
    cell::RefCell,
    collections::HashMap,
    sync::Arc,
    thread::{self, spawn, JoinHandle},
    time::Duration,
};
use thread_local::ThreadLocal;

thread_local! {
    static TL: RefCell<Arc<ThreadLocal<RefCell<HashMap<i32, String>>>>> = RefCell::new(Arc::new(ThreadLocal::new()));
}

fn main() {
    // Our top-level code
    let tls = Arc::new(ThreadLocal::new());

    {
        // Code we don't control

        let hs: Vec<JoinHandle<()>> = (0..5)
            .map(|i| {
                let tls = tls.clone();
                spawn(move || {
                    {
                        // Some code we don't control
                    }

                    {
                        {
                            // Our deeply nested code

                            TL.with(|x| {
                                let mut y = x.borrow_mut();
                                *y = tls;
                                let mut x = y.get_or(|| RefCell::new(HashMap::new())).borrow_mut();
                                println!("Before 1st insert for {i}: {x:?}");
                                x.insert(10 + i, "a".to_owned() + &i.to_string());
                                println!("After 1st insert for {i}: {x:?}");
                                thread::sleep(Duration::from_millis(10));
                                x.insert(20 + i, "b".to_owned() + &i.to_string());
                                println!("After 2nd insert for {i}: {x:?}");
                            });
                        }
                    }

                    {
                        // Some code we don't control
                    }
                })
            })
            .collect();
        hs.into_iter().for_each(|h| h.join().unwrap());
    }

    // Our top-level code
    let tls = Arc::try_unwrap(tls).unwrap();
    tls.into_iter()
        .for_each(|x| println!("tls.into_iter: {x:?}"));
}
