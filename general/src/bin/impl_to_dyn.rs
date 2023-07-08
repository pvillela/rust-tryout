//! This file demonstrates conversion of impl to dyn and &dyn.

#![allow(unused)]

use std::ops::Deref;
trait Foo {
    fn foo(&self);
}

struct Bar(i32);

impl Foo for Bar {
    fn foo(&self) {
        println!("{}", self.0);
    }
}

fn main() {
    {
        let str = "42".to_owned();
        let f = move || str.clone();
        let f_box_impl = Box::new(f);
        let f_box_dyn: Box<dyn Fn() -> String> = f_box_impl;
        let f_ref_dyn: &dyn Fn() -> String = f_box_dyn.as_ref();
        let f_ref_dyn: &dyn Fn() -> String = &*f_box_dyn;
        let f_ref_dyn: &dyn Fn() -> String = &f_box_dyn; // works for closure only
        let f_ref_dyn_leaked: &dyn Fn() -> String = Box::leak(f_box_dyn);

        let str = "42".to_owned();
        let f = move || str.clone();
        let f_box_impl = Box::new(f);
        let f_ref_dyn_leaked: &dyn Fn() -> String = Box::leak(f_box_impl);
    }

    {
        let b = Bar(84);
        let b_box_impl = Box::new(b);
        let b_box_dyn: Box<dyn Foo> = b_box_impl;
        let b_ref_dyn: &dyn Foo = b_box_dyn.as_ref();
        let b_ref_dyn: &dyn Foo = &*b_box_dyn;
        // let b_reb_dyn: &dyn Foo = &b_box_dyn; // works for closure only
        let b_ref_dyn_leaked: &dyn Foo = Box::leak(b_box_dyn);

        let b = Bar(84);
        let b_box_impl = Box::new(b);
        let b_ref_dyn_leaked: &dyn Foo = Box::leak(b_box_impl);
    }
}
