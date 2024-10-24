/* NOTE:
 *  Modules strictly follow directory naming.
 *  This means Rust uses looks for modules in the directory with the same name as this filename.
 */

pub mod defining_trait; // module: src/traits_behaviour/defining_trait.rs


pub fn run_all() {
    defining_trait::run_example();
}
