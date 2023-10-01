#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {

    // the closure example lives in this method
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /*
         * If the Option<T> (Option<ShirtColor>) is the Some variant:
         *  ..unwrap_or_else returns the value from within the Some.
         * 
         * If the Option<T> (Option<ShirtColor>) is the None variant:
         *  ..unwrap_or_else calls the closure and returns the value returned by the closure.
         *  ..notice the closure expression: || self.most_stocked()
         *  ..parameters to the closure would be placed between the two vertical bars..
         *  ..implementation of unwrap_or_else evaluates the closure later if needed..
         */
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
