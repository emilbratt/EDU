// https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#while-let-conditional-loops

pub fn run() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Using a while let loop to print values for as long as stack.pop() returns Some
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
