### Since we are isolating every solution, copy and paste these common functions.

#### Turn input into dimensional vector (grid)
```rust
const LINE_FEED: u8 = 10;

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
```

#### Print gridified input - Vec<Vec<u8>>
```rust
// For u8 holding any value
const NANO_SECONDS: u32 = 5000000;
const SLEEP: std::time::Duration = std::time::Duration::new(0, NANO_SECONDS);
fn p_grid(grid: &Vec<Vec<u8>>, sleep: u64) {
    let mut s = String::with_capacity(grid.len() * grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = if grid[row][col] <= 9 { '.' } else { (grid[row][col] + 48) as char };
            s.push(c);
        }
        s.push('\n');
    }
    println!("{s}");
    std::thread::sleep(SLEEP);
}

// For u8 holding numerics in range 0-9
const NANO_SECONDS: u32 = 5000000;
const SLEEP: std::time::Duration = std::time::Duration::new(0, NANO_SECONDS);
fn p_numeric_grid(grid: &Vec<Vec<u8>>) {
    let mut s = String::with_capacity(grid.len() * grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = if grid[row][col] > 9 { '.' } else { (grid[row][col] + 48) as char };
            s.push(c);
        }
        s.push('\n');
    }
    println!("{s}");
    std::thread::sleep(SLEEP);
}
```

#### Datatype for grid position, row and column
```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    fn is_inside<T>(&self, grid: &Vec<Vec<T>>) -> bool {
        // Skip v < 0, it is not possible with usize (pointer values) anyway..
        self.row < grid.len() && self.col < grid[self.row].len()
    }
}
```

#### HashMap - Iterate over Key Value pair
```rust
use std::collections::HashMap;

let map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

for (key, val) in map.iter() {
    println!("key: {key} val: {val}");
}
```


#### HashMap - Insert or modify existing
```rust
use std::collections::HashMap;

let mut map = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
]);

let key = "a";
map.entry(key).and_modify(|key| *key += 2).or_insert(1);
```
