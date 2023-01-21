//! This is a modification of with_pin.rs, with most function parameters changed from passsing
//! references of Pins of mutable references to references of Pins of immutable references.

use std::marker::PhantomPinned;
use std::ops::Deref;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(text: &str) -> Self {
        Test {
            a: String::from(text),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: &'a Pin<&'a Self>) -> &str {
        &self.a
    }

    fn b(self: &Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }

    fn bp(self: &Pin<&Self>) -> *const String {
        self.b
    }

    fn selfp(self: &Pin<&Self>) -> *const Test {
        self.deref()
    }
}

fn main() {
    let mut var1 = Test::new("test1");
    println!("address of var1: {:p}", &var1);
    let mut pin1 = unsafe { Pin::new_unchecked(&mut var1) };
    Test::init(pin1.as_mut());
    let mut pin1: Pin<&Test> = pin1.as_ref();

    let mut var2 = Test::new("test2");
    println!("address of var2: {:p}", &var2);
    let mut pin2 = unsafe { Pin::new_unchecked(&mut var2) };
    Test::init(pin2.as_mut());
    let mut pin2: Pin<&Test> = pin2.as_ref();

    println!("-- before swap");

    println!(
        "address of pin1: {:p}, pin1.get_ref(): {:p}, pin1.a(): {}, pin1.b(): {}, pin1.bp(): {:?}",
        &pin1,
        Test::selfp(&pin1),
        Test::a(&pin1),
        Test::b(&pin1),
        Test::bp(&pin1),
    );

    println!(
        "address of pin2: {:p}, pin2.get_ref(): {:p}, pin2.a(): {}, pin2.b(): {}, pin2.bp(): {:?}",
        &pin2,
        Test::selfp(&pin2),
        Test::a(&pin2),
        Test::b(&pin2),
        Test::bp(&pin2),
    );

    // Line below, from original example in https://cfsamson.github.io/books-futures-explained/5_pin.html.
    // It tries to swap the targets of pin1 and pin2, and does not compile.
    // std::mem::swap(&mut pin1.get_mut(), &mut pin2.get_mut());

    // The line below, replacing the above commented-out line, compiles fine.
    // It swaps the Pin containers instead of the wrapped target values.
    // After the swap, pin1's and pin2's addresses remain unchanges but their contents get swapped:
    std::mem::swap(&mut pin1, &mut pin2);
    // pin1 now points to var2 and pin2 now points to var1.

    println!("-- after swap");

    println!(
        "address of pin1: {:p}, pin1.get_ref(): {:p}, pin1.a(): {}, pin1.b(): {}, pin1.bp(): {:?}",
        &pin1,
        Test::selfp(&pin1),
        Test::a(&pin1),
        Test::b(&pin1),
        Test::bp(&pin1),
    );

    println!(
        "address of pin2: {:p}, pin2.get_ref(): {:p}, pin2.a(): {}, pin2.b(): {}, pin2.bp(): {:?}",
        &pin2,
        Test::selfp(&pin2),
        Test::a(&pin2),
        Test::b(&pin2),
        Test::bp(&pin2),
    );
}
