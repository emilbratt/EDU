// https://doc.rust-lang.org/book/ch16-03-shared-state.html#shared-state-concurrency

use std::sync::Mutex;
pub fn run() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
