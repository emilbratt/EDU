// https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html#advanced-functions-and-closures

fn example_function_pointer() {
    // https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html#function-pointers

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // Using the fn type to accept a function pointer as an argument
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    // running example
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

#[allow(unused_variables)]
fn either_closure_or_function() {
    // https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html#returning-closures

    // example where you could use either a closure defined inline or a named function

    // pass closure as argument to map
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    // pass named function as argument to map
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
}

#[allow(unused_variables)]
fn returning_closures() {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    let returned_closure = returns_closure();
}

pub fn run() {
    example_function_pointer();
    either_closure_or_function();
    returning_closures();
}
