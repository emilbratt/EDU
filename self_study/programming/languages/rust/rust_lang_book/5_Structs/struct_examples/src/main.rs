#[allow(dead_code)] // suppress compiler warnings for un-used code


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    // BUILDER FUNCTION
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // INTERACTING WITH STRUCT DIRECTLY
    let mut user_1 = User {
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

}
