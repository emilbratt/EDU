/*
 * Traits define "rules" for how you can implement methods on structs.
 * Used when you have multiple structs that share similar behavior.
 * Every struct that implements that trait "adheres" to the rules specified in the trait.
 *
 * What are we doing here?
 *   We have created the 2 structs -> NewsArticle and Tweet
 *   Structs are complex datatypes you design on top of primitive types.
 *   Then we implement the traits that have a shared method on these.

 * Implement a shared behavior e.g. a shared method?
 *   On these two "types" we created we want to implement the shared behavior.
 *   However, the same method might need different logic because the structs themselves are different.
 *   The "Trait" solves this having you "define" the method (as a signature or with a body).

 * How?
 *   You create the "template" e.g. a trait. Then add method signatures or actual methods.
 *   Then you implement the trait on a struct.
 *   You add the method body "logic" to each of the structs implementations.
 *   This is done inside impl <trait> for <struct> {..here..}".
 *   e.g. impl Summary for NewsArticle {..logic..}
 *
 * Use this in another file?
 *   Add a line in your rust file similar to this line below.
 *   use crate::path::thisfile::{Summary, NewsArticle, Tweet};
 */


// declaring a trait, we call it "Summary"
pub trait Summary {
    // This is like a template
    // Structs are going to "implement" their "version" of the methods in this body.

    // declare an "abstract implementation" e.g. method signature.
    fn greet(&self) -> String; // after signature comes the semicolon.
    // ..we are not creating a body with functionality within the method, only a template.

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // declaring "default implementation" e.g. a default method (can be overridden..)
    fn greeting(&self) -> String {
        String::from("Hello from Summary")
    }

    // Compiler will enforce that any type that has this trait
    // will have the methods defined the same way as these.
}

// this is the 1st struct we are going to implement the trait Summary on
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// here we are implementing the trait on the 1st struct
impl Summary for NewsArticle {
    // we use the "template" with the same name from the trait and implement our own body
    fn greet(&self) -> String {
        format!("Hi, from {}", self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// this is the 2nd struct we are going to implement the trait Summary on
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// here we are implementing the trait on the 2nd struct
impl Summary for Tweet {
    // we use the "template" with the same name from the trait and implement our own body
    fn greet(&self) -> String {
        format!("Hi, from {}", self.username)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// function that takes a parameter that implements a trait
pub fn notify(item: &impl Summary) {
    // the value passed to this function must be able to call summarize()
    println!("Breaking news! {}", item.summarize());

    // read: https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
}

// function returning a value with a type that implements a trait
fn function_returning_a_type_that_implements_summary() -> impl Summary {
    // the code calling this function doesn't need to know that the return value implements Summary.
    // useful in the context of closures and iterators discussed in chapter 13..
    // mainly it is a way to avoid writing out long verbose types
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


// Using Trait Bounds to Conditionally Implement Methods
// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// conditionally implement cmp_display method if its inner type T implements PartialOrd and Display
impl<T: Display + PartialOrd> Pair<T> {
    // this method will only be implemented for type <T> if
    // ..it also has implemended the traits PartialOrd and Display
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// run examples
pub fn run_example() {
    // use the Tweet struct and add values to it
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // we did not have to add a custom implementation for this method,
    // as we can use the "default" in the trait itself.
    println!("{}", tweet.greeting());

    // using our own implementation found in "impl Summary for Tweet" body
    println!("1 new tweet: {}", tweet.summarize());


    // use the NewsArticle struct and add values to it
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());


    // call a function that returns a type that implements a trait
    let a_value_that_implements_summary_trait = function_returning_a_type_that_implements_summary();
    // we dont do anything with this now, though..
}
