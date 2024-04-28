//! Demonstrates how a thread-local variable can be encapsulated in a struct

use std::thread::{self, LocalKey, ThreadId};

thread_local! {
    pub static FOO: ThreadId = thread::current().id();
}

struct Control {
    tl: &'static LocalKey<ThreadId>,
}

impl Control {
    fn show(&self) {
        self.tl
            .with(|x| println!("control.tl={:?}, tid={:?}", x, thread::current().id()));
    }
}

fn main() {
    let control = Control { tl: &FOO };
    thread::scope(|s| {
        s.spawn(|| {
            control.show();
        });
        s.spawn(|| {
            control.show();
        });
    });
}
