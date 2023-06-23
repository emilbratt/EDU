#[allow(dead_code)] // suppress compiler warnings for un-used code

// STANDARD STRUCT (KEY/VAL)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// TUPLE STRUCTS (INDEX/VAL)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// UNIT-LIKE STRUCTS (more on this in chapter 10)
struct AlwaysEqual;


// BUILDER FUNCTION
fn build_user(username: String, email: String) -> User {
    User {
        active: true, // set to true
        username, // field init shorthand - set to the variable passed
        email, // field init shorthand - set to the variable passed
        sign_in_count: 1, // set to 1
    }
}

fn main() {
    // INTERACTING WITH STRUCT DIRECTLY
    let mut user_1 = User { // NOTE: in a struct, either all are mutable or all are immutable
        active: true,
        username: String::from("bobby1"),
        email: String::from("bobby1@gmail.com"),
        sign_in_count: 1,
    };
    println!("{}", user_1.username);
    user_1.username = String::from("bobby2"); // CHANGE USERNAME
    println!("{}", user_1.username);

    // INTERACTING WITH STRUCT VIA A BUILDER FUNCTION
    let mut user_2 = build_user(String::from("alice1"), String::from("alice1@gmail.com"));
    println!("{}", user_2.username);
    user_2.username = String::from("alice2"); // CHANGE USERNAME
    println!("{}", user_2.username);

    // CREATING NEW FROM EXISTING INSTANCE USING STRUCT UPDATE SYNTAX
    let user_3 = User {
        username: String::from("jack1"),
        ..user_2 // this weird syntax makes so that all other fields should remain same as in user_2
    };
    println!("{}", user_3.username);

    // INSTANTIATE TUPLE STRUCTS
    let black = Color(0, 0, 0);
    println!("First index in the Color tuple is {}", black.1);
    let _origin = Point(0, 0, 0);

    // INSTANTIATE NON-FIELD STRUCT (UNIT-LIKE STRUCTS)
    let _subject = AlwaysEqual;
}
