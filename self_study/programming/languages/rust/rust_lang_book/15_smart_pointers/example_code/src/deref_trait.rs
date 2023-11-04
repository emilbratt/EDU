fn simple_dereference_example() {
    let x = 5;
    let y = &x;

    // REFERENCE (& operator)
    assert_eq!(y, &x); // reference x because y is a reference
    assert_eq!(y, &5); // reference 5 because y is a reference
    assert_ne!(y, &3); // reference 3 because y is a reference (not equal numbers though)

    // DEREFERENCE (* operator) we dereference y so we have access to the integer that y points to
    assert_eq!(*y, x); // dereference y and comparing with x, not &x
    assert_eq!(*y, 5); // dereference y and comparing 5
    assert_ne!(*y, 3); // dereference y and comparing 3

    // WHY IS THIS?
    // Because comparing a number and a reference to a number is not allowed!
    // They are different types..
    // We use the dereference operator to follow the reference to the value it’s pointing to.

    // When calling: *y => Rust actually calls: *(y.deref()).
    // Also, we would need to bring Deref into scope => use std::ops::Deref;
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    /*
     * Without the Deref trait, the compiler can only dereference &'references.
     * The deref method gives the compiler the ability to take a value of any type
     * that implements Deref and call the deref method to get a & reference that
     * it knows how to dereference.
     */
    type Target = T; // defines associated type for the Deref trait (more about this in ch. 19)

    fn deref(&self) -> &Self::Target {
        &self.0 // .0 accesses the first value in a tuple struct
    }
}

#[derive(Debug)] // Adding `#[derive(Debug)]` to `MyBox` so we can easily print out the values.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello_with_reference(name: &str) {
    println!("Hello, {name}!");
}

pub fn run() {
    println!("\n\nderef_trait.rs\n");

    // showing the dereference behaviour
    simple_dereference_example();

    // our own deref implementation
    let x = MyBox::new(5);
    println!("{:?}", x);
    assert_eq!(5, *x);
    println!("smart pointer");

    // deref coercion: Rust uses multiple deref calls on m going from &MyBox<String> => &String => &str
    // this way, even though the function takes &str, we can pass &MyBox<String>
    let m = MyBox::new(String::from("Rust Lang deref coercion"));
    hello_with_reference(&m);
}
