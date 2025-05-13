use std::fs;

const INPUT: &str = "y2025_6.in";

pub fn main() {
    // Tuple: (word-length, line-number, word)
    let mut word_list: Vec<(usize, usize, String)> = Vec::new();

    // Tuple: (word-length, char-index, character-to-match)
    let mut cross_word: Vec<(usize, usize, char)> = Vec::new();

    let input_str: String = fs::read_to_string(INPUT).unwrap();

    let mut first_part = true;
    for (i, line) in input_str.lines().enumerate() {
        if line.is_empty() {
            // An empty line means we have arrived at the 2nd half of the input.
            first_part = false;
        }

        if first_part {
            let line_number = i + 1;
            let word = match (line_number % 3 == 0, line_number % 5 == 0) {
                // every 3rd
                (true, false) => fix_word(line),
                // every 5th
                (false, true) => fix_word(line),
                // every 15th
                (true, true) => fix_word(&fix_word(line)),
                // every else
                (false, false) => line.to_string(),
            };

            let entry: (usize, usize, String) = (word.chars().count(), line_number, word);
            word_list.push(entry);
        } else {
            let line = line.trim();

            for (i, c) in line.chars().enumerate() {
                if c != '.' {
                    let entry: (usize, usize, char) = (line.chars().count(), i, c);
                    cross_word.push(entry);
                    break;
                }
            }
        }
    }

    let res = solve(word_list, cross_word);
    assert_eq!(11252, res);
    print!("{res}");
}

fn fix_word(line: &str) -> String {
    let raw_bytes = line
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();

    String::from_utf8(raw_bytes).unwrap()
}

fn solve(word_list: Vec<(usize, usize, String)>, cross_word: Vec<(usize, usize, char)>) -> usize {
    let mut res = 0;

    'cw: for (cw_len, char_index, cw_character) in cross_word.into_iter() {
        'wl: for (wl_len, line_number, wl_word) in word_list.iter() {
            if cw_len != *wl_len { continue; }
            if wl_word.chars().nth(char_index).unwrap() == cw_character {
                res += line_number;
                break 'wl
            }
        }
    }

    res
}
