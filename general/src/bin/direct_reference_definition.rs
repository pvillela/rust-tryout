//! This example shows that when a variable is directly defined as a reference then the underlying value
//! is dropped when the reference goes out of scope. Calling [`Drop::drop`] on a reference has no effect
//! and produces a warning.

#[derive(Debug)]
struct Foo(u32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropped {self:?}");
    }
}

fn main() {
    println!("Entered main");

    let foo1 = &Foo(1);

    {
        let foo2 = &Foo(2);

        println!("{foo1:?}");
        println!("{foo2:?}");
    }

    #[allow(dropping_references)]
    drop(foo1);

    println!("{foo1:?}");

    let foo3 = &Foo(3);
    println!("{foo3:?}");

    println!("Exiting main");
}
