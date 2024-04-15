pub mod conditional_if_let;
pub mod while_let;
pub mod pattern_syntax;

fn main() {
    println!("\n\nconditional_if_let.rs");
    conditional_if_let::run();
    
    println!("\n\nwhile_let.rs");
    while_let::run();

    println!("\n\npattern_syntax.rs");
    pattern_syntax::run();
}
