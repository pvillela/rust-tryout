//! Shows that importing an alias is sufficient to use it, without importing its dependencies,
//! provided that all types and traits the alias depends on are public.

mod a {
    pub trait Foo {
        fn foo(&self);
    }

    impl Foo for i32 {
        fn foo(&self) {
            println!("{self}")
        }
    }

    pub struct Bar<T: Foo> {
        pub bar: T,
    }

    pub type BarI32 = Bar<i32>;

    pub fn fbar(x: BarI32) {
        x.bar.foo();
    }
}

fn main() {
    use a::{fbar, BarI32};

    let x = BarI32 { bar: 42 };
    fbar(x);
}
