macro_rules! compose {
    ($h:ident, $s:ty, $u:ty, $f:ident, $g:ident) => {
        fn $h(x: $s) -> $u {
            $g($f(x))
        }
    };
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn to_string(x: i32) -> String {
    x.to_string()
}

compose!(comp, i32, String, add_one, to_string);

fn main() {
    assert_eq!(comp(5), "6");
    println!("comp(5)={}", comp(5));
}
