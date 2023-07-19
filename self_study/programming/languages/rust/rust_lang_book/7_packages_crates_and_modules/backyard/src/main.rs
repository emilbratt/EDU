use crate::garden::vegetables::Asparagus;

pub mod garden; // ..tells the compiler to include the code it finds in src/garden.rs

fn main() {
    // We can now use the dummy struct in this example.
    let plant = Asparagus {};
    // Printing is made possible by adding #[derive(Debug)] in the same file as the struct.
    println!("I'm growing {:?}!", plant);
}
