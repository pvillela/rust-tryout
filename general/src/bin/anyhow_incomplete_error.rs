#![allow(unused)]

use anyhow::{anyhow, Error as AnyError};
use std::error::Error;
use std::fmt::Display;

/// Does not implement Error trait
#[derive(Debug)]
struct IncompleteError {
    foo: String,
}

// impl Error for IncompleteError {}

impl Display for IncompleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = "Incomplete Error{".to_owned() + &self.foo + "}";
        f.write_str(&s)
    }
}

fn display_error<T: Error>(err: T) {
    println!("The error is: {}", err);
}

fn main() {
    let err: IncompleteError = IncompleteError {
        foo: "foo".to_owned(),
    };

    // Below oesn't compile because IncompleteError doesn't implement std::error::Error.
    // display_error(err);

    let any_err: AnyError = anyhow!(err);
    let chain = any_err.chain();
    for x in chain {
        // Below oesn't compile because IncompleteError doesn't implement std::error::Error.
        // let y = x.downcast_ref::<IncompleteError>();

        println!("x: {}", x);
    }
}
