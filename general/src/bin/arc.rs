use std::{ops::Deref, sync::Arc, thread};

struct Foo {
    v: Vec<i32>,
}

// This example shows that Arc::new() copies data to the heap.
// The original data x is created in the stack of thread t1, then it is owned by
// an Arc that is passed to thread t2, and finally the Arc is accessed by the
// main thread when it joins t2.
fn main() {
    let t1 = thread::spawn(move || {
        let x = Foo {
            v: vec![1, 2, 3, 4],
        };
        println!("{:p}", &x);
        let ax = Arc::new(x);
        // let y = x; // doesn't compile because x was moved
        println!("{:p}, {:p}, {:p}, {:p}", ax.deref(), &ax.deref(), ax, &ax);

        let t2 = thread::spawn(move || ax);
        // let y = ax; // doesn't compile because ax was moved
        t2
    });

    let t2 = t1.join();
    let x = t2.unwrap().join().unwrap();
    println!("{:?}", x.v);
}
