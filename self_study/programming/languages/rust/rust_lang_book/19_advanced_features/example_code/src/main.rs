pub mod deref_raw_pointer;
pub mod unsafe_functions;
pub mod c_code;
pub mod mutable_static_variable;
pub mod unsafe_trait;
pub mod advanced_traits;
pub mod advanced_types;
pub mod adv_functions_and_closures;
pub mod macros;

fn main() {
    deref_raw_pointer::run();
    unsafe_functions::run();
    c_code::run();
    mutable_static_variable::run();
    unsafe_trait::run();
    advanced_traits::run();
    advanced_types::run();
    adv_functions_and_closures::run();
    macros::run();
}
