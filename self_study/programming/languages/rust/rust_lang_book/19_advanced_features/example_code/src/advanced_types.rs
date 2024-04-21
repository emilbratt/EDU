#[allow(dead_code)]
fn bar() -> ! {
    // https://doc.rust-lang.org/book/ch19-04-advanced-types.html#the-never-type-that-never-returns
    // panic!() returns '!' type.
    panic!();
}

// DST - dynamically sized type
#[allow(dead_code)]
#[allow(unused_variables)]
fn example_dst() {
    // Rust provides the Sized trait to determine whether or not a type’s size is known at compile time.
    // This trait is automatically implemented for everything whose size is known at compile time.
    // In addition, Rust implicitly adds a bound on Sized to every generic function.

    // A a generic function definition like this:
    fn generic<T>(t: T) {
        // ...
    }

    // is actually treated as though we had written this: (use different name: treated_as_generic)
    fn treated_as_generic<T: Sized>(t: T) {
        // ...
    }
}
pub fn run() {
    println!("adv. types");
    // uncomment to run (will halt program)
    // bar();

    example_dst();
}
