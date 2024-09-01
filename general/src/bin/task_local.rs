//! Example usage of [`tokio::task_local`]

use std::{fmt::Debug, marker::PhantomData};

use tokio::task::LocalKey;

#[allow(unused)]
#[derive(Debug, Clone)]
struct TlWithLocale {
    locale: String,
}

tokio::task_local! {
    static NUMBER: u32;

    static CTX_TL: TlWithLocale;
}

async fn simple() {
    println!("task local value: {}", NUMBER.get());
}

trait TaskLocalCtx {
    type TaskLocalType: Clone + 'static;

    fn get_static() -> &'static LocalKey<Self::TaskLocalType>;

    fn tl_value() -> Self::TaskLocalType {
        let lk = Self::get_static();
        lk.with(|tlc| tlc.clone())
    }
}

async fn foo_sfl<CTX: TaskLocalCtx>() -> String
where
    CTX::TaskLocalType: Debug,
{
    format!("{:?}", CTX::tl_value())
}

struct Ctx;

impl TaskLocalCtx for Ctx {
    type TaskLocalType = TlWithLocale;

    fn get_static() -> &'static LocalKey<TlWithLocale> {
        &CTX_TL
    }
}

trait TaskLocalScopedFn<CTX: TaskLocalCtx> {
    type In;
    type Out;

    #[allow(async_fn_in_trait)]
    async fn call(input: Self::In) -> Self::Out;

    #[allow(async_fn_in_trait)]
    async fn tl_scoped(value: CTX::TaskLocalType, input: Self::In) -> Self::Out {
        let lk = CTX::get_static();
        lk.scope(value, Self::call(input)).await
    }
}

struct FooI<CTX>(PhantomData<CTX>);

impl<CTX: TaskLocalCtx> TaskLocalScopedFn<CTX> for FooI<CTX>
where
    CTX::TaskLocalType: Debug,
{
    type In = ();
    type Out = String;

    async fn call(_input: Self::In) -> Self::Out {
        foo_sfl::<CTX>().await
    }
}

#[tokio::main]
async fn main() {
    NUMBER.scope(1, simple()).await;

    // Scoping by hand
    {
        let h = tokio::spawn(async {
            let tlc = TlWithLocale {
                locale: "pt-br".into(),
            };
            CTX_TL.scope(tlc, foo_sfl::<Ctx>()).await
        });
        let foo_out = h.await.unwrap();
        println!("foo output: {foo_out}");
    }

    // Scoping with trait support
    {
        let h = tokio::spawn(async {
            let tlc = TlWithLocale {
                locale: "en-ca".into(),
            };
            FooI::<Ctx>::tl_scoped(tlc, ()).await
        });
        let foo_out = h.await.unwrap();
        println!("foo output: {foo_out}");
    }
}
