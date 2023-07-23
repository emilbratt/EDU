#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod hashmaps;
pub mod strings;
pub mod vectors;

fn main() {
    println!("Common Collections");
    vectors::run();
    strings::run();
    hashmaps::run();
}
