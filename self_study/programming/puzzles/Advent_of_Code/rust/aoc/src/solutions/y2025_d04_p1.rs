const INPUT: &str = "y2025_d04.in";

pub fn main() {
    let bytes = std::fs::read(INPUT).unwrap();
    let grid = gridify(bytes);

    let mut pos = Position { row: 0, col: 0 };
    let mut ans = 0_i64;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            pos.set(row, col);
            if let Some(v) =  pos.get(0, 0, &grid) {
                if v == '@' {
                    ans += check(&mut pos, &grid);
                }
            }
        }
    }

    print!("{ans}");
}

const ADJACENT: [(i16, i16); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];
fn check(pos: &Position, grid: &[Vec<char>]) -> i64 {
    let mut count: u8 = 0;
    for (r, c) in ADJACENT.iter() {
        if let Some(vv) = pos.get(*r, *c, grid) {
            if vv == '@' {
                count += 1;
            }
        }
    }

    if count < 4 { 1 } else { 0 }
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

struct Position {
    row: i16,
    col: i16,
}

impl Position {
    fn set(&mut self, r: usize, c: usize) {
        self.row = r as i16;
        self.col = c as i16;
    }

    fn get<T: Copy>(&self, r: i16, c: i16, grid: &[Vec<T>]) -> Option<T> {
        match grid.get((self.row+r) as usize) {
            None => None,
            Some(row) => {
                row.get((self.col+c) as usize).copied()
            }
        }
    }
}