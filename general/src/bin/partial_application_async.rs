use std::{future::Future, time::Duration};

fn partial_application_async<S1, S2, T, FUT>(f: fn(S1, S2) -> FUT, s1: S1) -> impl FnOnce(S2) -> FUT
where
    FUT: Future<Output = T>,
{
    move |s2| f(s1, s2)
}

async fn f(x: u64, y: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(x + y)).await;
    x + y
}

#[tokio::main]
async fn main() {
    let f_part = partial_application_async(f, 40);
    let res = f_part(2).await;
    println!("{res}");
}
