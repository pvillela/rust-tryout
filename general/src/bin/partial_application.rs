//! Examples of partial application. See `async-borrow.rs` for a more complex scenario involving closures
//! with higher-rank trait bounds (https://doc.rust-lang.org/nomicon/hrtb.html).

use std::{future::Future, time::Duration};

/// This works for both regular and async functions.
fn partial_application<S1, S2, T>(f: fn(S1, S2) -> T, s1: S1) -> impl Fn(S2) -> T
where
    S1: Clone,
{
    move |s2| f(s1.clone(), s2)
}

/// This works only for async functinos, returns FnOnce.
fn partial_application_async<S1, S2, T, FUT>(f: fn(S1, S2) -> FUT, s1: S1) -> impl Fn(S2) -> FUT
where
    S1: Clone,
    FUT: Future<Output = T>,
{
    move |s2| f(s1.clone(), s2)
}

fn f(x: u64, y: u64) -> u64 {
    x + y
}

async fn f_a(x: u64, y: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(x + y)).await;
    x + y
}

async fn f_a_r1(x: &u64, y: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(x + y)).await;
    x + y
}

async fn f_a_r2(x: u64, y: &u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(x + y)).await;
    x + y
}

#[tokio::main]
async fn main() {
    let f_part = partial_application(f, 20);
    _ = f_part(2);
    let res = f_part(2);
    println!("{res}");

    let f_part = partial_application(f_a, 40);
    _ = f_part(2).await;
    let res = f_part(2).await;
    println!("{res}");

    let f_part = partial_application(f_a_r1, &60);
    _ = f_part(2).await;
    let res = f_part(2).await;
    println!("{res}");

    let f_part = partial_application(f_a_r2, 60);
    _ = f_part(&2).await;
    let res = f_part(&2).await;
    println!("{res}");

    let f_part = partial_application_async(f_a, 60);
    let res = f_part(2).await;
    println!("{res}");

    let f_part = partial_application_async(f_a_r1, &60);
    let res = f_part(2).await;
    println!("{res}");

    let f_part = partial_application_async(f_a_r2, 60);
    let res = f_part(&2).await;
    println!("{res}");
}
