fn main() {
    variables();
    scalar_types();
    compound_types();
    math_operations();
}

fn variables() {
    mutable_variables();
    immutable_variable();
    constants();
}

fn scalar_types() {
    // scalar types are single value variabes
    type_integer();
    type_floating_point();
    type_boolean();
    type_chars();
}

fn compound_types() {
    // compount types are multi value variables
    type_tuple();
    type_array();
}

fn mutable_variables() {
    println!("\nCHANGING A VARIABLE (MUTABLE)");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn immutable_variable() {
    // shadow a variable
    println!("\nSHADOW VARIABLE (IMMUTABLE)");
    let y = 5;
    println!("The value of y is: {y}");
    let y = y + 1;
    println!("The value of y after shadowing is: {y}");
    {
        // shadow within scope
        let y = y + 1;
        println!("The value of y shadowed in the inner scope is: {y}");
        // after this, y value will return back to 6
    }
    println!("..but the value of y after is still the same: {y}");

    // shadow type change
    println!("\nSHADOW VARIABLE WITH NEW TYPE");
    let z = "   ";
    println!("z is and empty string with spaces: {z}");
    let z = z.len();
    println!("but now z is an int: {z}");
}

fn constants() {
    println!("\nDECLARE CONSTANT");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // will compute only at compile this way when executed,
    // 60 * 60 * 3 will never be calculated, result is instead stored forever
    println!("constant is: {THREE_HOURS_IN_SECONDS}");
}


fn type_integer() {
    println!("\nINTEGER VALUES");
    let x: u32 = 5;
    println!("The unsigned u32 value of x is: {x}");
    let y: i32 = -5;
    println!("The signed i32 value of y is: {y}");
}

fn type_floating_point() {
    // floating-point types are f32 and f64, f64 = default
    println!("\nFLOATING POINT VALUES");

    let x = 2.0; // f64
    println!("The f64 value of x is: {x}");
    let y: f32 = 3.0; // f32
    println!("The f32 value of y is: {y}");
    let z = x+y;
    println!("The value of z (x + y) is: {z}");
}

fn type_boolean() {
    // booleans take up exactly 1 byte in memory
    println!("\nBOOLEAN VALUES");

    let x: bool = true;
    println!("The value of x is = {x}");

    let y: bool = false;
    println!("The value of y is = {y}");
}

fn type_chars() {
    // char literals takes up 4 bytes (Unicode Scalar Value) and must be declared inside SINLGE QOUTES '
    println!("\nCHAR VALUES");
    let x: char = 'y';
    let y: char = 'o';
    println!("The value of concatinating x and y is {x}{y}");
    let emoticon: char = '😻';
    println!("Char literals are unicode (not just ASCII) and therefore, emojis can be stored like this {emoticon}");
}

fn type_tuple() {
    // tuples are considered a single compound element and binds to one variable
    println!("\nTUPLE VALUES");
    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value assigned from tuple to x, y and z is: {x}{y}{z}");

    let annotated_tuple: (i32, f64, u8) = (500, 6.4, 1);
    let x = annotated_tuple.0;
    let y = annotated_tuple.1;
    let z = annotated_tuple.2;
    println!("The value assigned from annotated_tuple to x, y and z is: {x}{y}{z}");
}

fn type_array() {
    // all values in arrays must be of same type and they store a fixed amount of elements
    println!("\nARRAY VALUES");

    let array: [&str; 12] = [
        "January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December"
    ];
    let x: &str = array[0];
    println!("First month extracted from array into x is {x}");

    let annotated_array: [i32; 5] = [1, 2, 3, 4, 5];
    let y: i32 = annotated_array[3];

    println!("The 3rd value extracted from annotated_array into y is {y}");
}

fn math_operations() {
    println!("\nMATH OPERATIONS");
    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("product: {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("qoutient: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("truncated: {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");
}
