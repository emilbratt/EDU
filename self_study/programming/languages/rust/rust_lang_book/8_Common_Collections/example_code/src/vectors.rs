pub mod examples; // ..declare src/vectors/examples.rs

// Rust type - Vec<T>
pub fn run() {
    println!("\ncalling vectors::examples::declare_vectors()");
    examples::declare_vectors();

    println!("\ncalling vectors::examples::index_vectors()");
    examples::access_value_from_vectors();

    println!("\ncalling vectors::examples::borrowing_and_vectors()");
    examples::borrowing_and_vectors();

    println!("\ncalling vectors::examples::iterating_vectors()");
    examples::iterating_vectors();

    println!("\ncalling vectors::examples::enums_in_vectors()");
    examples::enums_in_vectors();
    
    println!("\ncalling vectors::examples::scopes_and_vectors()");
    examples::scopes_and_vectors();


}
