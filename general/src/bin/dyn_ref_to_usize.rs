//! How to follow compiler help 'cast through a thin pointer first'.
//! See https://users.rust-lang.org/t/cast-through-a-thin-pointer-first/36311.

use std::fmt::Debug;

fn foo(x: &dyn Debug) {
    let y = x as *const dyn Debug;
    let z = y as *const () as usize;
    println!("{:?}", z);
}

fn main() {
    let x = "foo".to_owned();
    let x: &dyn Debug = &x;
    foo(x);
}
