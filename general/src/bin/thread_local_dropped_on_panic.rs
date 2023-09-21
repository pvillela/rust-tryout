//! Demonstrates that a thread-local is dropped when a thread panics.

use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        FOO.with(|v| println!("Foo is being used: {:?}", v));
        thread::sleep(Duration::from_millis(100));
        panic!("BOOM!");
    });
    thread::sleep(Duration::from_millis(300));
    println!("Execution completed.");
}

#[derive(Debug)]
struct Foo(String);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("I'm being dropped: {:?}", self);
    }
}

thread_local! {
    static FOO: Foo = Foo("foo".to_owned());
}
