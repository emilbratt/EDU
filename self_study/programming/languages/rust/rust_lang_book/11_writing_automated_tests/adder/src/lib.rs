#![allow(dead_code)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn multiply_with_zero(a: i32) -> i32 {
    a * 0
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_function() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn react_a_can_hold_rect_b() {
        let rect_a = Rectangle {
            width: 3,
            height: 3,
        };
        let rect_b = Rectangle {
            width: 2,
            height: 2,
        };
        assert!(rect_a.can_hold(&rect_b));
    }

    #[test]
    fn test_add_two() {
        let result = add_two(1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_multiply_with_zero() {
        let n: i32 = 5;
        let result = multiply_with_zero(n);
        assert_ne!(n, result);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic]
    fn invalid_guess() {
        Guess::new(200);
    }

    // this is a re-written version of the test: test_add_function()
    // https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests
    // It has now the Result<(), String> return type and we can use it in the body..
    #[test]
    fn test_add_function_v2() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }

}

/*
    assert macros in Rust

    check if true.
        assert!(value, optional_custom_message_on_failure);
    assert equal.
        assert_eq!(value_a, value_b, optional_custom_message_on_failure);
    assert difference.
        assert_ne!(value_a, value_b, optional_custom_message_on_failure);
*/
