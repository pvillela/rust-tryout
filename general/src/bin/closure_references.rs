#![allow(unused)]

use std::{marker::PhantomData, ops::Deref, sync::Arc, thread};

fn f() -> String {
    "me".to_owned()
}

fn main() {
    let x = FooRef::new(f);
    let y = x.src;
    println!("{}", y());
}

fn fref(f: &'static (dyn Fn() -> String + Send + Sync)) {
    let handle = thread::spawn(|| f());
    handle.join();

    let handle = thread::spawn(|| f());
    handle.join();
}

fn fbox(f: Box<dyn Fn() -> String + Send + Sync>) {
    let handle = thread::spawn(move || f());
    handle.join();

    // Below doesn't compile because f is consumed by above.
    // let handle = thread::spawn(|| f());
    // handle.join();
}

fn farc(f: Arc<dyn Fn() -> String + Send + Sync>) {
    let f1 = f.clone();
    let handle = thread::spawn(move || f());
    handle.join();

    let handle = thread::spawn(move || f1());
    handle.join();
}

fn fgeneral(f: impl Deref<Target = dyn Fn() -> String> + Send + Sync + Clone + 'static) {
    let f1 = f.clone();
    let handle = thread::spawn(move || f1());
    handle.join();

    let handle = thread::spawn(move || f());
    handle.join();
}

// fn fgeneral_fooref(x: FooRef<String>) {
//     let y = x.src.as_ref();
//     fgeneral(x.src.as_ref());
// }

struct Foo<T: 'static, F>
where
    F: Deref<Target = dyn Fn() -> T + Send + Sync + 'static>,
{
    src: F,
    _t: PhantomData<T>,
}

type FooRef<T> = Foo<T, &'static (dyn Fn() -> T + Send + Sync + 'static)>;
impl<T: 'static> FooRef<T> {
    fn foo() {}

    fn new(f: fn() -> T) -> Self {
        Foo {
            src: Box::leak(Box::new(move || f())),
            _t: PhantomData,
        }
    }
}

type FooBox<T> = Foo<T, Box<dyn Fn() -> T + Send + Sync + 'static>>;
impl<T: 'static> FooBox<T> {
    fn foo() {}

    fn new(f: fn() -> T) -> Self {
        Foo {
            src: Box::new(move || f()),
            _t: PhantomData,
        }
    }
}

type FooArc<T> = Foo<T, Arc<dyn Fn() -> T + Send + Sync + 'static>>;
impl<T: 'static> FooArc<T> {
    fn foo() {}

    fn new(f: fn() -> T) -> Self {
        Foo {
            src: Arc::new(move || f()),
            _t: PhantomData,
        }
    }
}

// fn fref_fooref(x: FooRef<String>) {
//     let y = x.src.as_ref();
//     println!("{}", y());
//     fref(x.src.as_ref());
//     // fref(x.src);
// }

// Below doesn't compile.
// fn fref_foobox(x: FooBox<String>) {
//     let y = x.src.as_ref();
//     println!("{}", y());
//     fref(y);
// }

// Below doesn't compile.
// fn fref_fooarc(x: FooArc<String>) {
//     let y = x.src.as_ref();
//     println!("{}", y());
//     fref(y);
// }
