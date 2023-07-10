//! This file demonstrates two things:
//!
//! 1. the creation of a higher order function accepting an async function that
//!    itself takes a reference argument.
//!
//!    See https://github.com/rust-lang/rust/issues/113495
//!    and https://github.com/rust-lang/rust/issues/113495#issuecomment-1627893701.
//!
//! 2. How a polymorphic function can be passed to a higher-order function that accepts
//!    a function whose arguments' types are compatible with those of the polymorphic
//!    function.

use core::future::Future;
use std::fmt::Debug;

#[tokio::main]
async fn main() {
    higher_order_tx(f_tx).await;
    higher_order(f_poly, 84).await;
}

//==========
// Example 1

trait Tx {
    fn show(&self);
}

impl Tx for u32 {
    fn show(&self) {
        println!("Tx for u32 value = {self}");
    }
}

async fn f_tx(input: &dyn Tx) {
    println!("f_tx executed");
    input.show();
}

/// The lifetime parameter `'a` is criticallu important. Without it, the above call in main generates
/// a compilation error.
async fn higher_order_tx<'a, Fut>(f: fn(&'a dyn Tx) -> Fut)
where
    Fut: Future<Output = ()>,
{
    f(&42u32).await;
}

//==========
// Example 2

async fn higher_order<Fut>(f: impl Fn(u32) -> Fut, input: u32)
where
    Fut: Future<Output = ()>,
{
    f(input).await;
}

async fn f_poly(x: impl Debug) {
    println!("f_poly executed");
    println!("Poly value = {:?}", x);
}
