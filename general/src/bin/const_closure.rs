/// Transforms a value into a nullary closure that returns the value.
fn const_closure<T: 'static + Clone>(x: T) -> impl Fn() -> T {
    move || x.clone()
}

fn main() {
    let f = const_closure(1);
    let x = f();
    println!("x: {}", x);
}
