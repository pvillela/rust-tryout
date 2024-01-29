//! Example executable for module [general::polymorphic_struct_extension];
//! Demonstration of 3 patterns to enable and/or facilitate the polymorphic extension of struct functionality.

use general::polymorphic_struct_extension::{
    abstr::abstr_method_main, discr::discriminated_main, pack::packing_main,
};

fn main() {
    packing_main();
    discriminated_main();
    abstr_method_main();
}
