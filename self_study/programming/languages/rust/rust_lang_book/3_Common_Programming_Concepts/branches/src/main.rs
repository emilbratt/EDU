fn main() {
    if_condition_1();
    if_condition_2();
    if_condition_3();
    loop_1();
    loop_2();
    while_loop_1();
    while_loop_2();
    for_loop_1();
}

fn if_condition_1() {
    println!("\nif_condition_1()");

    let number: u32 = 5;

    if number < 5 {
        println!("number is less than 5");
    } else if number > 5 {
        println!("number is greater than 5");
    } else {
        println!("this is the else block, so number must be 5");
    }
}

fn if_condition_2() {
    println!("\nif_condition_2()");

    let number: u32 = 5;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn if_condition_3() {
    // Using if in a let Statement
    println!("\nif_condition_3()");
    let condition = true;
    let number: i32 = if condition { -5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_1() {
    println!("\nloop_1()");

    let mut counter: u32 = 0;

    let result: u32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_2() {
    println!("\nloop_2()");

    let mut count: u32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: u32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop_1() {
    println!("\nwhile_loop_1()");
    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_loop_2() {
    println!("\nwhile_loop_2()");

    let some_array:[u32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", some_array[index]);

        index += 1;
    }   
}

fn for_loop_1() {
    println!("\nfor_loop_1()");

    let some_array: [u32; 5] = [10, 20, 30, 40, 50];

    for element in some_array {
        println!("the value is: {element}");
    }
}
