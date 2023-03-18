//! Demonstrates use of ArcSwap and RefCell as smart pointers.

#![allow(unused)]

use arc_swap::ArcSwap;
use std::{cell::RefCell, sync::Arc};

fn main() {
    {
        let x = ArcSwap::new(Arc::new("xxx".to_owned()));
        take_ref(&x);
    }

    {
        let x = RefCell::new(Arc::new("xxx".to_owned()));
        take_ref(&x);
    }

    {
        let x = RefCell::new("xxx".to_owned());
        let y: &String = &x.borrow();
        take_ref(&x);
        take_ref(y);
    }
}

fn take_ref<T>(x: &T) {
    println!("{:p}", x);
}

fn take_arc_swap<T>(x: &ArcSwap<T>) {
    take_ref(x);
}

fn take_ref_cell<T>(x: &RefCell<T>) {
    take_ref(x);
}

fn xxx<I>(x: &ArcSwap<I>, f: fn(&I)) {
    // Below doesn;t compile.
    // f(x);
    take_ref(x)
}
