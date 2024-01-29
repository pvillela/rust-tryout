//! Demonstration of 3 patterns to enable and/or facilitate the polymorphic extension of struct functionality.
//!
//! These are different from the well-know pattern of using a trait to extend the functionality of a struct
//! defined in another module.
//!
//! Patterns:
//! 1. **Parameter packing** -- This entails using a trait to pack together multiple parameters, so that the struct takes
//!    that trait as a parameter instead of multiple individual parameters. The individual parameters are defined as
//!    associated types in the trait. See [`pack`].
//! 2. **Discriminated inheritance** -- This involves using a trait (the discriminator) that is taken as a struct
//!    parameter and having different `impls` of the struct for different implementations of the discriminator trait.
//!    See [`discr`].
//! 3. **Abstract methods** -- Enables the definition of abstract methods in a struct by defining function fields that
//!    are used to implement the corresponding methods. The function fields are initialized with different functions
//!    for different concrete manifestations of the struct. This pattern also uses a discriminator trait, similar to
//!    the one in 2 above. See [`abstr`].
//!
//! It is important to note that these patterns can be used together, either with a single trait that supports packing,
//! discrimination, and abstraction, or with multiple traits that support one or more of these patterns.

pub mod abstr;
pub mod discr;
pub mod pack;
