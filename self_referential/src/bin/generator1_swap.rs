// Based on the last code example in https://cfsamson.github.io/books-futures-explained/5_pin.html.
// This example modifies generator1.rs to demonstrate unsafe swapping of heap-allocated generator
// structs wrapped by a Pin.

use std::marker::PhantomPinned;
use std::pin::Pin;

pub fn main() {
    let gen1 = GeneratorA::start("Gen1");
    let gen2 = GeneratorA::start("Generator2");

    // Construct pinned generator objects with a back-door to the wrapped data in the heap.
    let mut box1 = Box::new(gen1);
    let g1: *mut GeneratorA = &mut *box1;
    let mut pinned1 = unsafe { Pin::new_unchecked(box1) };
    let mut box2 = Box::new(gen2);
    let g2: *mut GeneratorA = &mut *box2;
    let mut pinned2 = unsafe { Pin::new_unchecked(box2) };

    // Print the generator states upon start.
    println!("Generator states upon start.");
    dump_raw(g1, "g1");
    dump_raw(g2, "g2");

    // Execute resume() on generators.
    println!("Executing resume() on generators.");
    if let GeneratorState::Yielded(n) = pinned1.as_mut().resume() {
        println!("Result from pinned1.as_mut().resume(): {}", n);
    }

    if let GeneratorState::Yielded(n) = pinned2.as_mut().resume() {
        println!("Result from pinned2.as_mut().resume(): {}", n);
    };

    // Print the generator states before swapping them.
    println!("Generator states after resume(), before swap.");
    dump_raw(g1, "g1");
    dump_raw(g2, "g2");

    unsafe {
        std::mem::swap(&mut *g1, &mut *g2);
    }
    println!("Swapped g1 and g2");

    // Print the generator states after swapping them.
    println!("Generator states after swap.");
    dump_raw(g1, "g1");
    dump_raw(g2, "g2");

    // Resume the swapped generators.
    println!("Resuming the swapped generators");
    println!("Executed pinned1.as_mut().resume():");
    let _ = pinned1.as_mut().resume();
    println!("Executed pinned2.as_mut().resume():");
    let _ = pinned2.as_mut().resume();

    // Print the updated generator states. Notice that g1 and g2 continue to point to the swapped generators
    // but they are now in the Exit state.
    println!("Updated generator states:");
    dump_raw(g1, "g1");
    dump_raw(g2, "g2");
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

#[derive(Debug)]
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
                let borrowed = unsafe { &**borrowed };
                println!(
                    "Yield1 state: borrowed={:p}, to_borrow={}, address of to_borrow={:p})",
                    borrowed, to_borrow, to_borrow
                );

                // Commented line below causes segmentation fault when executing pinned2.as_mut().resume() if uncommented.
                // The reason for that is that the `borrowed` pointer is pointing to an address that previously
                // contained the string "Gen1" but now has been overwritten as the state of g1 has changed from Yield1 to
                // Exit.
                // println!("   *borrowed={}", borrowed);

                *this = GeneratorA::Exit;
                GeneratorState::Complete(())
            }

            GeneratorA::Exit => panic!("Can't advance an exited generator!"),

            GeneratorA::_Phantom(_) => panic!("Unreachable code."),
        }
    }
}

// Prints the pointer address (twice) and the data pointed to by a raw pointer to a GeneratorA object.
fn dump_raw(g: *const GeneratorA, name: &str) {
    unsafe {
        println!(
            "{name}: {:p} {:?} {:?}",
            g,
            g,
            g.as_ref().as_deref().unwrap()
        );
    }
}
