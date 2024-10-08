use std::thread::{self, JoinHandle};
use std::time::Duration;

// Waiting for All Threads to Finish Using join Handles.
pub fn run() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
