pub fn main() {
    a();
    b();
    c();
    d();
}

fn a() {
    let mut x = Box::new(42);
    let y = &x;
    if true {
        *x = 5;
    } else {
        println!("{}", y);
    }
}

// Listing 1-9 Lifetimes can have holes in them.
fn b() {
    let mut x = Box::new(42);
    let mut z = &x;

    for i in 0..5 {
        x = Box::new(i); // Move out of x, we cannot use z now because it ends the lifetime (x does not exist anymore)..

        // println!("loop z: {}", z); // not allowed because of previous line, thus making a 'hole' until we re-assign z..

        z = &x; // Re-asign reference for x to z, we now CAN use z becuse lifetime is 'restarted'..

        println!("loop z: {}", z); // Allowed, lifetime for z was restarted..
    }
    println!("end z: {}", z);
}

// Listing 1-10 generic over multiple lifetimes
fn c() {
    struct StrSplit<'s, 'p> {
        delimiter: &'p str,
        document: &'s str,
    }
    impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
        type Item = &'s str;
        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }
    fn str_before(s: &str, c: char) -> Option<&str> {
        StrSplit { document: s, delimiter: &c.to_string() }.next()
    }
}

// Lifetime variance
fn d() {
    // Variance describe what types are subtypes of other types.
    // Subtypes can be used in place of their "supertypes".
    // Think of a type Turtle being a subtype of a type Animal.
    // This also applies to lifetimes as they are also 'types'.
    let a: &'static str = "hello world";

    fn f(s: &str) { // &'static is subtype of the elided lifetime of &str, so we can take in its type as well..
        println!("&'static subtype: {}", s);
    }
    f(a);


    // Listing 1-11 - Example of type that needs to be generic over multiple lifetimes.
    struct MutStr<'a, 'b> {
        s: &'a mut &'b str
    }

    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    // Above line is the same as writing..
    // let x = MutStr { s: &mut s } then writing *x.s = "world", except there is no variable and so the MutStr is dropped immediately..

    println!("MutStr: {}", s);
}


// Generic traits.
//  with generic type parameter
trait Foo<T> {
    fn hi() -> T;
}
//  with generic associated type
trait Bar {
    type Baz;
    type Bob;

    fn hello(&self, _: &Self::Baz) -> Self::Bob;
}
// Rationale: use the one with associated types whenever you can,
//            however, it will not allow multiple implementations
