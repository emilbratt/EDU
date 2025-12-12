use std::collections::VecDeque;

const INPUT: &str = "y2025_d07.in";

pub fn main() {
    let input = std::fs::read(INPUT).unwrap();
    let grid = gridify(input);
    println!();

    let ans = solve(grid);

    assert!(
        ![
            1725, // to high
            2017, // to high
        ].contains(&ans)
    );

    println!();
    print!("{ans}");
    println!();

}

fn gridify(input: Vec<u8>) -> Vec<Vec<char>> {
    input.split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.iter()
                .map(|b| *b as char)
                .collect()
        })
        .collect()
}

fn solve(grid: Vec<Vec<char>>) -> i64 {
    let mut seen: Vec<(usize, usize, bool)> = Vec::new();
    let mut done: Vec<(usize, usize)> = Vec::new();

    let mut start = 0;
    for i in 0..grid[0].len() {
        if grid[0][i] == 'S' {
            start = i;
            break;
        }
    }

    let mut d: VecDeque<(usize, usize, bool)> = VecDeque::new();

    d.push_back((0, start, true));

    while let Some((mut row, col, right)) = d.pop_back() {
        while row < grid.len() {
            if grid[row][col] == '^' {
                if !done.contains(&(row, col+1)) {
                    d.push_back((row, col+1, true));
                }
                if !done.contains(&(row, col-1)) {
                    d.push_back((row, col-1, false));
                }
                done.push((row, col+1));
                done.push((row, col-1));
                if !seen.contains(&(row, col, right)) {
                    seen.push((row, col, right));
                    row = grid.len();
                }
            }
            row += 1;
        }
        println!("({row}),({col}) {}", d.len());
    }

    seen.len() as i64
}