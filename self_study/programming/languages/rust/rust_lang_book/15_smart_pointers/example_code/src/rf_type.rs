// Rc<T> allows you to share data between multiple parts of your program via references.
// NOTE: for reading only (immutable references).
// E.g. it allows a single value to have multiple owners.
// It also ensures that the value remains valid as long as any of the owners still exist.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// bring List into scope
use self::List::{Cons, Nil};

// bring Rc<T> into scope
use std::rc::Rc;


pub fn run() {

    // A list holding 5 and 10 stored in a new Rc<List>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("reference count after creating a = {}", Rc::strong_count(&a));

    // When we create b and c, we call the Rc::clone function
    // and pass a reference to the Rc<List> in 'a' as an argument.
    let b = Cons(3, Rc::clone(&a));
    println!("reference count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("reference count after creating c = {}", Rc::strong_count(&a));
    }
    println!("reference count after c goes out of scope = {}", Rc::strong_count(&a));
}
