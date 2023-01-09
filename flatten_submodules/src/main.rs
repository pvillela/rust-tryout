mod math;

use crate::math::{Complex, Matrix, Vector};

fn main() {
    let vec = Vector { x: 1 };
    let mat = Matrix {};
    let cmp: Complex = Complex {};
    println!("vec.x: {}, mat: {:?}, cmp: {:?}", vec.x, mat, cmp)
}
