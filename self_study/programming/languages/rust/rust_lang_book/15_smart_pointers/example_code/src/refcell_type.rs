// RefCell.
// Interior mutability is a design pattern in Rust that allows you to mutate data
// even when there are immutable references to that data;

// ChatGPT:
// RefCell<T> is a smart pointer that allows for mutable borrowing at runtime,
// even when the Rust borrow checker would normally disallow it at compile time.
// It's primarily used in situations where you need interior mutability,
// meaning you want to mutate the contents of a value
// even when you only have an immutable reference to it.

// we’ll create a library (lib.rs) that tracks a value against a maximum value and sends messages
// based on how close to the maximum value the current value is.
// This library could for example be used to keep track of a user’s quota
// for the number of API calls they’re allowed to make.

use std::cell::RefCell;

fn simple_example() {
    // immutable borrow
    let x = RefCell::new(vec![1, 2, 3]);

    let borrowed_x = x.borrow();

    println!("Borrowed: {:?}", *borrowed_x); // * = deref borrowed_x

    // mutable borrow
    let y = RefCell::new(vec![1, 2, 3]);

    let mut borrowed_y_mut = y.borrow_mut();
    borrowed_y_mut.push(4);

    println!("borrowed_y_mut: {:?}", *borrowed_y_mut); // * = deref borrowed_y_mut

}


// lib.rs
use example_code::Messenger;
use example_code::LimitTracker;


struct MockMessenger {
    // Using RefCell to allow mutable borrow check at runtime.
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            // Using RefCell to create an empty vector.
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // We can not take a mutable reference here because we have to obey the trait definition.
        // We use RefCell->borrow_mut() to get around this limitation.
        // self.sent_messages.borrow_mut().push(String::from(message));
        let mut borrowed_mutable = self.sent_messages.borrow_mut();
        borrowed_mutable.push(String::from(message));

        // UNCOMMENTING THIS WILL PANIC AT RUNTIME, ONLY ONE MUTABLE BORROW ALLOWED..
        // let mut borrowed_mutable_b = self.sent_messages.borrow_mut();
    }
}

fn advanced_example() {
    // Using our MockMessenger struct which implements the trait Messenger in lib.rs.
    let mock_messenger = MockMessenger::new();

    // Passing our struct to LimitTracker found in lib.rs.
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    // RefCell->borrow() to access vec, we can have multiple immutable borrows in the same scope.
    limit_tracker.set_value(75);
    println!("message: {}", mock_messenger.sent_messages.borrow()[0]); // access 1st message

    limit_tracker.set_value(75);
    println!("message: {}", mock_messenger.sent_messages.borrow()[1]); // access 2nd message

    limit_tracker.set_value(95);
    println!("message: {}", mock_messenger.sent_messages.borrow()[2]); // access 3rd message
}

pub fn run() {
    println!("\n\nrefcell_type.rs\n");

    simple_example();
    advanced_example();
}

/* 
 * Showrt summary
 *
 * Normally when creating immutable and mutable references,
 * we use the & and &mut syntax, respectively.
 *
 * However, with RefCell<T>; we use the borrow() and borrow_mut() methods.
 * The borrow method() returns the smart pointer type Ref<T>.
 * The borrow_mut() returns the smart pointer type RefMut<T>.
 * ..both implement Deref, so we can treat them like regular references.
 *
 * RefCell<T> lets us have many immutable borrows or one mutable borrow.
 */
