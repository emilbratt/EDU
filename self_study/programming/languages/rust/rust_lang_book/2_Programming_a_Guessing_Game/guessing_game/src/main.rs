use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // includes 1 AND 100
    
    println!("Guess the number game!");

    loop {
        println!("Please input a number (1 to 100).");
        let mut guess = String::new();
        
        // READ USER INPUT
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // TYPE CASTING
        // trim (remove likely unwanted characters)
        //  removes string data that might persist from input,
        //  ..for instance any whitespace at the beginning and end, \n and \r\n ...
        // parse (change type)
        //  type casting e.g. converts a type to another type,
        //  ..in this case a string to u32 as seen when it was declared
        let guess: u32 = match guess.trim().parse() {
            // If parse is able to successfully turn the string into a number, it will return an Ok value
            // Then keep executing code below this block
            Ok(num) => num,
            // If parse is not able it will return Err value.
            // Then restart loop from top (continue)
            Err(_) => continue, // underscore, _, is a catchall value (matching all Err values)
        };

        // COMPARE VARIABLES
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct, bye!");
                break;
            }
        }
    }

    println!("The secret number is: {secret_number}");
}
