/*
 * Generic types in Enums
 *
 * NOTE: Passing the value to the enum will make it so the generic types
 *       become the correct primitive type when compiled.
 *       This is called "type coercion"
 *       e.g. ..forcing a type based on what the language imply it should be.
 */


// Enum that takes a generic type T.
// It also implements trait -> std::cmp::Ord on it (limits the type) see: https://doc.rust-lang.org/std/cmp/trait.Ord.html.
enum SomeEnum<T:std::cmp::Ord > {
  Infinite,
  SomeValue(T),
}

// example loading a number value
fn number_example() {
  let some_num: SomeEnum::<u64> = SomeEnum::SomeValue(10);
  match some_num {
    SomeEnum::Infinite => {
      println!("num is ... nothign?");
    }
    SomeEnum::SomeValue(n) => {
      println!("num is a real number {}", n);
    }
  
  }
  // BTW:  we can also just use if let block to shorten the syntax instead of match
  if let SomeEnum::SomeValue(value) = some_num {
    println!("some_num is: {}", value)
  }
}

// example loading a string slice value
fn string_slice_example() {
  let some_string_slice: SomeEnum::<&str> = SomeEnum::SomeValue("a");
  if let SomeEnum::SomeValue(value) = some_string_slice {
    println!("some_string_slice is: {}", value)
  }
}

// example loading a character value
fn character_example() {
  let some_char: SomeEnum::<char> = SomeEnum::SomeValue('a');
  if let SomeEnum::SomeValue(value) = some_char {
    println!("some_char is: {}", value)
  }
}

// example loading a string value
fn string_example() {
  let some_string: SomeEnum::<String> = SomeEnum::SomeValue(String::from("hello"));
  if let SomeEnum::SomeValue(value) = some_string {
    println!("some_string is: {}", value)
  }
}

pub fn run_example() {
  number_example();
  string_slice_example();
  character_example();
  string_example();
}
