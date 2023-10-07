pub mod examples;

pub fn run() {
    println!("\n\niterator_example_1()\n");
    examples::iterator_example_1(); // manually iterate with "next"

    println!("\n\niterator_example_2()\n");
    examples::iterator_example_2(); // sum uses an iterator

    println!("\n\niterator_example_3()\n");
    examples::iterator_example_3(); // iterator adapters - method that produce iterator

    println!("\n\niterator_example_4()\n");
    examples::iterator_example_4(); // iterator adapters - using closure that capture environment
}
