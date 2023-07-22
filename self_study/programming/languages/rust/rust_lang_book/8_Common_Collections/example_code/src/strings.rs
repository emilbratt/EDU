pub mod examples; // ..declare src/vectors/examples.rs
/*
The String type is a growable, mutable, owned, UTF-8 encoded string type
Not to be confused with string slice -> &str
Alltough both types are used heavily in Rusts standard library..
String and string slices are both UTF-8 encoded.

A String is a wrapper over a Vec<u8> type.
*/
pub fn run() {
    println!("\ncalling strings::examples::declare_strings()");
    examples::declare_strings();

    println!("\ncalling strings::examples::strins_are_utf8()");
    examples::strins_are_utf8();

    println!("\ncalling strings::examples::append_to_strings()");
    examples::append_to_strings();

    println!("\ncalling strings::examples::combine_strings()");
    examples::combine_strings();

    println!("\ncalling strings::examples::indexing_into_strings()");
    examples::indexing_into_strings();
}
