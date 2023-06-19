// constants can be declared globally
const X: u32 = 5;
const Y: &str = "fiiiive";

fn main() {
    println!("const X is {X}");
    println!("const Y is {Y}");
    immutable_string_literal();
    mutable_string_type();
    heap_and_stack_example();

    // REFERENCE AND BORROWING
    let mut some_string = String::from("some string");
    // borrow: sending the variable to a function (we need the variable back for re-using it)
    some_string = borrow_variable(some_string);
    // reference: sending a pointer-like variable (we do not need the variable back for re-using it)
    reference_variable(&some_string); // just append "&" in front of variable
    // here, we ar ere-using it without having it sent back
    modify_reference_not_allowed_by_default(&some_string);
    modify_reference_allowed_with_mut(&mut some_string);
    println!("{some_string}");
    multiple_references_and_scopes();

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
    // ints are small
    // ..so rust stores them on the stack because compiler knows the exact size at compile time
    let stack_variables_can_be_cloned_easily: u32 = var_on_stack;
    println!("this variable is stored on the stack and has the value {var_on_stack}");
    println!("this cloned variable is stored on the stack and has the value {stack_variables_can_be_cloned_easily}");

    // HEAP
    // strings are large
    // ..so rust stores them on the heap because compiler can not know the exact size at compile time
    let var_on_heap = String::from("hello from the heap memory");
    println!("this variable is stored on the heap and has the value {var_on_heap}");
    let var_on_heap_must_be_moved_instead = var_on_heap;
    // rust has now invalidated var_on_heap as it is now assigned to var_on_heap_must_be_moved_instead
    println!("this moved variable is stored on the heap and has the value {var_on_heap_must_be_moved_instead}");
}

fn borrow_variable(var: String) -> String {
    println!("{var}");
    var
    // rust is expression based, so we can (should) omit return key word when sending back
}

fn reference_variable(var: &String) { // append "&" in front of annotation
    println!("{var}");
    // Functions that takes references as params never gets ownership..
    // so we do not need to "send back" the value to keep using it.
}

fn modify_reference_not_allowed_by_default(var: &String) { // append "&" in front of annotation
    println!("changing a referenced variable is not allowed for this var with value {var}");
    // var.push_str(", adding this is not allowed")
    // Uncommenting the above lin would result in compilation error due to no ownership.
}

fn modify_reference_allowed_with_mut(var: &mut String) { // append "&" in front of annotation
    println!("changing a referenced variable is allowed using mut for this var with value {var}");
    var.push_str(", adding this now allowed") // we have now changed the variable   
}

fn multiple_references_and_scopes() {
    let mut s = String::from("a mutable string");
    // NOT ALLOWED
    // let _r1 = &mut s;
    // let r2 = &mut s;

    // ALLOWED (bacause after last usage in a function it is de-referenced..)
    let r1 = &s;
    println!("{r1} in a local scope");
    let r1 = &mut s; // so we can actually re-reference it as mutable, cool
    println!("{r1} in a local scope");
    let r2 = &s;
    println!("{r2} in a local scope");

    // ALLOWED (because it is in its own scope using curly brackets)
    {
        let _r1 = &mut s;
        // This is fine becase _r1 is in its own scope.
    }
    let r2 = &mut s;
    println!("{r2} in the same scope");
}
