/// Transforms a value into a nullary closure that returns the value.
fn const_closure<T: Clone>(x: T) -> impl Fn() -> T {
    move || x.clone()
}

/// Transforms a value into a nullary closure that returns the value but can only be called once.
fn const_once_closure<T>(x: T) -> impl FnOnce() -> T {
    move || x
}

fn main() {
    let f = const_closure(1);
    let x1 = f();
    let x2 = f();
    println!("x1: {}, x2: {}", x1, x2);

    let f_o = const_once_closure(2);
    let y1 = f_o();
    // let y2 = f_o(); // doesn't compile because f_o is FnOnce.
    println!("y1: {}", y1);
}
