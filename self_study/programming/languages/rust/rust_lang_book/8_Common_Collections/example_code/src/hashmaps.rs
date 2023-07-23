pub mod examples; // ..declare src/hashmaps/examples.rs

// Rust type - HashMap<K, V>
pub fn run() {
    println!("\ncalling hashmaps::examples::declare_hashmap()");
    examples::declare_hashmap();

    println!("\ncalling hashmaps::examples::iterate_over_k_v_pair()");
    examples::iterate_over_k_v_pair();

    println!("\ncalling hashmaps::examples::insert_and_ownership_for_hashmap()");
    examples::insert_and_ownership_for_hashmap();

    println!("\ncalling hashmaps::examples::on_updating_hashmap()");
    examples::on_updating_hashmap();
}
