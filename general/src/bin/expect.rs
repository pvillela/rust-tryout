//! Show what happens when `expect` is executed.

fn main() {
    let res = Err("my error");
    res.expect("trying expect")
}
