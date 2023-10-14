fn box_t_example_1() {
    let b = Box::new(5); // i32 value (using stack by default) now stored on the heap using a box.
    println!("b = {}", b);
    // When the box goes out of scope, as b does at here at the end of main,
    // it will be deallocated for box (stack) and data (heap).
}

use List::{Cons, Nil};
#[derive(Debug)] // Adding `#[derive(Debug)]` to `List` so we can easily print out the values.
enum List {
    // Cons(i32, List),   // List not inside Box.
    Cons(i32, Box<List>), // List inside Box so heap is used instead of stack.
    Nil,

    // By using a box, we’ve broken the infinite, recursive chain of List.
    // The compiler can figure out the size it needs to store a List type.
}

fn box_t_example_2() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}


pub fn run() {
    box_t_example_1();
    box_t_example_2();
}
