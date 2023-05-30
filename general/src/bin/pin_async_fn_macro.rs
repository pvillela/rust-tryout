use std::time::Duration;
use tokio::time::sleep;

macro_rules! pin_async_fn {
    ($f:ident) => {
        |s| Box::pin($f(s))
    };
}

async fn bar_a_bf(sleep_millis: u64) -> String {
    sleep(Duration::from_millis(sleep_millis)).await;

    "xxx".to_owned()
}

#[tokio::main]
async fn main() {
    let f = pin_async_fn!(bar_a_bf);
    let x = f(10).await;
    assert_eq!(x, "xxx".to_owned());
    println!("x={}", x);
}
