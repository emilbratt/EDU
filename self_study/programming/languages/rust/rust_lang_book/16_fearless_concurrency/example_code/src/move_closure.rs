use std::thread;

pub fn run() {
    let v = vec![1, 2, 3];

    // Moving a value into a new thread with "move" => main thread can not use it anymore
    let handle = thread::spawn(move || {
        println!("Hi, from spawned thread");
        println!("This is the vector I got from the 'move' closure");
        println!("{:?}", v);
    });

    handle.join().unwrap();
}
