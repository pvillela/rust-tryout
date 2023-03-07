//! Based on https://docs.rs/arc-swap/latest/arc_swap/cache/struct.Cache.html

#![allow(unused)]

use arc_swap::cache::Access;
use arc_swap::{ArcSwap, Cache};
use lazy_static::__Deref;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[allow(unused)]
#[derive(Debug)]
struct Config {
    x: String,
    y: u32,
    z: u32,
}

#[derive(Debug)]
struct AppCfgInfo {
    y: u32,
    z: u32,
}

static CURRENT_CONFIG: Lazy<ArcSwap<Config>> = Lazy::new(|| {
    ArcSwap::from_pointee(Config {
        x: "foo".to_string(),
        y: 42,
        z: 99,
    })
});

thread_local! {
    // * RefCell needed, because load on cache is `&mut`.
    // * You want to operate inside the `with` ‒ cloning the Arc is comparably expensive as
    //   ArcSwap::load itself and whatever you'd save by the cache would be lost on that.
    static CACHE: RefCell<Cache<&'static ArcSwap<Config>, Arc<Config>>> = RefCell::new(Cache::from(CURRENT_CONFIG.deref()));
}

struct InnerCfg {
    answer: usize,
}

struct FullCfg {
    inner: InnerCfg,
}

fn use_inner<A: Access<InnerCfg>>(cache: &mut A) {
    let value = cache.load();
    println!("The answer is: {}", value.answer);
}

fn example() {
    let full_cfg = ArcSwap::from_pointee(FullCfg {
        inner: InnerCfg { answer: 42 },
    });
    let cache = Cache::new(&full_cfg);
    use_inner(&mut cache.map(|full| &full.inner));

    let inner_cfg = ArcSwap::from_pointee(InnerCfg { answer: 24 });
    let mut inner_cache = Cache::new(&inner_cfg);
    use_inner(&mut inner_cache);
}

fn main() {
    {
        CACHE.with(|c| {
            println!("{:?}", c.borrow_mut().load());
        });
        {}
        let x = CACHE.with(|c| c.borrow_mut().load().x.clone());
        println!("{}", x);
    }

    {
        let cache = Cache::new(CURRENT_CONFIG.deref());
        let mut mapped_cache = cache.map(|full| &full.y);
        let val = mapped_cache.load();
        println!("val1={}", val);
    }

    {
        // What's the difference between Cache::new and Cache::from? I can't see it.
        let cache = Cache::from(CURRENT_CONFIG.deref());
        let mut mapped_cache = cache.map(|full| &full.y);
        let val = mapped_cache.load();
        println!("val1={}", val);
    }

    // Below doesn't compile.
    // {
    //     let cache = Cache::new(CURRENT_CONFIG.deref());
    //     let mut mapped_cache = cache.map(|full| {
    //         Rc::new(AppCfgInfo {
    //             y: full.y,
    //             z: full.z,
    //         })
    //     });
    //     let val = mapped_cache.load();
    //     println!("val1={:?}", val);
    // }
}
