use std::slice;

unsafe fn dangerous() {
    // Do something unsafe..
}

#[allow(dead_code)]
// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#creating-a-safe-abstraction-over-unsafe-code
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // This functino can be called from outside unsafe block e.g. it is safe.
    // Using unsafe code in the implementation of the split_at_mut function

    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn run() {

    unsafe {
        // We must call the dangerous function within a separate unsafe block.
        // If we try to call dangerous without the unsafe block, we’ll get an error:
        dangerous();
    }
}
