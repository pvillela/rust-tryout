//! Modified generator.rs to introduce parameter for GeneratorA::start.

use std::marker::PhantomPinned;
use std::pin::Pin;

pub fn main() {
    let gen1 = GeneratorA::start("Gen1");
    let gen2 = GeneratorA::start("Generator2");
    // Before we pin the data, this is safe to do
    // std::mem::swap(&mut gen, &mut gen2);

    // constructing a `Pin::new()` on a type which does not implement `Unpin` is
    // unsafe. An object pinned to heap can be constructed while staying in safe
    // Rust so we can use that to avoid unsafe. You can also use crates like
    // `pin_utils` to pin to the stack safely, just remember that they use
    // unsafe under the hood so it's like using an already-reviewed unsafe
    // implementation.

    let mut pinned1 = Box::pin(gen1);
    let mut pinned2 = Box::pin(gen2);

    // Uncomment these if you think it's safe to pin the values to the stack instead
    // (it is in this case). Remember to comment out the two previous lines first.
    //let mut pinned1 = unsafe { Pin::new_unchecked(&mut gen1) };
    //let mut pinned2 = unsafe { Pin::new_unchecked(&mut gen2) };

    if let GeneratorState::Yielded(n) = pinned1.as_mut().resume() {
        println!("Result from pinned1.as_mut().resume(): {}", n);
    }

    if let GeneratorState::Yielded(n) = pinned2.as_mut().resume() {
        println!("Result from pinned2.as_mut().resume(): {}", n);
    };

    println!("pinned1.as_mut().resume():");
    let _ = pinned1.as_mut().resume();
    println!("pinned2.as_mut().resume():");
    let _ = pinned2.as_mut().resume();
}

enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

trait Generator {
    type Yield;
    type Return;
    fn resume(self: Pin<&mut Self>) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorA<'a> {
    Enter(&'a str),
    Yield1 {
        to_borrow: String,
        borrowed: *const String,
    },
    Exit,
    _Phantom(PhantomPinned),
}

impl<'a> GeneratorA<'a> {
    fn start(s: &'a str) -> Self {
        GeneratorA::Enter(s)
    }
}

impl<'a> Generator for GeneratorA<'a> {
    type Yield = usize;
    type Return = ();
    fn resume(self: Pin<&mut Self>) -> GeneratorState<Self::Yield, Self::Return> {
        // lets us get ownership over current state
        let this = unsafe { self.get_unchecked_mut() };
        match this {
            GeneratorA::Enter(s) => {
                let to_borrow = s.to_owned();
                let borrowed = &to_borrow;
                let res = borrowed.len();
                *this = GeneratorA::Yield1 {
                    to_borrow,
                    borrowed: std::ptr::null(),
                };

                // Trick to actually get a self reference. We can't reference
                // the `String` earlier since these references will point to the
                // location in this stack frame which will not be valid anymore
                // when this function returns.
                if let GeneratorA::Yield1 {
                    to_borrow,
                    borrowed,
                } = this
                {
                    *borrowed = to_borrow;
                }

                GeneratorState::Yielded(res)
            }

            GeneratorA::Yield1 {
                to_borrow,
                borrowed,
            } => {
                let borrowed: &String = unsafe { &**borrowed };
                println!("{} world (to_borrow={})", borrowed, to_borrow);
                *this = GeneratorA::Exit;
                GeneratorState::Complete(())
            }

            GeneratorA::Exit => panic!("Can't advance an exited generator!"),

            GeneratorA::_Phantom(_) => panic!("Unreachable code."),
        }
    }
}
