//! The trait defined here was recommended by https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952
//! in response to my issue https://github.com/rust-lang/rust/issues/113495.

use std::future::Future;
use std::pin::Pin;

/// Represents an async function with single argument that is a reference.
pub trait AsyncBorrowFn1<'a, A: ?Sized + 'a>: Fn(&'a A) -> Self::Fut + Send + Sync {
    type Out;
    type Fut: Future<Output = Self::Out> + Send + Sync + 'a;
}

impl<'a, A, F, Fut> AsyncBorrowFn1<'a, A> for F
where
    A: ?Sized + 'a,
    F: Fn(&'a A) -> Fut + Send + Sync + 'a,
    Fut: Future + Send + Sync + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Represents an async function with 2 arguments; the first is not a reference, the last is a reference.
pub trait AsyncBorrowFn01<'a, A1, A2: ?Sized + 'a>:
    Fn(A1, &'a A2) -> Self::Fut + Send + Sync
{
    type Out;
    type Fut: Future<Output = Self::Out> + Send + Sync + 'a;
}

impl<'a, A1, A2, F, Fut> AsyncBorrowFn01<'a, A1, A2> for F
where
    A2: ?Sized + 'a,
    F: Fn(A1, &'a A2) -> Fut + Send + Sync + 'a,
    Fut: Future + Send + Sync + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Represents an async function with 3 arguments; the first 2 are not references, the last is a reference.
pub trait AsyncBorrowFn001<'a, A1, A2, A3: ?Sized + 'a>:
    Fn(A1, A2, &'a A3) -> Self::Fut + Send + Sync
{
    type Out;
    type Fut: Future<Output = Self::Out> + Send + Sync + 'a;
}

impl<'a, A1, A2, A3, F, Fut> AsyncBorrowFn001<'a, A1, A2, A3> for F
where
    A3: ?Sized + 'a,
    F: Fn(A1, A2, &'a A3) -> Fut + Send + Sync + 'a,
    Fut: Future + Send + Sync + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

//=================
// Examples

trait Tx {}
impl Tx for u32 {}

async fn higher_order_tx(f: impl for<'a> AsyncBorrowFn1<'a, dyn Tx + Send + Sync + 'a, Out = ()>) {
    f(&12u32).await;
}

async fn f_tx(_input: &(dyn Tx + Send + Sync)) {}

fn higher_order_tx2(
    f: impl for<'a> AsyncBorrowFn01<'a, u32, dyn Tx + Send + Sync + 'a, Out = u32>,
    i: u32,
) -> impl for<'a> Fn(&'a (dyn Tx + Send + Sync)) -> Pin<Box<dyn Future<Output = u32> + Send + Sync + 'a>>
{
    move |x: &(dyn Tx + Send + Sync)| {
        let y = f(i, x);
        Box::pin(y)
    }
}

async fn f_tx2(_i: u32, _input: &(dyn Tx + Send + Sync)) -> u32 {
    42
}

fn main() {
    _ = higher_order_tx(f_tx);
    _ = higher_order_tx2(f_tx2, 1);
}
