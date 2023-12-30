mod m {
    trait X {
        fn x(&self) -> i32;
    }

    trait A: X {
        fn f(&self) -> i32 {
            self.x() * 2
        }
    }

    struct Foo {
        pub x: i32,
    }

    impl X for Foo {
        fn x(&self) -> i32 {
            self.x
        }
    }

    impl A for Foo {}

    pub struct Bar(Foo);

    impl Bar {
        pub fn new(x: i32) -> Self {
            Self(Foo { x })
        }

        pub fn f(&self) -> i32 {
            self.0.f()
        }
    }
}

fn main() {
    use m::Bar;

    let bar = Bar::new(1);

    println!("{}", bar.f());
}
