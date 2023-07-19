
// import module declared in another file
mod front_of_house; // front_of_house.rs

fn deliver_order() {}

// this is a module declared in the same file.
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Call deliver_order() that exists in parend module which is the crate module itself.
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String, // Because public, we can write and read to the toast field using dot notation.
        seasonal_fruit: String, // Because private, it needs public associated function (below) for constructing.
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod customer {
    pub fn eat_at_restaurant() {

        pub use crate::front_of_house::hosting; // Re-exporting (pub use) the hosting module.
        // ..any external code can call restaurant::hosting::add_to_waitlist()

        use crate::back_of_house; // Calling for example back_of_house::Breakfast::summer(variable); will now work.

        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        /*
        meal.seasonal_fruit = String::from("blueberries");
        */
    }

}
