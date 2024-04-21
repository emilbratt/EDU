fn raw_pointer() {
    let mut num = 5;

    // create an immutable and a mutable "raw pointer" from references
    let _r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;
    // casting an immutable and a mutable reference into their corresponding raw pointer types..

    // Notice that we don’t include the unsafe keyword in this code.
    // We can create raw pointers in safe code;
    // we just can’t dereference raw pointers unless we write it in an unsafe block.
}

fn raw_pointer_arbitrary_mem_addr() {
    let address = 0x012345usize;

    // Creating a raw pointer to an arbitrary memory address
    let _r = address as *const i32;

    #[allow(unused_unsafe)]
    // dereference raw pointers with 'unsafe' keyword
    // ..uncommenting line inside unsafe block will likely panic (it is an uncertain value after all..)
    unsafe {
        // println!("r1 is: {}", *_r);
    }
}

pub fn run() {
    raw_pointer();
    raw_pointer_arbitrary_mem_addr();

    println!("defer raw pointer");
}
