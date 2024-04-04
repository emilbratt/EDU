use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// The idea is to create a channel with 'mpsc', a transmitter and a receiver.
// The transmitter is moved into a spawned thread and the receiver remains.
// The receiver can receive messages from multiple threads.
// This is the idea behind mpsc which stands for multiple producer, single consumer.

// Sending Value and Seeing the Receiver.
fn send_a_value() {
    // Create a new channel so that we can communicate.
    let (tx, rx) = mpsc::channel();
    // mpsc: multiple producer, single consumer
    // ..it returns a tuple (tx, rx)
    // ..first element is the the transmitter
    // ..the second element is the receiver

    thread::spawn(move || {
        let val = String::from("sending this value from the spawned thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Sending Multiple Values and Seeing the Receiver Waiting
fn send_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("sending"),
            String::from("multiple"),
            String::from("values"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in vals {
            // We moved the transmitting end 'tx' into this thread.
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Using the receiver to receive message from the thread.
    for received in rx {
        println!("Got: {}", received);
    }
}

fn send_multiple_values_from_multiple_threads() {
    // This is where "multiple producer, single consumer" really comes into play.

    let (tx, rx) = mpsc::channel();

    // Clone and get a new transmitter 'tx1' that we pass to the first spawned thread.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("first thread A"),
            String::from("first thread B"),
            String::from("first thread C"),
            String::from("first thread D"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Then we pass the original transmitter 'tx' to a second spawned thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("second thread A"),
            String::from("second thread B"),
            String::from("second thread C"),
            String::from("second thread D"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn run() {
    send_a_value();
    send_multiple_values();
    send_multiple_values_from_multiple_threads();
}