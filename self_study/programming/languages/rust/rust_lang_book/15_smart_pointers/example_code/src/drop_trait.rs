// The Drop trait (commonly named "destructor" in other languages)

struct CustomSmartPointer {
    data: String,
}

// Implementing the Drop ("destructor") functionality.
impl Drop for CustomSmartPointer {
    // This is our custom drop function that will run automatically.
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn custom_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // At this point, the .drop() method is called for both c and d by Rust automatically.
}

fn early_drop_value() {
    /*
    * You might want to clean up a value early.
    * One example is when using smart pointers that manage locks.
    * You might want to force the drop method that releases the lock.
    * That way other code in the same scope can acquire the lock.
    * Rust doesn’t let you call the Drop trait’s drop method manually;
    * instead you have to call the std::mem::drop function provided by the standard library.
    * That is, if you want to force a value to be dropped before the end of its scope.
    */

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");

    // Using std::mem::drop (in prelude, no need to import) to drop early.
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}

pub fn run() {
    println!("\n\ndrop_trait.rs\n");

    custom_drop_trait();
    early_drop_value();
}
