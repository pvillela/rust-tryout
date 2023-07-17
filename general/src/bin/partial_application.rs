//! Examples of partial application. See `async-borrow.rs` for a more complex scenario involving closures
//! with higher-rank trait bounds (https://doc.rust-lang.org/nomicon/hrtb.html).

use std::{future::Future, time::Duration};

/// This works for both regular and async functions, returns FnOnce.
fn partial_application<S1, S2, T>(f: fn(S1, S2) -> T, s1: S1) -> impl FnOnce(S2) -> T {
    move |s2| f(s1, s2)
}

/// This works for both regular and async functions, returns Fn.
fn partial_application_r1<'a, S1, S2: 'static, T: 'static>(
    f: fn(&'a S1, S2) -> T,
    s1: &'a S1,
) -> impl Fn(S2) -> T + 'a {
    move |s2| f(s1, s2)
}

fn partial_application_r2<'a, S1: 'static + Clone, S2: 'static, T: 'static>(
    f: impl Fn(S1, &'a S2) -> T + 'a,
    s1: S1,
) -> impl Fn(&'a S2) -> T + 'a {
    move |s2| f(s1.clone(), s2)
}

/// This works only for async functinos, returns FnOnce.
fn partial_application_async<S1, S2, T, FUT>(f: fn(S1, S2) -> FUT, s1: S1) -> impl FnOnce(S2) -> FUT
where
    FUT: Future<Output = T>,
{
    move |s2| f(s1, s2)
}

fn f(x: u64, y: u64) -> u64 {
    x + y
}

fn f_r(x: &u64, y: u64) -> u64 {
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
    let res = f_part(2);
    println!("{res}");

    let f_part = partial_application_r1(f_r, &20);
    let res = f_part(2);
    println!("{res}");

    let f_part = partial_application(f_a, 40);
    let res = f_part(2).await;
    println!("{res}");

    let f_part = partial_application_r1(f_a_r1, &40);
    let res = f_part(2).await;
    println!("{res}");
    let res = f_part(3).await;
    println!("{res}");

    let f_part = partial_application_r2(f_a_r2, 40);
    let res = f_part(&2).await;
    println!("{res}");
    let res = f_part(&3).await;
    println!("{res}");

    let f_part = partial_application_async(f_a, 60);
    let res = f_part(2).await;
    println!("{res}");
}
