//! Demonstrates that, on scoped threads that are implicitly joined at the end of the scope,
//! a thread-local's destructor is not guaranteed to complete before the scope is exited.

use std::{hint::black_box, sync::Mutex, thread, time::Duration};

static MTX: Mutex<()> = Mutex::new(());

fn main() {
    // Implicit join of scoped thread.
    {
        let lock = MTX.lock().unwrap();

        thread::scope(|s| {
            let _h = s.spawn(|| {
                println!("on {:?}", thread::current().id());
                FOO.with(|v| {
                    black_box(v);
                });
            });
        });

        println!("Executed 1st thread scope.");
        drop(lock);
        thread::sleep(Duration::from_millis(100));
    }

    // Explicit join of scoped thread.
    {
        let lock = MTX.lock().unwrap();

        thread::scope(|s| {
            let h = s.spawn(|| {
                println!("on {:?}", thread::current().id());
                FOO.with(|v| {
                    black_box(v);
                });
            });
            h.join().unwrap();
        });

        println!("Executed 2nd thread scope.");
        drop(lock);
        thread::sleep(Duration::from_millis(100));
    }
}

struct Foo(());

impl Drop for Foo {
    fn drop(&mut self) {
        println!("entering Foo::drop on {:?}", thread::current().id());
        let _lock = MTX.lock().unwrap();
        println!("Foo::drop completed on {:?}", thread::current().id());
    }
}

thread_local! {
    static FOO: Foo = Foo(());
}
