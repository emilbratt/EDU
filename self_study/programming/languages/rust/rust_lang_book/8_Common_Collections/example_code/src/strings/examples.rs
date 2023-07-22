pub fn declare_strings() {
    // Creates a new empty string which we can load data into
    let mut s = String::new();
    // ..the data to load into the string
    let data = "here are some strings to load";
    // ..add the data to the empty string
    let s = data.to_string();
    println!("{}", s);

    // Because strings are used for so many things,
    // we can use many different generic APIs for strings
    // ..like doing everything in one line this way -> String::from()
    let s = String::from("here are som strings in one line of rust");
    // ..or that way -> .to_string()
    let s = "here are som strings in one line of rust".to_string();
    println!("{}", s);
}

pub fn strins_are_utf8() {
    println!("Greetings in different languages using UTF-8 characters");
    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
}

pub fn append_to_strings() {
    // Grow a String by using the push_str() method.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // NOTE: does not take ownership
    println!("s2 is {}", s2);
    // ..if push_str() took ownership of s2, we wouldnt be able to print its value

    // push() takes only a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("{} - laughing out loud", s);

}

pub fn combine_strings() {
    // Read explanation under, quite some important remarks to follow here..
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The "add" statement above takes ownership of s1
    println!("{} is the combined string", s3);
    /*
    The reason s1 is no longer valid after the addition,
    and the reason we used a reference to s2,
    has to do with the signature of the method thats called
    when we use the + operator.
    Signature looks like this
    > fn add(self, s: &str) -> String {
    But the type of &s2 is actually &String.
    The compiler cane coerce the &String into a &str before the add is done.
    Also, also, s2 is still valid!!
    
    The s parameter in the add function: we can only add a &str to a String
    */

    // Combining multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // s2 and s3 are still valid, s1 is NOT valid
    println!("Combined with add: {}", s);

    // Using the format macro to combine multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("Combined with format macro: {}", s);
}

pub fn indexing_into_strings() {
    // In many other programming languages, accessing individual characters in a string
    // by referencing them by index is a valid and common operation,
    // ..but not in Rust as strings dont support indexing..

    /*
    This is not valid code (only Latin letters that would fit in the ASCII table )
    > let s1 = String::from("hello");
    > let h = s1[0];

    This is not valid code either (Only Unicode scalar values e.g. UTF-8)
    > let hello = "Здравствуйте";
    > let answer = &hello[0];
    */

    // To put it another way, UTF-8 is a variable-length encoding.
    // Normal ascii characters = 1 byte, non ascii are UTF-8 encoded Unicode > 1 bytes
    // ..and we can see this in the example below..
    let hello = String::from("Hola"); // 1 byte each char -> len will be 4 because
    let hello = String::from("твуй"); // 2 bytes each char -> len will be 8 bytes
    // Indexing strings will not always work.
    // You would get the byte value but not necessarily a valid chracter.
    // This means that to avoid returning an unexpected value,
    // Rust doesnt compile this code at all..

    /*
    A final reason Rust doesnt allow us to index into a String to get a character
    is that indexing operations are expected to always take constant time (O(1)).
    But it isnt possible to guarantee that performance with a String,
    because Rust would have to walk through the contents from the beginning to the
    index to determine how many valid characters there were.
    */

    // So how do we get a single or several characters from a String?
    let some_string = String::from("Зд");
    // Index a string using loop and the .chars() method.
    for c in some_string.chars() {
        println!("{}", c);
        // do something with the one of the 2 characters here
    }
    // Index a string using loop and the .bytes() method.
    for c in some_string.bytes() {
        println!("{}", c);
        // do something with one of the 4 byte numbers here..
        // why 4? ..because these are valid unicode scalar values, in this case 2 bytes for both
    }
}
