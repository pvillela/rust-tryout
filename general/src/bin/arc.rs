use std::{ops::Deref, sync::Arc, thread};

struct Foo {
    v: Vec<i32>,
    b: Box<i32>,
}

// This example shows that Arc::new() copies data to the heap.
// The original data x is created on the stack of thread t1, then it is owned by
// an Arc that is passed to thread t2, and finally the Arc is accessed by the
// main thread when it joins t2.
fn main() {
    let t1 = thread::spawn(move || {
        let x = Foo {
            v: vec![1, 2, 3, 4],
            b: Box::new(1),
        };

        println!("Pointer value of -- &x: {:p}", &x);

        let ax = Arc::new(x);
        // let y = x; // doesn't compile because x was moved

        println!(
            "Pointer values of -- ax: {:p}, &ax: {:p}, ax.deref(): {:p}, &ax.deref(): {:p}",
            ax,
            &ax,
            ax.deref(),
            &ax.deref(),
        );
        println!(
            "Pointer values of -- ax.v: n/a, &ax.v: {:p}, ax.deref().v: n/a, &ax.deref().v: {:p}",
            // ax.v,
            &ax.v,
            // ax.deref().v,
            &ax.deref().v,
        );
        println!(
            "Pointer values of -- ax.v[0]: n/a, &ax.v[0]: {:p}, ax.deref().v[0]: n/a, &ax.deref().v[0]: {:p}",
            // ax.v,
            &ax.v[0],
            // ax.deref().v,
            &ax.deref().v[0],
        );
        println!(
            "Pointer values of -- ax.b: {:p}, &ax.b: {:p}, ax.deref().b: {:p}, &ax.deref().b: {:p}",
            ax.b,
            &ax.b,
            ax.deref().b,
            &ax.deref().b,
        );
        println!(
            "Pointer values of -- ax.b.deref(): {:p}, &ax.b.deref(): {:p}, ax.deref().b.deref(): {:p}, &ax.deref().b.deref(): {:p}",
            ax.b.deref(),
            &ax.b.deref(),
            ax.deref().b.deref(),
            &ax.deref().b.deref(),
        );

        // Run main and look at the printed pointer values above.
        // Notice that the pointer values of &x, &ax, and &ax.deref() are on the stack,
        // while the pointer values of ax, &ax.v, and &ax.deref().v are identical and are on the heap.
        // The pointer value of &ax.b differs from the pointer value of &ax.v by 24 bytes, which is the length of
        // a vector representation (pointer to array, capacity, and length).

        let t2 = thread::spawn(move || ax);
        // let y = ax; // doesn't compile because ax was moved

        t2
    });

    let t2 = t1.join();
    let x = t2.unwrap().join().unwrap();
    println!("{:?}", x.v);
}
