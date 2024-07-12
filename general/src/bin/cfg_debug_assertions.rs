//! Demonstrates how to use the `cfg!` macro, `#[cfg()]`, and the `debug_assertions` condition to write
//! code that has different behaviour in dev and release.
//!
//! Run `cargo run --bin cfg_debug_assertions` and then `cargo run -r --bin cfg_debug_assertions`.

fn main() {
    {
        const IS_DEV: bool = if cfg!(debug_assertions) { true } else { false };

        println!("IS_DEV={IS_DEV}");
    }

    {
        #[cfg(debug_assertions)]
        fn example() {
            println!("Debugging enabled");
        }

        #[cfg(not(debug_assertions))]
        fn example() {
            println!("Debugging disabled");
        }

        example();
    }

    {
        #[cfg(debug_assertions)]
        const DEBUGGING_ENABLED: bool = true;

        #[cfg(not(debug_assertions))]
        const DEBUGGING_ENABLED: bool = false;

        println!("DEBUGGING_ENABLED={DEBUGGING_ENABLED}");
    }
}
