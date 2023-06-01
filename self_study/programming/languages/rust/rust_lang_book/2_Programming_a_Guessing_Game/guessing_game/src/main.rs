use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // if we omit the importing on top (first line )
    // the we could still call the io function using this method below
    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");


    println!("You guessed: {guess}");
}
