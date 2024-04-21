fn impl_add_trait_for_operator_overload() {
    use std::ops::Add; // Bring the 'Add' trait into scope.

    #[derive(Debug, Copy, Clone, PartialEq)] // Trait bounds that we need.
    struct Point {
        x: i32,
        y: i32,
    }

    // Implementing the Add trait to overload the + operator for Point instances
    impl Add for Point {
        type Output = Point; // the associated type 'Output' for the Add trait, which we assign the type 'Point'

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

fn implement_outline_print() {
    // Using Supertraits to Require One Trait’s Functionality Within Another Trait.
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    use std::fmt;

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // Use the implementation.
    let p = Point { x: 1, y: 3 };
    p.outline_print();

}

fn impl_ext_traits_on_ext_types() {
    // https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types

    // let’s say we want to implement Display on Vec<T>,
    // which the orphan rule prevents us from doing directly
    // because the Display trait and the Vec<T> type are defined outside our crate.

    // Solution: Creating a Wrapper type around Vec<String> to implement Display. See below.
    use std::fmt;

    // We can make a Wrapper struct that holds an instance of Vec<T>;
    // then we can implement Display on Wrapper and use the Vec<T> value, as shown in Listing.
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // Use the implementation.
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

pub fn run() {
    impl_add_trait_for_operator_overload();
    implement_outline_print();
    impl_ext_traits_on_ext_types();
}
