pub fn panic_macro() {
    // Makes the program panic
    panic!("crash and burn");
}

pub fn access_index_out_of_bounds() {
    let v = vec![1, 2, 3];
    v[99];
}

pub fn recoverable_error() {
    use std::fs::File;
    use std::io::ErrorKind;

    // Open a file, if not exist, create the file, if creating fails, panic..

    /*
    The return type for File::open() can be one of two values and looks like this
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    Like Option, Result is brought into scope in the prelude (no need to import)
    ..so we dont need to specify Result:: before the Ok and Err
    T represents the returned value if it succeeds inside OK()
    E represents the returned value if it failes inside Err()
    File::open() give us either the file handle or error information..
    */

    // Store the Result<T, E> in file_handle
    let file_handle = File::open("hello.txt"); // return type -> Result<T, E>

    // Lets handle Result<T, E> for each case that occours on trying to open the file..
    let file_result = match file_handle {
        Ok(file) => file, // will be assigned to file_result if succeeded
        Err(error) => match error.kind() { // What to do if error
            // Checking if the value returned by error.kind() is the NotFound variant of ErrorKind.
            ErrorKind::NotFound => match File::create("hello.txt") { // if not found, create
                Ok(fc) => fc, // file created, all good
                Err(e) => panic!("Problem creating the file: {:?}", e), // creating file failed
            },
            other_error => {
                panic!("Unknown problem when opening the file: {:?}", other_error); // other error occoured
            }
        },
    };
    println!("Cool, we have opened the filehandle: {:?}", file_result);

    /*
    Showing the same code as above, but cleaner and without match statements

    let file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */

    // BTW, opening a file can be done very easily using unwrap()

    // Uncomment the line below for testing
    // let file_result = File::open("some_file.txt").unwrap();

    // If the Result value is the Ok variant,
    //     ..unwrap() will return the value inside the Ok.
    // If the Result is the Err variant,
    //     ..unwrap() will call the panic! macro for us.

    // Also, the expect() method lets us also choose the panic! error message
    // Uncomment the 2 lines below for testing
    // let greeting_file = File::open("some_file.txt")
        // .expect("the file: some_file.txt does not exist");
}

pub fn propagating_error() {
    // send back the error from a failing function to the code that called it

    use std::fs::File;
    use std::io::{self, Read};
    
    // Lets create a function that returns either the expected value or an error
    fn _read_username_from_file(filename: String) -> Result<String, io::Error> {
        let username_file_result = File::open(filename);
        let mut username_file = match username_file_result {
            // One of these two will happen..
            Ok(file) => file, // value inside file is put into the username_file variable
            Err(e) => return Err(e), // e is returned if above fails
        };
    
        // If above succeeds, block below is executed..
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            // One of these two will be returned, no need for return statement in last expression
            Ok(_) => Ok(username), // if the file existed, username is the final return value
            Err(e) => Err(e),  // if reading file failed, e is the final return value
        }

        /*
        If this function succeeds without any problems,
            function will return an Ok value that holds a String.

        If this function encounters any problems,
            function will return an Err value that holds an instance of io::Error
            which contains more information about what the problems were.

            We chose io::Error as the return type of this function because
            that happens to be the type of the error value returned from
            both of the operations were calling in this functions body that might fail
            when calling File::open() and File::open::read_to_string()

        Its up to the calling code to decide what to do with either value..
        */
    }

    // Testing the function (spoiler: it will likely return an error)..
    let final_value = _read_username_from_file(String::from("username.txt"));
    println!("final result on calling read_username_from_file(): {:?}", final_value);

}

pub fn propagating_error_v2() {
    // Pattern of propagating errors is so common in Rust
    // ..that Rust provides the question mark operator ? to make this easier
    // In essence; the ? operator eliminates a lot of boilerplate.
    // Read more:
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used

    use std::fs::File;
    use std::io::{self, Read};

    // When the ? operator calls the from function,
    // the error type received is converted into the error type
    // defined in the return type of the current function it is inside.
    // In this case, the io::Error.
    // This is useful when one error type represent all the ways a function might fail.
    fn _read_username_from_file(filename: String) -> Result<String, io::Error> {
        let mut username_file = File::open(filename)?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
        /*
        The ? at the end of File::open() will return eitehr OK or Error
            If OK -> return value inside an Ok to variable username_file
            If Error -> return early out of the whole function and give any Err
        ..same thing applies to the ? at the end of read_to_string()
        */
    }

    // Testing the function (spoiler: it will likely return an error)..
    let final_value = _read_username_from_file(String::from("username.txt"));
    println!("final result on calling read_username_from_file(): {:?}", final_value);


    // Expansion on opening and reading a file for the curious..
    /*
    A shorter implementation of _read_username_from_file() would use method chaining.

    > fn _read_username_from_file(filename: String) -> Result<String, io::Error> {
    >     let mut username = String::new();
    >     File::open(filename)?.read_to_string(&mut username)?;
    >     Ok(username)
    > }
    > let username = read_username_from_file(String::from("sometext.txt"))
    */

    /*
    The shortest implementation of _read_username_from_file() would use one line.
    use std::fs;
    use std::io;

    > fn read_username_from_file(filename: String) -> Result<String, io::Error> {
    >     fs::read_to_string(filename)
    > }
    > let username = read_username_from_file(String::from("sometext.txt"))

    ..using fs::read_to_string instead of opening and then reading the file.
    */
}

pub fn validate_custom_types() {
    use std::cmp::Ordering;
    
    
    // Create a new type that validates input for the number ranging from 1 - 100
    pub struct Guess {
        value: i32,
    }
    // The methods for storing and validating the type
    impl Guess {
        // returns value if all is OK, panics if not
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // Create a function that uses our guess function
    fn guess_the_number(guess: Guess) {
        // hardcode the secret number for simplicity
        let secret_number: i32 = 43;
        // compare the number we set inside the Guess struct
        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    // The "Guess" type will only work if we have inserted a number that fits the criteria of n < 1 and n > 100
    let number = Guess::new(25);
    // At this point, the validation has taken place (program would gave crashed otherwise) -> pass to function
    guess_the_number(number);

}
