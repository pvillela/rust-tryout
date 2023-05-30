macro_rules! compose0 {
    ($h:ident, $s:ty, $u:ty, $f:ident, $g:ident) => {
        fn $h(x: $s) -> $u {
            $g($f(x))
        }
    };
}

macro_rules! compose {
    ($f:ident, $g:ident) => {
        |x| $g($f(x))
    };
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn to_string(x: i32) -> String {
    x.to_string()
}

compose0!(comp, i32, String, add_one, to_string);

fn main() {
    assert_eq!(comp(5), "6");
    println!("comp(5)={}", comp(5));

    let c: fn(i32) -> String = compose!(add_one, to_string);
    assert_eq!(c(5), "6");
    println!("c(5)={}", c(5));
}
