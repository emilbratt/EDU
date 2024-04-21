// https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming
fn declarative_macros_with_macro_rules_for_meta_programming() {
    // A simplified version of the vec! macro definition
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    
    // Running the vec! macro
    let _v: Vec<u32> = vec![1, 2, 3];
}

/*
    Not included here.

        Procedural Macros for Generating Code from Attributes:
        https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes

        How to Write a Custom derive Macro:
        https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

        Attribute-like macros:
        https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros
*/

pub fn run() {
    declarative_macros_with_macro_rules_for_meta_programming();
}
