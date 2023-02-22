struct Foo {
    a: String,
    b: i32,
}

fn foo_src() -> Foo {
    Foo {
        a: "foo".to_owned(),
        b: 42,
    }
}

fn main() {
    let Foo { a, b } = foo_src();
    println!("a={}, b={}", a, b);
}
