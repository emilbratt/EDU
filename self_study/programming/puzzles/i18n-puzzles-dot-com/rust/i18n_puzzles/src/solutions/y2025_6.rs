use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;

use std::fs;

const INPUT: &str = "y2025_6.in";

// struct Word<'a> {
//     word: &'a str,
//     len: usize,
//     index: usize,
// }

pub fn main() {
    let mut res = 0;

    println!();
    let input_str: String = fs::read_to_string(INPUT).unwrap();
    let input = fs::read(INPUT).unwrap();

    let mut all_words: Vec<String> = Vec::new();
    let mut cur_word: Vec<u8> = Vec::new();
    let mut index = 0;
    for b in input.into_iter() {
        if b as char == '\n' {
            let word = match (index % 3 == 0, index % 5 == 0) {
                // every 3rd
                (true, false) => {
                    ISO_8859_1.decode(&cur_word, DecoderTrap::Strict).unwrap()
                }

                // every 5th
                (false, true) => ISO_8859_1.decode(&cur_word, DecoderTrap::Strict).unwrap(),
                
                // every 15th
                (true, true) => String::from_utf8(cur_word.clone()).unwrap(),

                // every else
                (false, false) => String::from_utf8(cur_word.clone()).unwrap(),
            };

            all_words.push(word);
            cur_word.clear();
            index += 1;
        } else {
            cur_word.push(b);
        }
    }

    for word in all_words {
        println!("{}", word);
        // println!();
    }


    // let mut split = input_str.split("\n\n");
    // let words = split.next().unwrap();
    // // let mut word_vec: Vec<Word> = Vec::with_capacity(words.len());

    // let mut v_words: Vec<(&str, usize, usize)> = Vec::with_capacity(words.len());
    // for (index, word) in words.lines().enumerate() {
    //     // let len = word.chars().count();
    //     // word_vec.push( Word { word, len, index, } );

    //     let word: &str = match (index % 3 == 0, index % 5 == 0) {
    //         // every 3rd
    //         (true, false) => word,

    //         // every 5th
    //         (false, true) => word,
            
    //         // every 15th
    //         (true, true) => word,

    //         // every else
    //         (false, false) => word,
    //     };

    //     let v: (&str, usize, usize) = (&word, word.chars().count(), index);
    //     v_words.push(v);
    // }

    // for (w, len, index) in v_words {
    //     // println!("{}", w);
    // }

    // let crossword = split.next().unwrap();
    // println!("{}", crossword);

    // print!("{res}");


    println!();
}
