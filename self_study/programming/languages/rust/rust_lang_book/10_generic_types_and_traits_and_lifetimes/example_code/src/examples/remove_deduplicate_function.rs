fn dubplicate_code_find_largest_number() {
    // duplicate code for finding largest in two list -> BAD :(

    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

fn abstract_code_to_find_largest_number() {
    // Abstracted code to find the largest number in two lists -> GOOD :)

    // Create function for finding the largest number.
    // Function takes any concrete slice of i32 values -> &[i32]
    // Could also instead annotate &Vec<i32> if we wanted..
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

pub fn run_example() {
    // bad!
    dubplicate_code_find_largest_number();

    // good!
    abstract_code_to_find_largest_number();
}

