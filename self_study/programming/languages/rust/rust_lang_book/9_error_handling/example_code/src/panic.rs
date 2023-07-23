pub mod examples; // ..declare src/panic/examples.rs

pub fn run() {
    println!("\ncalling panic::examples::panic_macro()");
    examples::panic_macro();
}
