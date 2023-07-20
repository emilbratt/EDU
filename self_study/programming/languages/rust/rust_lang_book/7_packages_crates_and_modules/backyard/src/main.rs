// declare a module
pub mod garden; // ..tells the compiler the module exists in src/garden.rs

// so we can call Asparagus {} instead of garden::vegetables::Asparagus {};
use crate::garden::vegetables::Asparagus;

fn main() {
    // We can now use the dummy struct in this example. (see line 1)
    let plant = Asparagus {};
    // Printing is made possible by adding #[derive(Debug)] in the same file as the struct.
    println!("I'm growing {:?}!", plant);
}
