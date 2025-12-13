use std::collections::VecDeque;

const INPUT: &str = "y2025_d07.in";

pub fn main() {
    let input = std::fs::read(INPUT).unwrap();
    let grid = gridify(input);
    let ans = solve(grid);
    print!("{ans}");
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
    let mut start = 0;
    for i in 0..grid[0].len() {
        if grid[0][i] == 'S' {
            start = i;
            break;
        }
    }

    let mut ans = 0;
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut d: VecDeque<(usize, usize)> = VecDeque::new();
    d.push_back((0, start));
    while let Some((mut row, col)) = d.pop_back() {
        while row < grid.len() && !visited.contains(&(row, col)) {
            visited.push((row, col));

            if grid[row][col] == '^' {
                ans += 1;
                d.push_back((row+1, col+1));
                d.push_back((row+1, col-1));
                break;
            }
            row += 1;
        }
    }

    ans
}