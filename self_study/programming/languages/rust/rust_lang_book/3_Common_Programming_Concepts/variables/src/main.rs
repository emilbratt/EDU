fn main() {
    behavior();
    numbers();
}

fn numbers() {
    floating_points();
    math_operations();
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

fn floating_points() {
    // floating-point types are f32 and f64, f64 = default
    println!("\nFLOATING POINTS");
    let x = 2.0; // f64
    println!("The f64 value of x is: {x}");
    let y: f32 = 3.0; // f32
    println!("The f32 value of y is: {y}");
    let z = x+y;
    println!("The value of z (x + y) is: {z}");

}

fn behavior() {
    mutable_variables();
    immutable_variable();
    constants();
}

// CHANGE MUTABLE VARIABLE
fn mutable_variables() {
    println!("\nCHANGING A VARIABLE (MUTABLE)");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

}

// SHADOW IMMUTABLE VARIABLE
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

// DECLARE CONSTANTS
fn constants() {
    println!("\nDECLARE CONSTANT");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // will compute only at compile this way when executed,
    // 60 * 60 * 3 will never be calculated, result is instead stored forever
    println!("constant is: {THREE_HOURS_IN_SECONDS}");
}
