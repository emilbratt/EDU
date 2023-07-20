
pub fn example_1() {
    println!("vector::example_1()\n");

    // Create a vector with no values annotated as Vec<T> using Vec::new()
    let v: Vec<i32> = Vec::new(); // will hold elements of the i32 type

    // Create a vector with values using the vec! macro
    let v = vec![1, 2, 3]; // Rust infer i32 as Vec<i32>, because i32 is default for integers

    // Create mutable vector and push values to it
    let mut v = Vec::new();
    // we did not annotate above, but Rust infers the type when we push numbers to it
    v.push(1);
    v.push(2);
}
