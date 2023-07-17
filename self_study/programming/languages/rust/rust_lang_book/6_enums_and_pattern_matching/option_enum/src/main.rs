fn main() {

    // Going over the Option<T> enum that contains "None" and "Some" fields.
    let some_number = Some(5); // Gets the type Option<i32>.
    let some_char = Some('e'); // Gets the type Option<char>.
    let absent_number: Option<i32> = None; // Gets the type that is annotated Option<i32>, but holds None for now.

    // Some value: we know that a value is present and the value is held within the Some.
    // None value: in some sense it means the same thing as null (not a valid value).

    // You have to convert an Option<T> to a T before you can perform T operations with it. (T = type).
    // In general, in order to use an Option<T> value, you want to have code that will handle each variant.
    // You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T.
}
