

pub fn panic_macro() {
    // Makes the program panic
    panic!("crash and burn");
}

pub fn access_index_out_of_bounds() {
    let v = vec![1, 2, 3];
    v[99];
}

use std::fs::File;
use std::io::ErrorKind;

pub fn recoverable_error() {
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
