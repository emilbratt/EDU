use std::thread;
use std::time::Duration;


// ..using-move-closures-with-threads.
fn move_example() {
    let v = vec![1, 2, 3];

    // Moving 'v' into a new thread with "move", this also means main thread can not use 'v' anymore.
    let handle = thread::spawn(move || {
        println!("Hi, from spawned thread");
        println!("This is the vector I got from the 'move' closure");
        println!("{:?}", v); // Using 'v'.
    });

    handle.join().unwrap();
}

// ..waiting for all threads to finish using join handles
fn finish_thread_example_before_last_loop_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    // This block runs after the handle is done because we called join().unwrap().
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn run() {
    move_example();
    finish_thread_example_before_last_loop_example();
}
