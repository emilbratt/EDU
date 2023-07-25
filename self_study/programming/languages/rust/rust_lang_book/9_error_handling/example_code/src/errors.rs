pub mod examples; // ..declare src/errors/examples.rs

pub fn run() {
    // Uncomment the call you want to test

    // println!("\ncalling errors::examples::errors_macro()");
    // examples::errors_macro();

    // println!("\ncalling errors::examples::access_index_out_of_bounds()");
    // examples::access_index_out_of_bounds();

    println!("\ncalling errors::examples::recoverable_error()");
    examples::recoverable_error();

    println!("\ncalling errors::examples::propagating_error()");
    examples::propagating_error();

    println!("\ncalling errors::examples::propagating_error_v2()");
    examples::propagating_error_v2();

    println!("\ncalling errors::examples::validate_custom_types()");
    examples::validate_custom_types();
}
