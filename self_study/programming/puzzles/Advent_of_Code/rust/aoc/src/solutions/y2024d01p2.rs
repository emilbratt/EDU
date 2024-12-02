use std::fs;

pub fn main() {
    let input = fs::read_to_string("y2024d01.in").unwrap();

    let mut res: i64 = 0;

    let capacity = input.lines().count();

    let mut v_l: Vec<i64> = Vec::with_capacity(capacity);
    let mut v_r: Vec<i64> = Vec::with_capacity(capacity);

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let left = split.next().unwrap();
        let right = split.next().unwrap();

        v_l.push(left.parse::<i64>().unwrap());
        v_r.push(right.parse::<i64>().unwrap());
    }

    for i in 0..capacity {
        let a = v_l[i];
        let mut r_f: i64 = 0;
        for j in 0..capacity {
            let b = v_r[j];
            if a == b {
                r_f += 1;
            }
        }

        res += (a * r_f);
    }

    print!("{res}");
}
