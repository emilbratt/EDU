// constants can be declared globally
const X: u32 = 5;
const Y: &str = "fiiiive";

fn main() {
    println!("const X is {X}");
    println!("const Y is {Y}");
    immutable_string_literal();
    mutable_string_type();
    heap_and_stack_example();
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

fn heap_and_stack_example() {
    // the stack is push and pop (fast)
    // the heap is allocation based (not as fast as stack)
    // some datatypes are stored on the stak by default because they are small and have a fixed size
    // other datatypes are stored on the heap because they are dynamically sized and usually larger..

    // STACK
    let var_on_stack: u32 = 5;
    // ints are small and stored on the stack because compiler knows the memory size at compile time
    println!("this variable is stored on the stack and has the value {var_on_stack}");

    // HEAP
    // strings are large and stored on the heap because compiler does not know size at compile time
    let var_on_heap = String::from("hello from the heap memory");
    println!("this variable is stored on the heap and has the value {var_on_heap}");
}
