use std::fs;

const INPUT: &str = "y2024_d06.in";

const LINE_FEED: u8 = 10;
const START_MARK: u8 = 94;
const DOT: u8 = 46;
const OBSTACLE: u8 = 35;
const MARKED: u8 = 88;

enum Direction {
    N, S, E, W,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }
}

struct Position {
    x: i16,
    y: i16,
    direction: Direction,
}

impl Position {
    fn new(y: i16, x: i16,) -> Self {
        Self {
            y,
            x,
            direction: Direction::N,
        }
    }

    fn turn_right(&mut self) {
        self.direction.turn_right()
    }

    fn step(&mut self, reverse: bool) {
        match self.direction {
            Direction::N => {
                if !reverse {
                    self.y -= 1
                } else {
                    self.y += 1
                }
            }
            Direction::E => {
                if !reverse {
                    self.x += 1
                } else {
                    self.x -= 1
                }
            }
            Direction::S => {
                if !reverse {
                    self.y += 1
                } else {
                    self.y -= 1
                }
            }
            Direction::W => {
                if !reverse {
                    self.x -= 1
                } else {
                    self.x += 1
                }
            }
        }
    }

    fn x(&self) -> usize {
        self.x as usize
    }

    fn y(&self) -> usize {
        self.y as usize
    }
}

pub fn main() {
    let input: Vec<u8> = fs::read(INPUT).unwrap();

    let mut grid = gridify(&input);

    let mut x = 0;
    let mut y = 0;
    let mut start_x = 0;
    let mut start_y = 0;
    for row in grid.iter() {
        for b in row {
            if b == &START_MARK {
                (start_y, start_x) =  (y, x);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    let mut p = Position::new(start_y, start_x);

    let mut visited = 0;
    while p.y >= 0 && p.y() < grid.len() && p.x >= 0 && p.x() < grid[0].len() {
        if grid[p.y()][p.x()] == OBSTACLE {
            p.step(true);
            p.turn_right();
        }

        if grid[p.y()][p.x()] != MARKED {
            visited += 1;
        }

        grid[p.y()][p.x()] = MARKED;

        p.step(false);
    }

    let limit = visited * 2;

    // This is a brute-force solution where we place obstacle and have the guard run around.
    // I wonder if there are faster solutions.
    let mut placed = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {

            // We only add obstacles where we have marked (other tiles are not reachable).
            if grid[y][x] == MARKED {
                // Add obstacle.
                grid[y][x] = OBSTACLE;

                // Do a walk until x steps are taken and see if we do not escape the grid.
                if walk(&grid, limit, start_y, start_x) {
                    placed += 1;
                }

                // Remove obstacle.
                grid[y][x] = DOT;

            }
        }
    }

    print!("{placed}");
}

fn walk(grid: &Vec<Vec<u8>>, limit: u64, start_y: i16, start_x: i16) -> bool {
    let mut p = Position::new(start_y, start_x);
    let mut grid = grid.clone();

    let mut count = 0;
    while p.y >= 0 && p.y() < grid.len() && p.x >= 0 && p.x() < grid[0].len() {
        if grid[p.y()][p.x()] == OBSTACLE {
            p.step(true);
            p.turn_right();
        }

        grid[p.y()][p.x()] = MARKED;

        // If reached limit, we assume the guard is stuck in a loop.
        count += 1;
        if count > limit {
            return true;
        }

        p.step(false);
    }

    false
}

fn gridify(input: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut row: Vec<u8> = Vec::new();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for b in input {
        if *b == LINE_FEED {
            grid.push(row.clone());
            row.clear();
        } else {
            row.push(*b);
        }
    }

    grid
}

fn start_position(grid: &Vec<Vec<u8>>) -> (i16, i16) {
    let mut x = 0;
    let mut y = 0;
    for row in grid {
        for b in row {
            if b == &START_MARK {
                return (y, x);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    (y, x)
}
