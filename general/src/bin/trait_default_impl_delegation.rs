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

trait FooASflDeps<CTX>: BarABf<CTX> {}

trait FooASflDepsParam {
    type Deps;
}

trait CtxParam {
    type Ctx;
}

fn foo_a_sfl_c<P>() -> i32
where
    P: CtxParam + FooASflDepsParam,
    P::Ctx: CfgSrc,
    P::Deps: FooASflDeps<P::Ctx>,
{
    P::Deps::bar_a_bf() + P::Ctx::cfg_src() * 2
}

struct Deps;

impl<CTX> BarABfBoot<CTX> for Deps where CTX: CfgSrc + T1 {}
impl<CTX> FooASflDeps<CTX> for Deps where CTX: CfgSrc + T1 {}

struct P<CTX>(PhantomData<CTX>);

impl<CTX> CtxParam for P<CTX> {
    type Ctx = CTX;
}

impl<CTX> FooASflDepsParam for P<CTX> {
    type Deps = Deps;
}

fn foo_a_sfl_boot<CTX>() -> i32
where
    CTX: CfgSrc + T1,
{
    foo_a_sfl_c::<P<CTX>>()
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
