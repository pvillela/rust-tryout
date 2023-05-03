//! Calculation of probability of survival for prisoners problem described in
//! https://www.youtube.com/watch?v=iSNsgj1OCLA&t=509s
//!
//! Let D = number of permutations of 100 items with a cycle of length 51 or greater:
//! Let C(n, m) be the number of subsets of cardinality m of a set of cardinality n.
//! D = C(100, 51) * 50! * 49! +
//!     C(100, 52) * 51! * 48! +
//!     ...
//!     C(100, 100) * 99! * 0!
//!   = 100! / (51! * 49!) * 50! * 49! +
//!     100! / (52! * 48!) * 51! * 48! +
//!     ...
//!     100! / (100! * 0!) * 99! * 0!
//!   = 100! / 51 +
//!     100! / 52 +
//!     ...
//!     100! / 100
//!
//! The probability of death is D / 100! = 1 / 51 + 1 / 52 + ... + 1 / 100  

fn main() {
    let p_death = f(51, 0.0);
    let p_life = 1.0 - p_death;
    println!("p_death: {}, p_life: {}", p_death, p_life);
}

fn f(n: i32, acc: f64) -> f64 {
    if n > 100 {
        return acc;
    };
    let acc = acc + 1.0 / (n as f64);
    f(n + 1, acc)
}
