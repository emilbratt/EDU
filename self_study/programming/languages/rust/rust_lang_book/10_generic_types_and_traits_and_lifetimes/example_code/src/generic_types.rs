/* NOTE:
 *  Modules strictly follow directory naming.
 *  This means Rust uses looks for modules in the directory with the same name as this filename.
 */

pub mod remove_deduplicate_function; // module: src/generic_types/remove_deduplicate_function.rs
pub mod function_definitions; // module: src/generic_types/function_definitions.rs
pub mod struct_definitions; // module: src/generic_types/struct_definitions.rs
pub mod enum_definitions; // module: src/generic_types/enum_definitions.rs
pub mod methods_definitions; // module: src/generic_types/methods_definitions.rs

pub fn run_all() {
    println!("running all examples");
    remove_deduplicate_function::run_example();
    function_definitions::run_example();
    struct_definitions::run_example();
    enum_definitions::run_example();
    methods_definitions::run_example();
}
