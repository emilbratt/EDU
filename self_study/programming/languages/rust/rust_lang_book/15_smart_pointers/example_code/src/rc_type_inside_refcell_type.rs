// Rc<T> lets you have multiple owners of some data,
// but it only gives immutable access to that data.
// An Rc<T> that holds a RefCell<T> gets you a value
// that can have multiple owners that can mutate.

// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt

use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn run() {
    println!("\n\nrc_type_inside_refcell_type.rs\n");

    let some_value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&some_value), Rc::new(Nil)));

    // b and c both refer to a 
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // deref some_value and add 10
    *some_value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
