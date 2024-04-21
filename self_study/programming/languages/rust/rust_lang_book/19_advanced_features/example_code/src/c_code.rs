
// Declaring extern function defined in a another language (C's ABI in this case).
extern "C" {
    // The "C" part defines which application binary interface (ABI)
    // the external function uses: the ABI defines how to call the function at the assembly level.
    // The "C" ABI is the most common and follows the C programming language’s ABI.

    // C's abs function.
    fn abs(input: i32) -> i32;

    // Note: We can also use extern to create an interface that allows other languages to call Rust functions.
    // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-rust-functions-from-other-languages
}

pub fn run() {
    // Calling an extern function defined in the C language
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
