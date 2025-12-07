const INPUT: &str = "y2025_d05.in";

pub fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut split = input.split("\n\n");
    let ranges = split.next().unwrap();
    let ids = split.next().unwrap();

    let ranges: Vec<(u64, u64)> = ranges.lines().map(|s| {
        let mut slt = s.split('-');
        (
            slt.next().unwrap().parse::<u64>().unwrap(),
            slt.next().unwrap().parse::<u64>().unwrap()
        )
    }).collect::<Vec<(u64, u64)>>();

    let ids: Vec<u64> = ids.lines().map(|s| {
        s.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>();

    let mut ans = 0_i64;
    for id in ids {
        for (lo, hi) in ranges.iter() {
            if id >= *lo && id <= *hi {
                ans += 1;
                break;
            }
        }
    }

    print!("{ans}");
}
