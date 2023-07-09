//! Demonstrates higher-order functions that takes an async fn as an argument.
//! See https://github.com/rust-lang/rust/issues/113495
//! and https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952.

use std::future::Future;

struct X;

trait Tx {}

fn main() {
    higher_order(f_x);
    higher_order_x(f_x);

    higher_order(f_tx);
    // higher_order_tx(f_tx); // does not compile

    higher_order_tx_nf(f_tx_nf);

    higher_order_tx_corrected(f_tx);
}

fn higher_order<I, FUT>(_f: fn(I) -> FUT)
where
    FUT: Future<Output = ()>,
{
}

fn higher_order_x<FUT>(_f: fn(X) -> FUT)
where
    FUT: Future<Output = ()>,
{
}

/// Doesn't work, see [higher_order_tx_corrected] below.
#[allow(unused)]
fn higher_order_tx<FUT>(_f: fn(&dyn Tx) -> FUT)
where
    FUT: Future<Output = ()>,
{
}

fn higher_order_tx_nf(_f: fn(&dyn Tx) -> ()) {}

async fn f_x(_input: X) {}

async fn f_tx(_input: &dyn Tx) {}

fn f_tx_nf(_input: &dyn Tx) {}

////
// Fix to type issue, presented in https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952

trait AsyncBorrowFn<'a, A: ?Sized + 'a>: Fn(&'a A) -> Self::Fut {
    type Out;
    type Fut: Future<Output = Self::Out> + 'a;
}

impl<'a, A, F, Fut> AsyncBorrowFn<'a, A> for F
where
    A: ?Sized + 'a,
    F: Fn(&'a A) -> Fut,
    Fut: Future + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

impl Tx for u32 {}

fn higher_order_tx_corrected(_f: impl for<'a> AsyncBorrowFn<'a, dyn Tx + 'a, Out = ()>) {}
