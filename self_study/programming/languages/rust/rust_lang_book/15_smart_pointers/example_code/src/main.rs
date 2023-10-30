pub mod box_type;
pub mod deref_trait;
pub mod drop_trait;
pub mod rf_type;

fn main() {
    box_type::run();
    deref_trait::run();
    drop_trait::run();
    rf_type::run();
}
