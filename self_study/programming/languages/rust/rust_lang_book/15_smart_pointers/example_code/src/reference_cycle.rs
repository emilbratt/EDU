// Using Rc<T> and RefCell<T>:
// it’s possible to create references where items refer to each other in a cycle.
// This creates memory leaks
// The reference count of each item in the cycle will never reach 0
// ..e.g. the values will never be dropped.


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil, // Nil is the canonical name to denote the base case of the recursion..
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    println!("\n\nreference_cycle.rs\n");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // This is where the overflow on the stack happens.
    // Uncomment the next line to see that we have a reference cycle;
    // println!("a next item = {:?}", a.tail());
}
