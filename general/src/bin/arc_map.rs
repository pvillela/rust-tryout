//! Demonstrates the creation of a smart pointer that returns a reference to the
//! mapped value of an Arc.
//! There are implementations for AsRef and Deref. The Deref variant is more egonomic.

use std::{ops::Deref, sync::Arc};

struct ArcMap<T, TX> {
    value: Arc<T>,
    xform: fn(&T) -> &TX,
}

impl<T, TX> ArcMap<T, TX> {
    fn new(value: Arc<T>, xform: fn(&T) -> &TX) -> Self {
        ArcMap { value, xform }
    }
}

impl<T, TX> AsRef<TX> for ArcMap<T, TX> {
    fn as_ref(&self) -> &TX {
        (self.xform)(&self.value)
    }
}

impl<T, TX> Deref for ArcMap<T, TX> {
    type Target = TX;
    fn deref(&self) -> &Self::Target {
        (self.xform)(&self.value)
    }
}

#[allow(unused)]
#[derive(Debug)]
struct Bar {
    x: String,
}

struct Foo {
    bar: Bar,
}

fn xform(foo: &Foo) -> &Bar {
    println!("xform executed");
    &foo.bar
}

fn take_bar_ref(y: &Bar) {
    println!("{:?}", y);
}

fn main() {
    // Using AsRef.
    {
        let value = Arc::new(Foo {
            bar: Bar {
                x: "bar AsRef example".to_owned(),
            },
        });
        let mapped = ArcMap::new(value, xform);
        let r = mapped.as_ref();
        take_bar_ref(r);

        // Below doesn't compile.
        // drop(mapped);
        // take_bar_ref(r);
    }

    //Using Deref.
    {
        let value = Arc::new(Foo {
            bar: Bar {
                x: "bar Deref example".to_owned(),
            },
        });
        let mapped = ArcMap::new(value, xform);
        let r: &Bar = &mapped;
        take_bar_ref(r);

        // Below doesn't compile.
        // drop(mapped);
        // take_bar_ref(r);
    }
}
