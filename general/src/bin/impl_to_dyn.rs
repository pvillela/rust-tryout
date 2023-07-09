//! This file demonstrates conversion of impl to dyn and &dyn, as well as the conversion of
//! Box<&dyn> to Box<dyn> for closures.

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

        // Convert a closure's leaked dyn reference back to a Box<dyn>.
        let f_box_dyn: Box<dyn Fn() -> String> = Box::new(f_ref_dyn_leaked);

        // Convert the inner reference to a Box of a closure back to a Box<dyn>.
        let str = "42".to_owned();
        let f = move || str.clone();
        let f_box = Box::new(f);
        let f_box_dyn: Box<dyn Fn() -> String> = Box::new(f_box.as_ref());

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

        // Unlike closures, can't convert a Foo leaked dyn reference to a Box<dyn>.
        // let b_box_dyn: Box<dyn Foo> = Box::new(b_ref_dyn_leaked);

        // Unlike closures, can't convert the inner reference to a Box<Foo> back to a Box<dyn Foo>.
        let b = Bar(84);
        let b_box = Box::new(b);
        // let b_box_dyn: Box<dyn Foo> = Box::new(b_box.as_ref()); // doesn't compile

        let b = Bar(84);
        let b_box_impl = Box::new(b);
        let b_ref_dyn_leaked: &dyn Foo = Box::leak(b_box_impl);
    }
}
