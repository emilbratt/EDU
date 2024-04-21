// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait

unsafe trait Foo {
    // methods go here
}

// By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.
unsafe impl Foo for i32 {
    // method implementations go here
}

pub fn run() {}
