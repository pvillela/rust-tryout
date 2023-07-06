//! This file demonstrates conversion of impl to dyn.

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
    let s = "42".to_owned();
    let s_lr: &String = &Box::new(s);

    let str = "84".to_owned();
    let f = move || str.clone();
    let f_ib: Box<dyn Fn() -> String> = Box::new(f); // conversion from Box<impl> to Box<dyn>
    let f_lr: &dyn Fn() -> String = &f_ib;

    let b = Bar(42);
    let b_ib: Box<dyn Foo> = Box::new(b); // conversion from Box<impl> to Box<dyn>
    let b_lr: &dyn Foo = b_ib.as_ref(); // &b_ib doesn't compile

    // Attempt to convert Box<&dyn> to Box<dyn> doesn't compile
    // let b_lrb: Box<dyn Foo> = Box::new(b_lr); // doesn't compile

    println!("{}", s_lr);
    println!("{}", f_lr());

    {
        let s_lrb: Box<&String> = Box::new(s_lr);
        println!("{}", s_lrb);

        let f_lrb: Box<&dyn Fn() -> String> = Box::new(f_lr);
        // Conversion from Box<&dyn> to Box<dyn> works for closures but not other traits
        let f_lrb1: Box<dyn Fn() -> String> = Box::new(f_lr);
        let f_lrb2: Box<dyn Fn() -> String> = f_lrb;
        // Conversion in the other direction doesn't work
        // let f_lrb3: Box<&dyn Fn() -> String> = f_lrb2; //doesn't compile
        println!("{}", f_lrb1());
    }

    println!("{}", s_lr);
    println!("{}", f_lr());
}
