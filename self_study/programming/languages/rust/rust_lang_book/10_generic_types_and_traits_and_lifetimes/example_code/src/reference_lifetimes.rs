/* NOTE:
 *  Modules strictly follow directory naming.
 *  This means Rust uses looks for modules in the directory with the same name as this filename.
 */

 pub mod lifetime_example; // module: src/reference_lifetimes/defining_trait.rs


 pub fn run_all() {
    lifetime_example::run_example();
 }
 