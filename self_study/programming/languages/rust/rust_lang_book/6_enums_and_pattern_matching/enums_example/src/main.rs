// Various examples of enums and how they are put into use in Rust.

fn struct_with_enums() {
    // Declare a struct
    struct IpAddr {
        kind: IpAddrKind, // this is an enum type.. declared below this block and can hold one of two values
        address: String,
    }
    // .. and add an enum for IpAddr.IpAddrKind used in the struct.
    enum IpAddrKind {
        V4,
        V6,
    }

    // Instantiate the IpAddr structs for both IP versions.
    let local_v4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let local_v6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn enum_with_enum_pattern() {
    #[derive(Debug)] // for printing out using {:?}

    enum UsState {
        Alabama,
        Alaska,
        // ..
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), // Quarters come with designated UsState symbols, we add an enum for this.
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn catch_all_patterns() {

    fn dice_roll(dice: i8) {

        fn add_fancy_hat() {
            println!("add fancy hat");
        }

        fn remove_fancy_hat() {
            println!("remove fancy hat");
        }

        fn reroll() {
            println!("re roll please");
        }

        match dice {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }
    }

    dice_roll(9);
    dice_roll(7);
    dice_roll(3);
}

fn do_nothing_example() {
    let some_val: i8 = 40;
    match some_val {
        20 => println!("some_val is 20"),
        30 => println!("some_val is 40"),
        _ => (), // Here, were telling Rust explicitly that we arent going to use any other value.
                 // E.g. ..we dont want to run any code in this case.
    }
}

fn if_let_example() {
    #[derive(Debug)] // for printing out using {:?}

    // instead of using a match statement, this syntax can shorten the code if only one case has logic.
    enum Color {
        Yellow,
        Blue,
    }

    enum Car {
        Toyota,
        Honda,
        Porche(Color),
    }

    fn check_car(car: Car) {
        if let Car::Porche(clr) = car {
            println!("The color of the car is {:?}", clr);
        } else {
            println!("Car has no colour");
        }
    }

    let car_1 = Car::Porche(Color::Yellow);
    let car_2 = Car::Toyota;
    check_car(car_1);
    check_car(car_2);
}

fn main() {
    struct_with_enums();
    enum_with_enum_pattern();
    catch_all_patterns();
    do_nothing_example();
    if_let_example();
}
