//! This submodule demonstrates the ***discriminated inheritance*** pattern (see [parent module](super)).
//!
//! This involves using a trait (the discriminator) that is taken as a struct
//! parameter and having different `impls` of the struct for different implementations of the discriminator trait.

use std::marker::PhantomData;

pub trait Discr {}

pub struct Foo<D: Discr> {
    value: i32,
    _d: PhantomData<D>,
}

impl<D: Discr> Foo<D> {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            _d: PhantomData,
        }
    }

    pub fn square_value(&self) -> i32 {
        self.value * self.value
    }
}

pub struct Discr1;

impl Discr for Discr1 {}

pub struct Discr2;

impl Discr for Discr2 {}

impl Foo<Discr1> {
    pub fn f(&self) -> i32 {
        self.square_value() * 10
    }

    pub fn g(&self) {
        println!("g-{}", self.f())
    }
}

impl Foo<Discr2> {
    pub fn f(&self) -> String {
        self.square_value().to_string()
    }

    pub fn h(&self) {
        println!("h-{}", self.f())
    }
}

pub fn discriminated_main() {
    println!("discriminated_main");

    let foo1: Foo<Discr1> = Foo::new(5);
    foo1.g();

    let foo2: Foo<Discr2> = Foo::new(5);
    foo2.h();
}
