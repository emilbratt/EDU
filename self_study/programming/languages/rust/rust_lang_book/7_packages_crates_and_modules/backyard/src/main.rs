// tell compiler to use Asparagus
use crate::garden::vegetables::Asparagus;

// declare a module
pub mod garden; // ..the name tells the compiler the module exists in src/garden.rs

fn main() {
    // We can now use the dummy struct in this example. (see line 1)
    let plant = Asparagus {};
    // Printing is made possible by adding #[derive(Debug)] in the same file as the struct.
    println!("I'm growing {:?}!", plant);
}
