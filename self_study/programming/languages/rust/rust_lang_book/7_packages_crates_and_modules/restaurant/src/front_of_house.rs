pub mod hosting; // ./front_of_house/hosting.rs
// works because THIS file is named front_of_house.rs
// which is the same name as the directory holding hosting.rs


mod serving {
    fn take_order() {}

    fn serve_order() {}
    
    fn take_payment() {}
}
/*
    While front_of_house isnt public,
    because the eat_at_restaurant function is defined in the same module as
    front_of_house (eat_at_restaurant and front_of_house are siblings),
    we can refer to front_of_house from eat_at_restaurant.

    The hosting module is marked with pub.
    We can access the parent module of hosting, so we can access hosting.

    The add_to_waitlist function is marked with pub.
    We can access it from parent module or via use keyword in another source file.
*/
