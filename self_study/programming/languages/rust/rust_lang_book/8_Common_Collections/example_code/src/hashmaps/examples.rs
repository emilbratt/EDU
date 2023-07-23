use std::collections::HashMap; // "symlink" HashMap
/*
Of our three common collections, this one is the least often used,
so its not included in the features brought into scope automatically in the prelude
Therefore we include the "use" statement on top

Like vectors, hasmaps are stored on the heap.
*/

pub fn declare_hashmap() {
    let mut scores = HashMap::new();
    // Create hashmap where keys -> type String and values -> type i32
    // NOTE: all keys must have same type, and all values must same type..
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Getting a value from the hashmap using the get().copied().unwrap_or() method
    let key_team = String::from("Blue"); // store the key in a variable
    let val_score = scores.get(&key_team).copied().unwrap_or(0); // use the key
    // The get method returns an Option<&V>;
    println!("The score for team {} is {}", key_team, val_score);
    /*
    ..more about "getting" the value..
        get(&key)
            ..if not value, return None
            ..if value, return Option<&V>
        
        copied()
            ..turn Option<&V> into Option<V>
            ..in this case turn Option<&i32> into Option<i32>
        
        unwrap_or(val)
            ..if not value, return n that is specified manually by you in unwrap_or(value)
            ..if value, return the value specified by you in get(key)
    */
}

pub fn iterate_over_k_v_pair() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // This will print each pair in an arbitrary order
    for (key, value) in &scores {
        println!("key: {}\nvalue: {}\n", key, value);
    }
}

pub fn insert_and_ownership_for_hashmap() {
    // Types that implement the Copy trait, like i32, will have values copied into hash map.
    // Others "Owned values" like Strings, will have the values moevd to the hashmap.

    // Example how the hashmap will take ownership of the fields below
    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    map.insert(field_name, field_value);
    // ..field_name and field_value are both invalid -> values owned by the hashmap.

    // If we insert references to values into the hash map,
    // the values wont be moved into the hash map.
    // The values that the references point to must be valid
    // for at least as long as the hash map is valid. 
}

pub fn on_updating_hashmap() {
    /*
    If you want to change the data in a hash map,
    you have to decide how to handle the case
    when a key already has a value assigned..
    Basiaclly..
        remove old and keep new,
        ignore new and keep old,
        combine old and new..
    */
    let mut scores = HashMap::new();

    // Insert value (and overwrite if key val pair exist)
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // ..value is 25 because it was inserted last

    // Add new value if no key val pair exists
    scores.entry(String::from("Yellow")).or_insert(50); // 50 is inserted
    scores.entry(String::from("Blue")).or_insert(50); // nothing is inserted
    // ..or_insert() will insert if no entry exists for the key

    // Combine old value and new value (for example when counting words)
    let text = String::from("hello world wonderful world beaufiful world");
    let mut map = HashMap::new();
    // Using split_whitespace() to iterate over words in the String..
    for word in text.split_whitespace() {
        // this line will simply add key={word} and value=0 for new words
        let count = map.entry(word).or_insert(0);
        // or_insert() returns the mutable reference &mut V to the value for the key
        *count += 1;
        // we use the returned reference, dereference it with * and add +1 to the value
    }
    // should get {"wonderful": 1, "beaufiful": 1, "hello": 1, "world": 3}
    println!("{:?}", map);
}
