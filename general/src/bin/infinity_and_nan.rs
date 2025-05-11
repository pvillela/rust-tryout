//! Demonstrates comparison with [`f64::INFINITY`] and [`f64::NAN`].

fn main() {
    assert_eq!(f64::INFINITY, f64::INFINITY);
    assert!(f64::INFINITY == f64::INFINITY);
    assert!(-f64::INFINITY < 0.);
    assert!(f64::INFINITY > 0.);

    assert_ne!(f64::INFINITY, f64::NAN);
    assert_ne!(f64::NAN, f64::NAN);

    println!("success");
}
