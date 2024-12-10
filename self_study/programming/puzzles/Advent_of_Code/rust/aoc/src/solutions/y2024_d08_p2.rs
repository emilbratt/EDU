use std::fs;
use std::collections::{HashMap, HashSet};

const LINE_FEED: u8 = 10;
const DOT: u8 = 46;
const HASHTAG: u8 = 35;

const INPUT: &str = "y2024_d08.in";

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn try_from(row: i64, col: i64) -> Option<Self> {
        if row >= 0 && col >= 0 {
            let (row, col) = (row as usize, col as usize);
            Some( Self { row, col } )
        } else {
            None
        }
    }

    fn try_set(&mut self, row: i64, col: i64) -> bool {
        if row >= 0 && col >= 0 {
            self.row = row as usize;
            self.col = col as usize;
            true
        } else {
            false
        }
    }

    fn is_inside<T>(&self, grid: &Vec<Vec<T>>) -> bool {
        self.row < grid.len() && self.col < grid[self.row].len()
    }
}

pub fn main() {
    let input: Vec<u8> = fs::read(INPUT).unwrap();

    let grid = gridify(input);
    let mut antennas: HashMap<u8, Vec<Position>> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == DOT { continue; }
            let key = grid[row][col];
            antennas.entry(key)
                .and_modify(|key| key.push(Position { row, col }))
                .or_insert(Vec::from([Position { row, col }]));
        }
    }

    let res = count_antinodes(&grid, &antennas);

    assert_eq!(951, res);
    print!("{res}");
}

fn gridify(input: Vec<u8>) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![Vec::new()];
    let mut columns: Option<usize> = None;
    let mut i: usize = 0;
    for b in input {
        if b == LINE_FEED {
            match columns {
                // First round (length = None), save length of first row.
                None => columns = Some(grid[i].len()),

                // Make sure that every row has equal amount of columns e.g. is a grid.
                Some(v) => assert_eq!(grid[i].len(), v),
            }

            // Add the next vector as an empty one.
            grid.push(Vec::with_capacity(grid[i].len()));
            // ..and move index to the next vector.
            i += 1;
        } else {
            grid[i].push(b);
        }
    }
    assert!(grid.len() > 1);

    grid
}

fn count_antinodes(grid: &Vec<Vec<u8>>, antennas: &HashMap<u8, Vec<Position>>) -> i64 {
    let mut res: i64 = 0;
    let mut antinodes: HashSet<Position> = HashSet::new();
    for (key, val) in antennas.iter() {
        for i in 0..val.len() - 1 {
            for j in i+1..val.len() {
                let cur = val[i];
                let next = val[j];

                let d_row = cur.row - next.row; // change in row (dx)
                let d_col = cur.col - next.col; // change in col (dy)
                let mut multiply: usize;

                multiply = 1;
                // Traverse north and-or west by decreasing row and-or  col.
                if let Some(mut pos) = Position::try_from(cur.row as i64, cur.col as i64) {
                    while pos.is_inside(&grid) {
                        antinodes.insert(pos);
                        pos.row = cur.row - (d_row * multiply);
                        pos.col = cur.col - (d_col * multiply);
                        multiply += 1;
                    }
                }

                multiply = 1;
                // Traverse south and-or east by increasing row and-or col.
                if let Some(mut pos) = Position::try_from(next.row as i64, next.col as i64) {
                    while pos.is_inside(&grid) {
                        antinodes.insert(pos);
                        pos.row = next.row + (d_row * multiply);
                        pos.col = next.col + (d_col * multiply);
                        multiply += 1;
                    }
                }

            }
        }
    }

    antinodes.len() as i64
}
