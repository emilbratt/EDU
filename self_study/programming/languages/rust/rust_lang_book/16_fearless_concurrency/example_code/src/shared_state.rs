// https://doc.rust-lang.org/book/ch16-03-shared-state.html#the-api-of-mutext

/*
Mutexes have a reputation for being difficult to use because you have to remember two rules:

    1. You must attempt to acquire the lock before using the data.

    2. When you’re done with the data that the mutex guards,
       you must unlock the data so other threads can acquire the lock.

*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    // Count from 10 to 10 via multiple threads by sharing memory.
    let start_number: i32 = 10;
    let counter = Arc::new(Mutex::new(start_number));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // acquire the mutex’s lock

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
