pub fn main() {
    let bar = Bar { a: 1, b: String::from("hello") };
    println!("{}", bar.b);
}

// Type alignmnet in memory..
// Enforce type alignment for compatibility with C. This will introduce padding of bytes.
#[repr(C)]
struct Foo {
    tiny: bool, // bool will byte align as it is the first value..
    normal: u32, // 4 byte type -> compiler inserts 3 bytes of padding after tiny's 1 byte, to align correctly..
    small: u8, // and so one..
    long: u64, // ..
    short: u16, // ..
}

// Option for larger alignment (can improve performance when store in for example an array)
#[repr(align(2))]
struct Bar {
    a: u32,
    b: String,
}
// Common use case to ensure different values stored contigously in memory
// end up in different cache lines on the CPU. Can improve performance.


// VTable
// Every trait object holds a vtable. A vtable is a dispatch table
// which during run-time, refers to the correct function for that specific type.
