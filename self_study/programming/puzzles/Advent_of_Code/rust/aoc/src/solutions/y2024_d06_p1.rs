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
    pub x: i16,
    pub y: i16,
    pub direction: Direction,
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

    let mut p = Position::new(0, 0);

    let mut x: i16 = 0;
    let mut y: i16 = 0;

    let mut row: Vec<u8> = Vec::new();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for b in input {
        if b == START_MARK {
            p.x = x;
            p.y = y;
        }

        if b == LINE_FEED {
            grid.push(row.clone());
            row.clear();
            x = 0;
            y += 1;
        } else {
            x += 1;
            row.push(b);
        }
    }

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

    print!("{visited}");
}
