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

    let mut particles = vec![0i64; grid[0].len()];
    particles[start] = 1;
    // step by 2 because input only has splitters on every other row,
    // as a simple optimization - essentially compressing rows.
    for row in (0..grid.len()).step_by(2) {
        let mut next = vec![0i64; grid[0].len()];
        for col in 0..grid[0].len() {
            if particles[col] != 0 {
                if grid[row][col] == '^' {
                    next[col-1] += particles[col];
                    next[col+1] += particles[col];
                } else {
                    next[col] += particles[col];
                }
            }
        }
        particles = next;
    }

    particles.iter().sum()
}
