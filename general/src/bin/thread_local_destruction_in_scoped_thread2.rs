//! Demonstrates that scoped thread implicit join fails 'happens before' guarantee.
//! Reported bug: https://github.com/rust-lang/rust/issues/116237

use std::{hint::black_box, mem::replace, thread};

static mut CONTROL: Option<String> = None;

fn main() {
    // Explicit join of scoped thread.
    {
        for _ in 0..5000 {
            thread::scope(|s| {
                let h = s.spawn(|| {
                    FOO.with(|v| {
                        black_box(v);
                    });
                });
                h.join().unwrap();
            });

            // SAFETY: this happens after the thread join, which provides a `happens before` guarantee
            let v = unsafe { &CONTROL };
            black_box(format!("{:?}", v));
            print!(".");
        }
        println!("Completed EXPLICIT join loop.");
    }

    println!();

    // Implicit join of scoped thread.
    {
        for _ in 0..5000 {
            thread::scope(|s| {
                let _h = s.spawn(|| {
                    FOO.with(|v| {
                        black_box(v);
                    });
                });
            });

            // SAFETY: this happens after the implicit thread join, which should provide a `happens before` guarantee
            let v = unsafe { &CONTROL };
            black_box(format!("{:?}", v));
            print!(".");
        }
        println!("Completed IMPLICIT join loop.");
    }
}

struct Foo(());

impl Drop for Foo {
    fn drop(&mut self) {
        // SAFETY: this happens before the thread join, which provides a `happens before` guarantee
        let _: Option<String> = unsafe { replace(&mut CONTROL, Some("abcd".to_owned())) };
    }
}

thread_local! {
    static FOO: Foo = Foo(());
}
