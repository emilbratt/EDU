pub fn iterator_example_1() {
    // create array
    let v1 = vec![1, 2, 3];

    // store an iterator in a variable by calling iter()
    let mut v1_iter = v1.iter();

    // call next on the iterator to get immutable references (next eventually uses the iterator)
    println!("{:?}", v1_iter.next()); // will print Some(1)
    println!("{:?}", v1_iter.next()); // will print Some(2)
    println!("{:?}", v1_iter.next()); // will print Some(3)
    println!("{:?}", v1_iter.next()); // will print None
    println!("{:?}", v1_iter.next()); // will print None
}

pub fn iterator_example_2() {
    // create array
    let v1 = vec![1, 2, 3];

    // store an iterator in a variable by calling iter()
    let v1_iter = v1.iter();

    // sum method takes ownership of the iterator and is repeatedly calling next in the background
    let total: i32 = v1_iter.sum();

    println!("{}", total);
}

pub fn iterator_example_3() {
    // create array
    let v1: Vec<i32> = vec![1, 2, 3];

    // store an iterator in a variable by calling iter()
    let v1_iter = v1.iter();

    // calling the iterator adaptor (map) which returns a new iterator
    let v2 = v1_iter.map(|x| x + x); // notice that map takes a closure

    // consumes the iterator and collects resulting values into a collection type
    let v3: Vec<_> = v2.collect();

    println!("{:?}", v3);

    /*
     *  Summary:
     *      Calling the map method produces a new iterator
     *      Then calling the collect method to consume the new iterator
     */
}

pub fn iterator_example_4() {
    /*
     * Many iterator adapters take closures as arguments.
     * Commonly the closures we'll specify as arguments to iterator adapters will be closures.
     * Those closures might need to capture their environment, like in this example.
     */

    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        /*
         * What is happening
         * We call into_iter to create an iterator.
         * We that takes ownership of the vector.
         * Then we call filter to adapt that iterator into a new iterator.
         * It only contains elements for which the closure returns true.
         */
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        // note: into_iter() takes ownership of the vector "shoes" and returns owned values
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("{} with size {}", in_my_size[0].style, in_my_size[0].size);
    println!("{} with size {}", in_my_size[1].style, in_my_size[1].size);
}
