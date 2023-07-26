pub mod remove_deduplicate_function; // ..declare src/examples/remove_deduplicate_function.rs
pub mod function_definitions; // ..declare src/examples/function_definitions.rs
pub mod struct_definitions; // ..declare src/examples/struct_definitions.rs

pub fn run_all() {
    println!("running all examples");
    remove_deduplicate_function::run_example();
    function_definitions::run_example();
    struct_definitions::run_example();
}
