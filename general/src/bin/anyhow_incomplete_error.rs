//! This example shows how anyhow can be used with structs that do not implement std::error::Error,
//! demonstrates use of the downcast method of Any, and shows how the type_id method is disabled
//! in std::error::Error and how to work around this.

#![allow(unused)]

use anyhow::{anyhow, Error as AnyhowError};
use std::any::Any;
use std::error::Error;
use std::fmt::Display;

/// Does not implement Error trait
#[derive(Debug, Clone)]
struct IncompleteError {
    foo: String,
}

/// Implements Error trait
#[derive(Debug, Clone)]
struct CompleteError {
    bar: String,
}

impl Display for CompleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("CompleteError {{ bar: {} }}", self.bar))
    }
}

impl Error for CompleteError {}

impl Display for IncompleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("IncompleteError {{ foo: {} }}", self.foo))
    }
}

fn display_error<T: Error>(err: T) {
    println!("The error is: {}", err);
}

fn main() {
    {
        let err: IncompleteError = IncompleteError {
            foo: "foo".to_owned(),
        };

        // Below oesn't compile because IncompleteError doesn't implement std::error::Error.
        // display_error(err);

        let any_err: AnyhowError = anyhow!(err.clone());
        let chain = any_err.chain();
        for x in chain {
            // Below oesn't compile because IncompleteError doesn't implement std::error::Error.
            // if let Some(y) = x.downcast_ref::<IncompleteError>() {
            //     println!("y: {}", y);
            // }

            let type_id = err.type_id();
            println!("x: &{}, err.type_id() {:?}", x, type_id);
        }
    }

    {
        let err: CompleteError = CompleteError {
            bar: "bar".to_owned(),
        };

        let any_err: AnyhowError = anyhow!(err.clone());
        let chain = any_err.chain();
        for x in chain {
            if let Some(y) = x.downcast_ref::<CompleteError>() {
                println!("y: &{}", y);
            }

            // Line below doesn't compile because std::error::Error disables the type_id() method.
            // Need workaround below.
            // let type_id = err.type_id();

            let type_id = std::any::Any::type_id(&err);
            println!("x: &{}, err.type_id() {:?}", x, type_id);
        }
    }
}
