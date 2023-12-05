use std::thread;

pub fn run() {
    let v = vec![1, 2, 3];

    // Moving a value into a new thread with "move", main thread wont use it anymore..
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
