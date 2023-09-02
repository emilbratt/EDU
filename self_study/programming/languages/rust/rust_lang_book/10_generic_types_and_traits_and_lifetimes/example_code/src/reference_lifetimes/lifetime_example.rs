// Every reference have a lifetime.
// The borrow checker need clues to figure out how long a reference's lifetime.
// Therefore, we specify lifetime parameters for functions and structs that use references.

// Though, here is a major exeption called "lifetime elision rules"
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision

/* Overview of the lifetime rules

    Exactly one input lifetime parameters in a function,
    then lifetime is assigned to all output lifetime parameters
    fn foo<'a>(s: &'a str) -> &str { {
        ..
    }

    Multiple input lifetime parameters in a function, 
    then lifetime one for each is assigned for all lifetime parameters
    fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &str {
        ..
    }

    Multiple input lifetime parameters in a method.
    if one of them is &self or &mut self
    the lifetime of self is assigned to all output lifetime parameters
    impl<'a> Foo<'a> {
        fn bar(&self, var: &str) -> &str {
            ..
        }
    }
*/

fn dangling_lifetime() {
    let r;
    {
        // The binding `x`
        let x = 5; // ..is declared here.
        r = &x; // A borrowed value appears here
        // ..and will not live long enough..
    }
    // to be borrowed here..
    // println!("r: {}", r);
}


// &i32        -> a reference
// &'a i32     -> a reference with an explicit lifetime
// &'a mut i32 -> a mutable reference with an explicit lifetime
fn lifetime_annotation_in_function() {
    /*
        The longest function definition <'a> below is
        specifying that all the references in the signature
        must have the same lifetime 'a
    */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}


fn lifetime_annotation_in_struct() {

    struct ImportantExcerpt<'a> { // generic lifetime parameter <'a>
        part: &'a str, // lifetime parameter &'a
    } // ImportantExcerpt can not outlive the reference it holds
    
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // novel does not go out of scope until after the ImportantExcerpt goes out of scope,
    // so the reference in the ImportantExcerpt instance is valid
    println!("{}", i.part);
}

fn static_lifetime() {
    // All string literals have the 'static lifetime, which we can annotate as follows:
    let s: &'static str = "I have a static lifetime.";
    // The value of this string is stored directly in the programs binary.
}


// run examples
pub fn run_example() {
    dangling_lifetime();
    lifetime_annotation_in_function();
    lifetime_annotation_in_struct();
    static_lifetime();
}
