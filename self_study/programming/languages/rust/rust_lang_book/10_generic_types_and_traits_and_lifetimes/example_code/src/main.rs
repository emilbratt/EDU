#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

pub mod generic_types;
pub mod traits_behaviour;
pub mod reference_lifetimes;

fn main() {
    generic_types::run_all();
    traits_behaviour::run_all();
    reference_lifetimes::run_all();
}
