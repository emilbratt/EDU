#[derive(Debug)]

// Creating struct Rectangle..
struct Rectangle {
    width: u32,
    height: u32,
}
// ..then implementing methods on Rectangle.
impl Rectangle {
    // Get area of rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Rectangle has width.
    fn has_width(&self) -> bool {
        self.width > 0
    }

    // Can the current instances rectangle hold another instances rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main () {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.has_width() {
        println!("The rectangle has a width; it is {}", rect1.width);
    } else {
        println!("The rectangle has a no width because it is set to 0");
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    if rect1.can_hold(&rect2) {
        println!("yes, rect1 can hold rect2");
    } else {
        println!("no, rect1 can not hold rect2");
    }
    if rect1.can_hold(&rect3) {
        println!("yes, rect1 can hold rect3");
    } else {
        println!("no, rect1 can not hold rect3");
    }
}
