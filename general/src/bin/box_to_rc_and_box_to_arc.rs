//! Illustrate conversion of Box to Rc and Arc. Conversions in the other direction are not possible.
//! Conversions between Rc and Arc are not possible.

use std::{rc::Rc, sync::Arc};

#[derive(Debug)]
struct Foo(u32);

fn main() {
    {
        let b: Box<Foo> = Box::new(Foo(42));
        let r: Rc<Foo> = Rc::from(b);
        println!("{:?}", r);
    }

    {
        let b: Box<Foo> = Box::new(Foo(84));
        let a: Arc<Foo> = Arc::from(b);
        println!("{:?}", a);
    }
}
