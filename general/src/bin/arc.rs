use std::{ops::Deref, sync::Arc, thread};

struct Foo {
    s: i32,
    b: Box<i32>,
}

// This example shows that Arc::new() copies data to the heap.
// The original data x is created on the stack of thread t1, then it is owned by
// an Arc that is passed to thread t2, and finally the Arc is accessed by the
// main thread when it joins t2.
fn main() {
    let x = Foo {
        s: 1,
        b: Box::new(42),
    };
    println!("Pointer value of &x: {:p} in thread main", &x);

    let t1 = thread::spawn(move || {
        println!("Pointer value of &x: {:p} in thread t1", &x);

        let ax = Arc::new(x);
        // let y = x; // doesn't compile because x was moved

        arc_ptr_dump(ax.clone(), "ax.clone() in thread t1");
        arc_ptr_dump1(&ax, "ax ...... in thread t1");

        println!(
            "Pointer values of -- ax: {:p}, &ax: {:p}, *ax: n/a, &*ax: {:p}, ax.deref(): {:p}, &ax.deref(): {:p}",
            ax,
            &ax,
            // *ax,
            &*ax,
            ax.deref(),
            &ax.deref(),
        );
        println!(
            "Pointer values of -- ax.s: n/a, &ax.s: {:p}, ax.deref().s: n/a, &ax.deref().s: {:p}",
            // ax.s,
            &ax.s,
            // ax.deref().s,
            &ax.deref().s,
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
        // while the pointer values of ax, &ax.b, and &ax.deref().b are identical and on the heap.
        // The pointer value of &ax.b differs from the pointer value of &ax.s by 8 bytes, which is the size of
        // Foo.b. The compiler has reordered Foo.s and Foo.b in memory.

        let t2 = thread::spawn(move || ax);
        // let y = ax; // doesn't compile because ax was moved

        t2
    });

    let t2 = t1.join();
    let ax = t2.unwrap().join().unwrap();
    arc_ptr_dump(ax.clone(), "ax.clone() in thread main");
    arc_ptr_dump1(&ax, "ax ...... in thread main");
    println!("{:?}", ax.s);
}

fn arc_target_address<T>(rarc: &Arc<T>) -> usize {
    Arc::as_ptr(rarc) as usize
}

fn arc_ptr_dump<T>(a: Arc<T>, name: &str) {
    let ra: &Arc<T> = &a;
    let rt: &T = &a;
    let da: &T = a.deref();
    let ap: usize = arc_target_address(&a);
    println!(
        "arc_ptr_dump for {name} -- a: {:p}, &a: {:p}, ra: {:p}, rt: {:p}, da: {:p}, ap: {:x}",
        a, &a, ra, rt, da, ap
    );
}

fn arc_ptr_dump1<T>(ra: &Arc<T>, name: &str) {
    // let ra: &Arc<T> = a1;
    let a = ra.clone();
    let rt: &T = &ra;
    let da: &T = ra;
    let ap: usize = arc_target_address(ra);
    println!(
        "arc_ptr_dump1 for {name} -- a: {:p}, &a: {:p}, ra: {:p}, rt: {:p}, da: {:p}, ap: {:x}",
        a, &a, ra, rt, da, ap
    );
}
