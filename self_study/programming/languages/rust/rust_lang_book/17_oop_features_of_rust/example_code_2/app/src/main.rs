// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

// For structure: https://doc.rust-lang.org/rust-by-example/cargo/deps.html

use blog::Post;

fn main() {
    println!("Hello, world!");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

}
