// Two functions that differ only in their names and the types in their signatures

// NOTE: the bad example of solving the problem
// finds the largest i32 in a slice
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// finds the largest char in a slice
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// NOTE: the good example of solving the problem
// The functions above have the same code..
// Lets eliminate the duplication using generic type parameter.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    /*
        To parameterize the types in a new single function like this one,
        we need to name the type parameter,
        just as we do for the value parameters to a function.
    
        We use the char "T" as identifier (common in Rust) but could be any character..
        Type name declarations goes inside angle brackets, <>, in the function signature.
        ..function "largest" is generic over some type T
        ..and returns a reference to a value of the same type T
    
        Because we want to compare values of type T in the body,
        we can only use types whose values can be ordered for both i32 and char.
        To enable comparisons, the standard library has the std::cmp::PartialOrd trait
        that we in this case implement on the T type.
        In essence, we restrict the types valid for T to only those that implement PartialOrd.

        We use:
            <T: std::cmp::PartialOrd>
        ..instead of:
            <T>
    */
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn run_example() {
    // This example uses a list of numbers and a list of characters
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    // BAD - run each on their own dedicated function
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // GOOD - run both on the same function
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
