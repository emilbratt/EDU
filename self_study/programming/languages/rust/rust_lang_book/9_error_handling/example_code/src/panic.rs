pub mod examples; // ..declare src/panic/examples.rs

pub fn run() {
    // Uncomment the call you want to test

    // println!("\ncalling panic::examples::panic_macro()");
    // examples::panic_macro();

    // println!("\ncalling panic::examples::access_index_out_of_bounds()");
    // examples::access_index_out_of_bounds();

    println!("\ncalling panic::examples::recoverable_error()");
    examples::recoverable_error();
}
