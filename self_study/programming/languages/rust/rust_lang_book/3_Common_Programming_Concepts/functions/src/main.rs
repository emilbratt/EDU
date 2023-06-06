fn main() {
    println!("Hello, world!");

    another_function();
    a_function_with_a_parameter(4);
    print_labeled_measurement(5, 'h');
    let return_value = funciton_with_return_value();
    println!("The value of return_value is: {return_value}");
    let another_return_value = another_funciton_with_return_value(5);
    println!("The value of another_return_value is: {another_return_value}");
}

fn another_function() {
    println!("Another function.");
}

fn a_function_with_a_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn funciton_with_return_value() -> u32 {
    // An expression is contained within curly brackets {}
    // therefore, a function is an expression.
    // We can just leave a value inside it and it will be returned.
    5
    // ..notice how we do not need to end the line with a semi-colon.
}

fn another_funciton_with_return_value(x: u32) -> u32 {
    x + 5
}
