//! This example demonstrates the creation of a higher order function accepting an async function that
//! itself takes a reference argument.
//! See https://github.com/rust-lang/rust/issues/113495
//! and https://github.com/rust-lang/rust/issues/113495#issuecomment-1627893701.

use core::future::Future;

#[tokio::main]
async fn main() {
    _ = higher_order_tx(f_tx).await;
}

trait Tx {
    fn show(&self);
}

impl Tx for u32 {
    fn show(&self) {
        println!("Value = {self}");
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
