// Define a module with the mod keyword.
mod front_of_house {
    // Inside modules, we can place other modules which in this example contain functions.
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
