// constants can be declared globally
const X: u32 = 5;
const Y: &str = "fiiiive";

fn main() {
    println!("const X is {X}");
    println!("const Y is {Y}");
    immutable_string_literal();
    mutable_string_type();
}

fn immutable_string_literal() {
    // string literal is
    // ..not mutable
    // ..hardcoded into compiled binary
    // ..very performant
    let immutable_string: &str = "an immutable string";
    println!("{immutable_string}");
}

fn mutable_string_type() {
    // String type is
    // ..mutable
    // ..not as performant as a string literal
    // ..allocated on the heap
    // ..able to store an amount of text unknown at compile time
    // ..not hardcoded into compiled binary
    // ..created with the String::from("..") function
    let mut mutable_string = String::from("a mutable string");
    mutable_string.push_str(", can be changed!");
    println!("{mutable_string}");
}
