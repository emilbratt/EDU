## Advent of Code - Rust

### Input files

* get session cookie using inspection tool in your browser

Program will automatically download input files. Make sure to create file "session.in"
and put your session token inside it..

Manually download input with curl
```sh
AOC_SESSION=746c5645f5...........................

# run curl for select year and day, example uses year 2024 and day 1
curl https://adventofcode.com/2024/day/1/input --cookie "session=${AOC_SESSION}" > y2024_d01.in
```

### Run solution

* at least two args (year day) is needed
Run with for example `cargo run 2025 1 1`.\
All solutions are isolated (no shared code).\
This means that you can copy code directly from the solution file e.g. src/solutions/y2024d01p1.rs\
..and into your crate if you want.


### Disclaimer

I will most likely select the puzzles that are not too hard to solve.
When the first 9-10 puzzle days for 2024 are completed or when they get sufficiently hard,\
then I will pick days from earlier years and solve them instead. :)

### How I solve

I have three things in mind when solving.\
1. No external dependencies as I like to make my own implementations.\
2. Try to write performant code.\
3. Isolate solutions having each one in their own respective file.

### Solution template

When adding new solution file, copy the code block below to save some time.

```rs
const CHAR_NUMBER_OFFSET: u8 = 48;
const DOT: u8 = 46;
const HASHTAG: u8 = 35;
const LINE_FEED: u8 = 10;

const INPUT: &str = "y_d.in";

pub fn main() {
    let input: Vec<u8> = std::fs::read(INPUT).unwrap();
    let input: String = std::fs::read_to_string(INPUT).unwrap();

    print!("N/A");
}
```
