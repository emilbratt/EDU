fn main() {

    // Going over the Option<T> enum that contains "None" and "Some" fields.
    let _some_number = Some(5); // Gets the type Option<i32>.
    let _some_char = Some('e'); // Gets the type Option<char>.
    let _absent_number: Option<i32> = None; // Gets the type that is annotated Option<i32>, but holds None for now.

    // Some value: we know that a value is present and the value is held within the Some.
    // None value: in some sense it means the same thing as null (not a valid value).

    // You have to convert an Option<T> to a T before you can perform T operations with it. (T = type).
    // In general, in order to use an Option<T> value, you want to have code that will handle each variant.
    // You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T.

    // Example using match to handle the option type.
    let _six = plus_one(Some(5));
    println!("Result: {:?}", _six);
    let _none = plus_one(None);
    println!("Result: {:?}", _none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // if None -> return None
        Some(i) => Some(i + 1), // if Some(i) -> return i + 1
    }
}
