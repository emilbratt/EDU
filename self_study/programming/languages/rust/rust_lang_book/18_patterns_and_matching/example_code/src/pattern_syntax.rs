#![allow(dead_code)]

fn standard_match() {
    let x = Some(1);

    let _ = match x {
        None => None,
        Some(i) => Some({
            println!("{}", i.to_string());
            Some(i + 1);
        }),
    };
}

fn matching_literalls() {
    let y = 1;

    match y {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn function_parameters() {

    // a function f that takes a pattern (deconstructs a tuple).
    fn f(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    f(&point);
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // multiple
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn ranges_of_values() {

    fn with_numbers() {
        let x = 5;

        match x {
            1..=5 => println!("one through five"), // a range from 1 to 5
            _ => println!("something else"),
        }
    }

    fn with_chars() {
        let x = 'c';

        match x {
            'a'..='l' => println!("1st half ASCII letter: '{}'", x), // chars a - l
            'm'..='z' => println!("2nd half ASCII letter: '{}'", x), // chars m - z
            _ => println!("{}", x),
        }
    }

    with_numbers();
    with_chars();
}

fn destructuring_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // Destructuring a struct’s fields into separate variables
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);


    let Point { x, y } = p; // shorthand version
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // Matching on nested enums
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

fn ignoring_parts_of_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // Using an underscore within patterns that match Some variants when we don’t need to use the value inside the Some
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    // Ignoring multiple parts of a tuple
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

fn ignoring_all_but_one_part_of_value() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    // Ignoring all fields of a Point except for x by using ..
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn match_guards() {
    let num = Some(4);

    // Adding a match guard to a pattern
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }


    let x = Some(5);
    let y = 10;

    // test for equality with an outer variable
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    // Combining multiple patterns with a match guard
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn test_and_bind_to_value() {
    // The '@' operator lets us create a variable that holds a value while
    // at the same time allows testing that value for a pattern match.

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    // Using @ to bind to a value in a pattern while also testing it
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

pub fn run() {
    standard_match();
    matching_literalls();
    function_parameters();
    matching_named_variables();
    multiple_patterns();
    ranges_of_values();
    destructuring_structs();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    ignoring_parts_of_value();
    ignoring_all_but_one_part_of_value();
    match_guards();
    test_and_bind_to_value();
}
