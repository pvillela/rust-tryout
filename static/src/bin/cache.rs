//! Based on https://docs.rs/arc-swap/latest/arc_swap/cache/struct.Cache.html

use arc_swap::{ArcSwap, Cache};
use lazy_static::__Deref;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::sync::Arc;

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

fn main() {
    CACHE.with(|c| {
        println!("{:?}", c.borrow_mut().load());
    });

    let foo = CACHE.with(|c| c.borrow_mut().load().foo.clone());
    println!("{}", foo);
}
