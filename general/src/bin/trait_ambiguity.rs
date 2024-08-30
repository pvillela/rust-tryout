/// Demonstrates how a type can implement two traits with the same associated types and/or functions.

trait Tr1 {
    type T;

    fn f() -> Self::T;
}

trait Tr2 {
    type T;

    fn f() -> Self::T;
}

struct Foo;

impl Tr1 for Foo {
    type T = i32;

    fn f() -> Self::T {
        42
    }
}

impl Tr2 for Foo {
    type T = String;

    fn f() -> Self::T {
        "99".into()
    }
}

fn main() {
    let x1 = <Foo as Tr1>::f();
    let x2 = <Foo as Tr2>::f();

    println!("x1={x1}, x2={x2}");
}
