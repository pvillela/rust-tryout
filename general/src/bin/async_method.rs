struct Foo;

impl Foo {
    async fn foo(&self) {}
}

#[tokio::main]
async fn main() {
    let foo = Foo;
    foo.foo().await;
}
