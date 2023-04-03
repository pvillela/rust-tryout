//! This example shows the anomaly that a closure returning a box-pinned async block does not
//! get properly converted to a closure returning a box-pinned future without additional code
//! that presumably does nothing but add overhead.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

// Below requires the return of the spurious additional closure at the end.
fn foo_c(
) -> impl Fn(u64) -> Pin<Box<dyn Future<Output = String> + 'static + Send + Sync>> + Send + Sync {
    let f = move |sleep_millis: u64| {
        let fut = async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            "Foo's answer is 42.".to_owned()
        };
        Box::pin(fut)
    };

    // Returning f does not compile.
    // f

    // Need to return below closure.
    move |x| f(x)
}

// However, if we don't name the closure that returns the async block, the additional closure
// is not required.
fn bar_c(
) -> impl Fn(u64) -> Pin<Box<dyn Future<Output = String> + 'static + Send + Sync>> + Send + Sync {
    move |sleep_millis: u64| {
        let fut = async move {
            sleep(Duration::from_millis(sleep_millis)).await;
            "Bar's answer is 84.".to_owned()
        };
        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() {
    {
        let foo = foo_c();
        let res = foo(1).await;
        println!("{}", res);
    }

    {
        let bar = bar_c();
        let res = bar(1).await;
        println!("{}", res);
    }
}
