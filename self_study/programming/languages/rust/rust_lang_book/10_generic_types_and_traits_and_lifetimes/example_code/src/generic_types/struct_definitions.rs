/*
 * Generic types on Structs
 *
 * NOTE: Passing the values to the struct will make it so the generic types
 *       become the correct primitive type when compiled.
 *       This is called "type coercion"
 *       e.g. ..forcing a type based on what the language imply it should be.
 */

// example when using a struct with a single generic type
fn same_type_example() {
    struct SomeStruct<T> {
        x: T,
        y: T,
    }

    let integers = SomeStruct { x: 5, y: 10 };
    let floats = SomeStruct { x: 1.4, y: 4.3 };
}

// example when using a struct with multiple generic types
fn multiple_type_example() {
    struct SomeStruct<T, U> {
        x: T,
        y: U,
    }

    let float_and_integer = SomeStruct { x: 5.5, y: 10 };
    let integer_and_float = SomeStruct { x: 5, y: 10.5 };
}

pub fn run_example() {
    same_type_example();
    multiple_type_example();
}
