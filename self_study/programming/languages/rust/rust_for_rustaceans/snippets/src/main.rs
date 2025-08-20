#![allow(unused)]

mod _1_foundations;
mod _2_types;

static mut CHAPTER: usize = 0;

fn main() {
    chapter("Foundations", _1_foundations::main);
    chapter("Types", _2_types::main);
}

fn chapter(title: &str, f: impl FnOnce()) {
    let chapter = unsafe {
        CHAPTER += 1;
        *(&raw const CHAPTER)
    };

    println!("---Chapter {}. {}---", chapter, title);
    f();
    println!();
}
