use std::thread;

pub fn closure_example_1() {
    /*
     * A closure will handle passed values differnetly.
     * Based on what the body of the function does with the captured values, it will either:
     *   ..borrow immutably
     *   ..borrow mutably
     *   ..or take ownership
     */

    let mut list = vec![1, 2, 3];


    // closure that captures a reference (list) - borrow immutably
    let only_borrows = || println!("From closure: {:?}", list);

    only_borrows();
    println!("After calling only_borrows(): {:?}", list);

    // closure that captures a mutable reference (list) - borrow mutably
    let mut borrows_mutably = || list.push(7);

    // note: no other borrows are allowed if there is mutable borrow later
    // println!("Before calling borrows_mutably(): {:?}", list); // immutable borrow occurs here
    borrows_mutably();                                           // mutable borrow occurs here
    println!("After calling borrows_mutably(): {:?}", list);
}

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

pub fn closure_example_2() {

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

pub fn closure_example_3() {
    /*
     * Sometimes, we might want to "move" the variable out of the closure.
     * For example, if we call the closure from a new thread.
     * This is because the main thread might finish before the new.
     */
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Using the keyword "move" to explicitly tell rust that the ownership is moved.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn closure_example_4() {
    // sort a slice by a particular attribute of each item (width in this example)
    let mut some_slice = [
        Rectangle { width: 10, height: 1 }, // 3rd
        Rectangle { width: 3, height: 5 },  // 1st
        Rectangle { width: 7, height: 12 }, // 2nd
    ];

    // sort_by_key is defined on slices -> [] and implements trait bound "FnMut"
    some_slice.sort_by_key(|r| r.width); // gets one argument in the form of a reference
    println!("{:#?}", some_slice);

    /*
     * The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times.
     * Once for each item in the slice.
     * The closure |r| r.width doesn't capture, mutate, or move out anything from its environment,
     * so it meets the trait bound requirements.
     */
}

pub fn closure_example_5() {
    // same as example 4, but we also count the times sort_by_key is called
    let mut some_slice = [
        Rectangle { width: 10, height: 1 }, // 3rd
        Rectangle { width: 3, height: 5 },  // 1st
        Rectangle { width: 7, height: 12 }, // 2nd
    ];
    let mut num_sort_operations = 0;
    // capturing a mutable reference to the num_sort_operations counter
    some_slice.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", some_slice);
}
