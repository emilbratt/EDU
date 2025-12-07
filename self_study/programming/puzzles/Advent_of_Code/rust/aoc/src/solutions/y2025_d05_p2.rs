const INPUT: &str = "y2025_d05.in";

pub fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();

    let mut split = input.split("\n\n");
    let ranges = split.next().unwrap();

    let mut ranges: Vec<(i64, i64)> = ranges.lines().map(|s| {
        let mut slt = s.split('-');
        (
            slt.next().unwrap().parse::<i64>().unwrap(),
            slt.next().unwrap().parse::<i64>().unwrap()
        )
    }).collect::<Vec<(i64, i64)>>();

    srt(&mut ranges);
    let ans = solve(&ranges);
    print!("{ans}");
}

fn srt(ranges: &mut [(i64, i64)]) {
    for i in 0..ranges.len()-1 {
        let mut j = i+1;
        for j in i..ranges.len() {
            if ranges[j].0 < ranges[i].0 {
                ranges.swap(i, j);
            }
        }
    }
}

fn solve(ranges: &[(i64, i64)]) -> i64 {
    let (mut ans, mut c) = (0, -1);

    for (mut s, e) in ranges {
        if c >= s {
            s = c+1;
        }
        if s <= *e {
            ans += *e-s+1;
        }
        c = c.max(*e);
    }

    ans
}