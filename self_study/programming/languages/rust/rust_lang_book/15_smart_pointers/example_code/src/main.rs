pub mod box_type;
pub mod deref_trait;
pub mod drop_trait;

fn main() {
    box_type::run();
    deref_trait::run();
    drop_trait::run();
}
