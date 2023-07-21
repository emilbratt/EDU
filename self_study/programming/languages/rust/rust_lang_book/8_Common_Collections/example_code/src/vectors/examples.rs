use rand::Rng;

pub fn declare_vectors() {
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

pub fn access_value_from_vectors() {
    // we can access a value using either "index" or "get"

    // Index: if out of range -> cause the program to panic as it references a nonexistent element.
    // Get: if out of range   -> return None so we can handle the error gracefully without crashing.

    let v = vec![1, 2, 3, 4, 5];

    // trying to access index 100 using index -> will crash if uncommented
    // let does_not_exist = &v[100];

    // trying to access index 100 using get -> will not crash if uncommented
    // let does_not_exist = v.get(100);
}

pub fn borrowing_and_vectors() {
    // You cant have mutable and immutable references in the same scope.
    // When the program has a valid reference,
    // the borrow checker enforces the ownership
    // and borrowing rules to ensure valid references..
    // Lets declare a vector to show this.
    let mut v = vec![1, 2, 3, 4, 5];

    // immutable borrow occurs when we reference first number in "v"
    let var_first_number = &v[0];

    // mutable borrow occurs when we change "v"
    v.push(6); // ..var_first_number is not valid anymore, we changed the vector

    // ..the immutable borrow "var_first_number" is no longer valid (can not be used)

}

pub fn iterating_vectors() {
    // loop over immutable references to each element
    let v = vec![100, 32, 57];
    for i in &v {
        println!("immutable reference \"i\" is {i}");
    }

    // loop over mutable references to each element and change them usng dereference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("mutable reference \"i\" before change is {i}");
        *i += 50; // dereference operator * so we can get the direct value i and then change it
        println!("mutable reference \"i\" after  change is {i}");
    }
}

pub fn enums_in_vectors() {
    // Vectors can only store values that are the same type.
    // Enums can hold multiple types, so we store enums in the vector.

    // Using an enum plus a match expression means that Rust will
    // ensure at compile timethat every possible case is handled.

    // Create an enum to hold numbers in different forms, like in a spreadsheet..
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // The vector can then represent a row in this case..
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Using enums will also mean that we must handle all type cases..
    // Lets pick a random index using rand
    let index: usize = rand::thread_rng().gen_range(0..=2); // will be 0, 1 or 2
    // ..and reference the currently unknown element using the index
    let referenced_element: &SpreadsheetCell = &row[index];
    // ..and match the type and apply logic thereafter
    match referenced_element {
        SpreadsheetCell::Int(_) => {
            println!("It is int, lets to int stuff!");
        }
        SpreadsheetCell::Text(_) => {
            println!("It is text, lets do text stuff!");
        }
        SpreadsheetCell::Float(_) => {
            println!("It is float, lets float on!");
        }

    }

}

pub fn scopes_and_vectors() {
    // Follows the same idea as any other type in Rust.
    // The borrow checker will ensure that any references to contents
    // of a vector are only used while the vector itself is valid

    let v_in_parent = vec![1, 2, 3, 4];

    { // start of a scope..
        let v_in_child = vec![1, 2, 3, 4];
        // do stuff with v....
        // ...
        // ...
    } // ..end of scope -> v_in_child gets dropped

    // however.. v_in_parent is still valid
}
