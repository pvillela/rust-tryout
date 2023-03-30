// use std::rc::Rc;

fn send_and_sync<T: Send, U: Sync>(_x: T, _y: U) {}

#[allow(unused)]
struct Foo {
    x: i32,
    y: String,
}

fn main() {
    let foo1 = Foo {
        x: 42,
        y: "hello".to_string(),
    };

    let foo2 = Foo {
        x: 42,
        y: "hello".to_string(),
    };

    send_and_sync(&foo1, &foo2);

    send_and_sync(foo1, foo2);

    // Below doesn't compile because Rc is not Send.
    // send_and_sync(Rc::new(1), yRc::new(1));
}
