pub fn main() {
    a();
}

// Listing 2-1 Same memory layout for types.
fn a() {
    #[repr(C)] // Enforce type alignment for compatibility with C.
    struct Foo {
        tiny: bool,
        normal: u32,
        small: u8,
        long: u64,
        short: u16,
    }
}
