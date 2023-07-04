struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_two_params(width: u32, height: u32) {
    println!("area_with_two_params = {}", width*height);
}

fn area_with_tuple(dimensions: (u32, u32)) {
    println!("area_with_tuple = {}", dimensions.0 * dimensions.1);
}

fn area_with_a_struct(rect: Rectangle) {
    println!("area_with_a_struct = {}", rect.width * rect.height);
}

fn main () {
    
    // BAD IMPLEMENTATION
    let width: u32 = 5;
    let height: u32 = 10;
    area_with_two_params(width, height);
    
    // BAD IMPLEMENTATION
    let rect = (8, 4);
    area_with_tuple(rect);
    
    // GOOD IMPLEMENTATION
    let rect = Rectangle {
        width: 12,
        height: 2,
    };
    area_with_a_struct(rect);
}
