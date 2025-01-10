//! Generation of a one-tail Student's t-test table using the [statrs] crate.
//! The values in the table have negative signs because CDFs provide cumulative probabilities from minus infinity to
//! the argument value.

use statrs::distribution::{ContinuousCDF, StudentsT};

fn main() {
    let dfs: Vec<usize> = (1..=30).collect();
    let alphas: &[f64] = &[0.1, 0.05, 0.025, 0.01, 0.005, 0.001, 0.0005];

    let table: Vec<Vec<f64>> = dfs
        .iter()
        .map(|df| {
            let stud = StudentsT::new(0.0, 1.0, *df as f64).unwrap();
            alphas
                .iter()
                .map(|alpha| stud.inverse_cdf(*alpha))
                .collect()
        })
        .collect();

    println!("ps -> {alphas:?}");
    println!("dfs");
    println!("vvv");
    for i in 0..(dfs.len()) {
        print!("{} | ", dfs[i]);
        println!("{:?}", table[i]);
    }
}
