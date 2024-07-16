//! This example shows how a trait ([`BarABf`]) can have a default implementaion provided by another
//! trait ([`BarABfBoot`]) and how other code can use that.

use std::marker::PhantomData;

trait CfgSrc {
    fn cfg_src() -> i32;
}

trait T1 {
    fn f1();
}

trait BarABf<CTX> {
    fn bar_a_bf() -> i32;
}

trait BarABfBoot<CTX>
where
    CTX: CfgSrc + T1,
{
    fn bar_a_bf_boot() -> i32 {
        CTX::f1();
        CTX::cfg_src() * 10
    }
}

impl<CTX, T> BarABf<CTX> for T
where
    CTX: CfgSrc + T1,
    T: BarABfBoot<CTX>,
{
    fn bar_a_bf() -> i32 {
        T::bar_a_bf_boot()
    }
}

#[allow(unused)]
/// This trait is not useful for `Sfl`s but is needed for `Fl`s.
trait FooASfl<CTX> {
    fn foo_a_sfl() -> i32;
}

/// This trait is not useful for `Sfl`s but is needed for `Fl`s.
trait FooASflC<CTX>: BarABf<CTX>
where
    CTX: CfgSrc,
{
    fn foo_a_sfl_c() -> i32 {
        Self::bar_a_bf() + CTX::cfg_src() * 2
    }
}

struct Boot<CTX>(PhantomData<CTX>);

impl<CTX> BarABfBoot<CTX> for Boot<CTX> where CTX: CfgSrc + T1 {}
impl<CTX> FooASflC<CTX> for Boot<CTX> where CTX: CfgSrc + T1 {}

fn foo_a_sfl_boot<CTX>() -> i32
where
    CTX: CfgSrc + T1,
{
    Boot::<CTX>::foo_a_sfl_c()
}

#[allow(unused)]
/// This trait is not useful for `Sfl`s but is needed for `Fl`s.
trait FooASflBoot<CTX>
where
    CTX: CfgSrc + T1,
{
    fn foo_a_sfl_boot() -> i32 {
        Boot::<CTX>::foo_a_sfl_c()
    }
}

struct Ctx;

impl CfgSrc for Ctx {
    fn cfg_src() -> i32 {
        42
    }
}

impl T1 for Ctx {
    fn f1() {}
}

fn main() {
    let res = foo_a_sfl_boot::<Ctx>();
    println!("{res}");
}
