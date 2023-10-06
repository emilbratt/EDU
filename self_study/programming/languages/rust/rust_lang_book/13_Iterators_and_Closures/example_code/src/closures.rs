pub mod examples;

pub fn run() {
    println!("closure_example_1()\n");
    examples::closure_example_1(); // simple example to get started

    println!("\n\nclosure_example_2()\n");
    examples::closure_example_2(); // closure as a method within a struct

    println!("\n\nclosure_example_3()\n");
    examples::closure_example_3(); // when you want to move even though closure body need no ownership

    println!("\n\nclosure_example_4()\n");
    examples::closure_example_4(); // when the closure mutates the value, it implements FnMut

    println!("\n\nclosure_example_5()\n");
    examples::closure_example_5(); // same as example 4, but with a counter
}
