use just_convert::JustConvert;
use std::collections::HashMap;

/// `Model1` can be converted from `Model2a`:
/// - All the fields in `Model1` also exist and have the same type in `Model2a`.
#[derive(JustConvert, Debug, Clone, PartialEq)]
#[convert(from(Model2a))]
struct Model1 {
    id: i32,
    name: String,
    attrs: Vec<String>,
}

#[derive(Debug, Clone)]
struct Model1a {
    #[allow(unused)]
    id: i32,
    nom: String,
    attrs: Vec<String>,
    m: Model1,
}

/// `Model2a` can be converted from `Model1`:
/// - Three fields from `Model1` correspond directly to those in `Model2` and are mapped
///   directly.
/// - The `meta` field exists only in `Model2`, so it can't be directly mapped from `Model1`.
///   In the conversion, it gets its value from the expression provided in the field attribute,
///   i.e., the default value for the field type (an empty `HashMap`).
#[derive(JustConvert, Debug, Clone, Default, PartialEq)]
#[convert(from(Model1))]
struct Model2a {
    id: i32,
    name: String,
    attrs: Vec<String>,
    #[convert(map = "Default::default()")]
    meta: HashMap<String, String>,
}

/// `Model3a` can be converted from both `Model1a` and `Model2a`.
///
/// For the conversion from `Model1a`:
/// - The `id` field gets its value from the provided expression, not from the corresponding
///   field in `Model1a`.
/// - The `name` field doesn't exist in `Model1a` but it is mapped from the `nom` field in
///   `Model1a`.
/// - The `attrs` field is mapped directly from the corresponding field in `Model1a`.
/// - The `metadata` field gets its value from the provided expression, the default value
///   for its type (empty `HashMap`).
/// - The `m` field is mapped directly from the corresponding field in `Model1a`. Although
///   `Model1a::m` has a different type from `Model3a::m`, the latter type implements the
///   `From` trait for the former type (by virtue of using the `map_from` macro).
///
/// For the conversion from `Model2a`:
/// - The `id` field is mapped directly from the corresponding field in `Model2a`. Although
///   `Model2a::id` has a different type from `Model3a::id`, the latter type implements the
///   `From` trait for the former type.
/// - The `name` field is mapped directlyfrom the corresponding field in `Model2a`.
/// - The `attrs` field is mapped directly from the corresponding field in `Model2a`.
/// - The `metadata` field is mapped from the `meta` field in `Model2a`.
/// - The `m` field receives its value from the provided expression, the default value
///   of its type.
#[derive(JustConvert, Debug)]
#[convert(from(Model1a))]
#[convert(from(Model2a))]
struct Model3a {
    #[convert(map(from(Model1a, "42")))]
    id: i64,

    #[convert(rename(from(Model1a, nom)))]
    name: String,

    attrs: Vec<String>,

    #[convert(map(from(Model1a, "Default::default()")))]
    #[convert(rename(from(Model2a, meta)))]
    metadata: HashMap<String, String>,

    #[convert(map(from(Model2a, "Default::default()")))]
    m: Model2a,
}

fn main() {
    println!("\nModel2a from Model1 and Model1 from Model2a");
    {
        let m1 = Model1 {
            id: 1,
            name: "Xyz".into(),
            attrs: vec!["a".into(), "b".into()],
        };

        let m1_2a: Model2a = m1.clone().into();
        let m1_2a_1: Model1 = m1_2a.clone().into();

        println!("m1={m1:?}");
        println!("m1={m1_2a:?}");
        println!("m1_2a_1={m1_2a_1:?}");

        assert_eq!(m1_2a_1, m1);
    }

    println!("\nModel3a from Model1a");
    {
        let m1a = Model1a {
            id: 1,
            nom: "Joe".into(),
            attrs: vec!["a".into(), "b".into()],
            m: Model1 {
                id: 99,
                name: "Mary".into(),
                attrs: vec!["x".into(), "y".into(), "z".into()],
            },
        };

        let m1a_3a: Model3a = m1a.clone().into();

        println!("m1a={m1a:?}");
        println!("m1a_3a={m1a_3a:?}");

        assert_eq!(m1a_3a.id, 42);
        assert_eq!(m1a_3a.name, m1a.nom.to_owned());
        assert_eq!(m1a_3a.attrs, m1a.attrs);
        assert_eq!(m1a_3a.metadata, HashMap::new());
        assert_eq!(m1a_3a.m, m1a.m.into());
    }

    println!("\nModel3a from Model2a");
    {
        let m2a = Model2a {
            id: 1,
            name: "Xyz".into(),
            attrs: vec!["a".into(), "b".into()],
            meta: HashMap::from([("abc".into(), "111".into()), ("def".into(), "222".into())]),
        };

        let m2a_3a: Model3a = m2a.clone().into();

        println!("m2a={m2a:?}");
        println!("m2a_3a={m2a_3a:?}");

        assert_eq!(m2a_3a.id, m2a.id.into());
        assert_eq!(m2a_3a.name, m2a.name);
        assert_eq!(m2a_3a.attrs, m2a.attrs);
        assert_eq!(m2a_3a.metadata, m2a.meta);
        assert_eq!(m2a_3a.m, Default::default());
    }
}
