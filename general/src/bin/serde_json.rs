use serde::Serialize;

#[derive(Serialize)]
struct Foo(u32);

#[derive(Serialize)]
struct Bar {
    x: u32,
    y: &'static str,
}

#[derive(Serialize)]
enum Baz {
    X,
    Y(u32),
}

fn serialize_to_json_string<T: Serialize>(x: &T) {
    let j = serde_json::to_string(x).unwrap();
    println!("{j}");
}

fn main() {
    let foo = Foo(10);
    let bar = Bar { x: 1, y: "2" };
    let baz_x = Baz::X;
    let baz_y = Baz::Y(99);

    serialize_to_json_string(&foo);
    serialize_to_json_string(&bar);
    serialize_to_json_string(&baz_x);
    serialize_to_json_string(&baz_y);
}
