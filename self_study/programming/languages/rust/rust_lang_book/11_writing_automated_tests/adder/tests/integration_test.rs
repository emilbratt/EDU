/*
    integration tests
        Each file in this directory is a separate crate,
        so we need to bring our library into each test crate's scope.
        For that reason we add "use adder" at the top of the code,
        which we didn't need in the unit tests.
    
    Cargo treats this directory specially and compiles files in this directory
    only when we run cargo test
*/

use adder;

mod common;

#[test]
fn it_adds_two() {
    let i: i32 = common::get_number_4();
    assert_eq!(i, adder::add_two(2));
}
