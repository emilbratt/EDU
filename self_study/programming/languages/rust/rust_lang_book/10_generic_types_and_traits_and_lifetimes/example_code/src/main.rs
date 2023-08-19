#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod generic_types;
pub mod traits_behaviour;

fn main() {
    generic_types::run_all();
    traits_behaviour::run_all();
}
