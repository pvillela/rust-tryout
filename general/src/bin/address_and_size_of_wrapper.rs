//! Demonstrates that the address and size of a wrapper struct are the same as those of the wrapped
//! struct. Thus, wrapping has zero cost.

use std::mem::size_of;

#[allow(unused)]
struct Foo {
    x: i32,
    y: u64,
}

struct Bar(Foo);

fn main() {
    let bar = Bar(Foo { x: 1, y: 2 });
    println!("address of bar: {:p}, address of bar.0: {:p}", &bar, &bar.0);

    let bar_addr = {
        let ptr: *const Bar = &bar;
        ptr as usize;
    };
    let bar_0_addr = {
        let ptr: *const Foo = &bar.0;
        ptr as usize;
    };
    assert_eq!(bar_addr, bar_0_addr, "address");

    let foo_size = size_of::<Foo>();
    let bar_size = size_of::<Bar>();
    assert_eq!(foo_size, bar_size, "size");
}
