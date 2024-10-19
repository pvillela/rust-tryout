//! Demonstrates how to use the default `random()` function as well as how to define a simple random distribution
//! for a custom type.

use rand::{prelude::Distribution, random, thread_rng};

#[derive(Debug)]
#[allow(unused)]
struct Foo {
    x: u32,
}

struct DistrFoo;

impl Distribution<Foo> for DistrFoo {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Foo {
        Foo { x: rng.next_u32() }
    }
}

fn main() {
    for _ in 0..3 {
        let rnd = random::<u16>();
        println!("random i16: {rnd}");
    }

    let mut rng = thread_rng();

    for _ in 0..3 {
        let rnd = DistrFoo.sample(&mut rng);
        println!("random Foo: {rnd:?}")
    }
}
