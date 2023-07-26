struct Point<T> {
    x: T,
    y: T,
}

pub fn run_example() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
