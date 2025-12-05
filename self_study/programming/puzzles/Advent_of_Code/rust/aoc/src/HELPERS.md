### Since we are isolating every solution, copy and paste these common functions.

#### Turn input into dimensional vector (grid)
```rust
fn gridify(input: Vec<u8>, is_integer: bool) -> Vec<Vec<u8>> {
    let subtract: u8 = if is_integer { b'0' } else { 0 };
    input.split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.iter()
                .map(|b| b-subtract)
                .collect()
        })
        .collect()
}
```

#### Print gridified input - Vec<Vec<u8>>
```rust
const NANO_SECONDS: u32 = 5000000;
const SLEEP: std::time::Duration = std::time::Duration::new(0, NANO_SECONDS);
fn p_grid(grid: &Vec<Vec<u8>>, is_integer: bool) {
    let add: u8 = if is_integer { b'0' } else { 0 };
    let mut s = String::with_capacity(grid.len() * grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = if grid[row][col] <= 9 { '.' } else { (grid[row][col] + add) as char };
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
