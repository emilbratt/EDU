// Rust does include functionality to print out debugging information,
// ..we have to explicitly opt in to make that functionality available.
// We use derive(debug) decleration.
#[derive(Debug)]
// The structs can now be printed out easily, as the one below
struct Fruits {
    apples: u32,
    bananas: u32,
    oranges: u32,
}

fn main() {
    let f = Fruits {
        apples: 10,
        bananas: 4,
        oranges: 8,
    };
    // Testing out the printing of the struct in this easy example.
    println!("The :? examle\n{:?}\n", f);
    println!("The :#? example\n{:#?}\n", f);

    // We can also use the debug macro provided - also needs the #[derive(Debug)] declaration
    dbg!(&f);

    // this is an example of how the dbg! macro works - see output from executable
    dbg!(30 * 30);
}
