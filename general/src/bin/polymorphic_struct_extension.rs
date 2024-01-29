//! Example executable for module [`general::polymorphic_struct_extension`], which describes and demonstrates
//! 3 patterns to enable the polymorphic extension of struct functionality.

use general::polymorphic_struct_extension::{
    abstr::abstr_method_main, discr::discriminated_main, pack::packing_main,
};

fn main() {
    packing_main();
    discriminated_main();
    abstr_method_main();
}
