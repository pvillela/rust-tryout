//! This submodule demonstrates the ***abstract methods*** pattern (see [parent module](super)).
//!
//! The example in this module shows how abstract methods can be implemented as function fields on structs
//! and abstract fields can be implemented as a field with a discriminant trait as its type.
//! The implementations of abstract methods can use both concrete and abstract fields.

/// Trait that serves as discriminant for different implementations of abstract method and
/// holder for abstract fields.
pub trait Abstr {}

/// Struct with an abstract method based on function field `a_fn` and abstract fields `a_value`.
pub struct Foo<A: Abstr> {
    value: i32,
    a_fn: fn(&Self) -> i32,
    a_value: A,
}

impl<A: Abstr> Foo<A> {
    /// Abstract method based on function field `a_fn`.
    fn a_meth(&self) -> i32 {
        (self.a_fn)(self)
    }

    /// Use of abstract method.
    pub fn calc(&self) -> i32 {
        self.value * self.a_meth()
    }
}

/// Discriminant struct with single String abstract field.
pub struct Abstr1(String);

impl Abstr for Abstr1 {}

/// Another discriminant struct, with two i32 abstract fields.
pub struct Abstr2 {
    x: i32,
    y: i32,
}

impl Abstr for Abstr2 {}

/// Struct with an implementation of the abstract method.
impl Foo<Abstr1> {
    fn a_fn(this: &Self) -> i32 {
        this.value + (this.a_value.0.len() as i32)
    }

    pub fn new(value: i32, a_value: String) -> Self {
        Self {
            value,
            a_fn: Self::a_fn,
            a_value: Abstr1(a_value),
        }
    }
}

/// Struct with a different implementation of the abstract method.
impl Foo<Abstr2> {
    fn a_fn(this: &Self) -> i32 {
        this.value + this.a_value.x + this.a_value.y
    }

    pub fn new(value: i32, x: i32, y: i32) -> Self {
        Self {
            value,
            a_fn: Self::a_fn,
            a_value: Abstr2 { x, y },
        }
    }
}

pub fn abstr_method_main() {
    println!("abstr_method_main");

    let foo1: Foo<Abstr1> = Foo::<Abstr1>::new(5, "foo".to_owned());
    println!("foo1.calc(): {}", foo1.calc());

    let foo2: Foo<Abstr2> = Foo::<Abstr2>::new(5, 4, 6);
    println!("foo2.calc(): {}", foo2.calc());
}
