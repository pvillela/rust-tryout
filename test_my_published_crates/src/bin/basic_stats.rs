//! To simulate another package that also implements `AokFloat`.
mod another {
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    #[derive(Debug)]
    pub struct AnotherError;

    impl Display for AnotherError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self, f)
        }
    }

    impl Error for AnotherError {}

    #[derive(Debug)]
    pub struct X {
        pub x: f64,
    }

    impl X {
        pub fn new(x: f64) -> Self {
            X { x }
        }

        pub fn div(self, y: f64) -> Result<Self, AnotherError> {
            if y != 0. {
                Ok(X::new(self.x / y))
            } else {
                Err(AnotherError)
            }
        }
    }

    pub trait AokFloat {
        type Value;

        fn aok(self) -> Self::Value;
    }

    impl<E> AokFloat for Result<f64, E> {
        type Value = f64;

        fn aok(self) -> Self::Value {
            self.unwrap_or(f64::NAN)
        }
    }

    pub trait AokAnother {
        type Value: AokAnotherValue;

        fn aok(self) -> Self::Value;
    }

    pub trait AokAnotherValue {
        fn aok_fallback() -> Self;
    }

    impl<T, E> AokAnother for Result<T, E>
    where
        T: AokAnotherValue,
    {
        type Value = T;

        fn aok(self) -> Self::Value {
            self.unwrap_or_else(|_| T::aok_fallback())
        }
    }

    impl AokAnotherValue for X {
        // Returns an instance constructed with `NaN`s as a fallback value.
        fn aok_fallback() -> Self {
            X { x: f64::NAN }
        }
    }
}

fn main() {
    use basic_stats::core::SampleMoments;

    let x = [14., 15., 15., 15., 16., 18., 22., 23., 24., 25., 25.];
    let moments_x = SampleMoments::from_slice(&x);

    {
        use basic_stats::aok::AokFloat;

        {
            println!("*** Ok scenario:");

            // Function calls below return Ok prior to invocation of noerr().

            let mean = moments_x.mean().aok();
            println!("mean={mean}");

            assert!(mean.is_finite());
        }

        {
            println!("*** Err scenario:");

            // Function calls below return Err prior to invocation of noerr().

            let mean = SampleMoments::default().mean().aok();
            println!("mean={mean}");

            assert!(mean.is_nan());
        }
    }

    // To demonstrate the use of another implementation of `AokFloat`.
    {
        use another::{AokAnother, AokFloat, X};

        {
            println!("*** Ok scenario:");

            // Function calls below return Ok prior to invocation of noerr().

            let mean = moments_x.mean().aok();
            println!("mean={mean}");

            let x = X::new(1.);
            let y = x.div(2.).aok();
            println!("y={y:?}");

            assert!(mean.is_finite());
            assert!(y.x.is_finite());
        }

        {
            println!("*** Err scenario:");

            // Function calls below return Err prior to invocation of noerr().

            let mean = SampleMoments::default().mean().aok();
            println!("mean={mean}");

            let x = X::new(1.);
            let y = x.div(0.).aok();
            println!("y={y:?}");

            assert!(mean.is_nan());
            assert!(y.x.is_nan());
        }
    }
}
