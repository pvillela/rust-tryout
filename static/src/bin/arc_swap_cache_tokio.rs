//! Based on https://docs.rs/arc-swap/latest/arc_swap/cache/struct.Cache.html

use arc_swap::{ArcSwap, Cache};
use lazy_static::__Deref;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::time::sleep;

#[allow(unused)]
#[derive(Debug)]
struct Config {
    foo: String,
}

static CURRENT_CONFIG: Lazy<ArcSwap<Config>> = Lazy::new(|| {
    ArcSwap::from_pointee(Config {
        foo: "foo".to_string(),
    })
});

thread_local! {
    // * RefCell needed, because load on cache is `&mut`.
    // * You want to operate inside the `with` ‒ cloning the Arc is comparably expensive as
    //   ArcSwap::load itself and whatever you'd save by the cache would be lost on that.
    static CACHE: RefCell<Cache<&'static ArcSwap<Config>, Arc<Config>>> = RefCell::new(Cache::from(CURRENT_CONFIG.deref()));
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async move {
        sleep(Duration::from_millis(0)).await;

        let x = CACHE.with(|c| {
            // Below doesn't compile.
            // sleep(Duration::from_millis(0)).await;

            let x = c.borrow_mut().load().foo.clone();
            Rc::new(x.clone())
        });
        println!("Rc(x)={}", x);

        let y = CACHE.with(|c| c.borrow_mut().load().foo.clone());
        y
    });

    // Below doesn't compile.
    // let handle2 = tokio::spawn(async move {
    //     let x = CACHE.with(|c| {
    //         // Below doesn't compile.
    //         // sleep(Duration::from_millis(0)).await;

    //         let x = c.borrow_mut().load().foo.clone();
    //         Rc::new(x.clone())
    //     });
    //     println!("{}", x);
    //     x
    // });

    let res = handle.await;
    println!("res={:?}", res);
}
