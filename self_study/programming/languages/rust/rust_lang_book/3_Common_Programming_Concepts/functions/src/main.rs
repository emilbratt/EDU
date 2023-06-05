fn main() {
    println!("Hello, world!");

    another_function();
    a_function_with_a_parameter(4);
    print_labeled_measurement(5, 'h');
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
