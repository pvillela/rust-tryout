use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Foo(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Bar {
    x: u32,
    y: String,
    foo: Foo,
    foo_box: Box<Foo>,
    foo_opt: Option<Foo>,
    foo_box_opt: Option<Box<Foo>>,
    foo_box_opt1: Option<Box<Foo>>,
    fuz: (),
    fuz_box: Box<()>,
    fuz_opt: Option<()>,
    fuz_box_opt: Option<Box<()>>,
    fuz_box_opt1: Option<Box<()>>,
}

#[derive(Serialize, Debug)]
enum Baz {
    X,
    Y(&'static str),
}

fn serialize_to_json_string<T: Serialize + Debug>(x: &T) -> String {
    let j = serde_json::to_string(x).unwrap();
    println!("=== value: {x:?}, \n--- serialized: {j}");
    j
}

fn deserialize_from_json_string<T: DeserializeOwned + Debug>(jstr: &str) -> T {
    let t: T = serde_json::from_str(jstr).unwrap();
    println!("=== json_string: {jstr}, \n--- deserialized: {t:?}");
    t
}

fn main() {
    let foo = Foo("hello".into());
    let baz_x = Baz::X;
    let baz_y = Baz::Y("hello");

    let bar = Bar {
        x: 1,
        y: "2".into(),
        foo: foo.clone(),
        foo_box: Box::new(foo.clone()),
        foo_opt: Some(foo.clone()),
        foo_box_opt: Some(Box::new(foo.clone())),
        foo_box_opt1: None,
        fuz: (),
        fuz_box: Box::new(()),
        fuz_opt: Some(()),
        fuz_box_opt: Some(Box::new(())),
        fuz_box_opt1: None,
    };

    let bar1 = Bar {
        x: 1,
        y: "2".into(),
        foo: foo.clone(),
        foo_box: Box::new(foo.clone()),
        foo_opt: Some(foo.clone()),
        foo_box_opt: Some(Box::new(foo.clone())),
        foo_box_opt1: None,
        fuz: (),
        fuz_box: Box::new(()),
        fuz_opt: None,
        fuz_box_opt: None,
        fuz_box_opt1: None,
    };

    serialize_to_json_string(&foo);
    serialize_to_json_string(&baz_x);
    serialize_to_json_string(&baz_y);

    let bar_jstr = serialize_to_json_string(&bar);
    let deser_bar: Bar = deserialize_from_json_string(&bar_jstr);
    assert_ne!(bar, deser_bar);
    assert_eq!(bar1, deser_bar);
}
