fn simple_dereference_example() {
    // lets first demonstrate the reference and the deref trait
    let x = 5;
    let y = &x;

    // REFERENCE (& operator)
    assert_eq!(y, &x); // reference x because y is a reference
    assert_eq!(y, &5); // reference 5 because y is a reference
    assert_ne!(y, &3); // reference 3 because y is a reference (not equal numbers though)

    // DEREFERENCE (* operator) we dereference y so we have access to the integer that y points to
    assert_eq!(*y, x); // dereference y and comparing with x, not &x
    assert_eq!(*y, 5); // dereference y and comparing 5
}

pub fn run() {
    simple_dereference_example();
}
