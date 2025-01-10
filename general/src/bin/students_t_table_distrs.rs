//! Generation of a one-tail Student's t-test table using the [distrs] crate.
//! The values in the table have negative signs because CDFs provide cumulative probabilities from minus infinity to
//! the argument value.

use distrs::StudentsT;

fn main() {
    let dfs: Vec<usize> = (1..=30).collect();
    let aphas: &[f64] = &[0.1, 0.05, 0.025, 0.01, 0.005, 0.001, 0.0005];

    let table: Vec<Vec<f64>> = dfs
        .iter()
        .map(|df| {
            aphas
                .iter()
                // `ppf` is the inverse of the `cdf`.
                .map(|alpha| StudentsT::ppf(*alpha, *df as f64))
                .collect()
        })
        .collect();

    println!("ps -> {aphas:?}");
    println!("dfs");
    println!("vvv");
    for i in 0..(dfs.len()) {
        print!("{} | ", dfs[i]);
        println!("{:?}", table[i]);
    }
}
