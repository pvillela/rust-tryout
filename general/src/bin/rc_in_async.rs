use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

async fn foo(x: String) {
    let y = Rc::new(x);
    let z = (*y.as_ref()).clone();
    drop(y);

    sleep(Duration::from_millis(0)).await;
    println!("z={}", z);
}

async fn foo_with_workaround(x: String) {
    let z = {
        let y = Rc::new(x);
        let z = (*y.as_ref()).clone();
        z
    };
    sleep(Duration::from_millis(0)).await;
    println!("z={}", z);
}

#[tokio::main]
async fn main() {
    foo("foo".to_owned()).await;

    // Below doesn't compile due to Rust compiler bug.
    // let handle = tokio::spawn(async move {
    //     foo("bar".to_owned()).await;
    // });
    // let _ = handle.await;

    let handle = tokio::spawn(async move {
        foo_with_workaround("bar".to_owned()).await;
    });
    let _ = handle.await;
}
