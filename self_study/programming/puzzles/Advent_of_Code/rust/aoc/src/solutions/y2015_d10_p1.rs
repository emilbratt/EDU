pub fn main() {
    let input = std::fs::read("y2015_d10.in").unwrap();
    let mut numbers: Vec<u32> = Vec::with_capacity(input.len()*100);
    for c in input {
        if c == b'\n' {
            continue;
        }
        numbers.push((c-48) as u32 );
    }
    let ans = solve(numbers, 40);
    print!("{ans}");
}

fn solve(mut v: Vec<u32>, iterations: usize) -> usize {
    for n in 0..iterations {
        let mut cur = v[0];
        let mut count = 0;
        let mut new: Vec<u32> = Vec::with_capacity(v.len());
        for i in 0..v.len() {
            if v[i] == cur {
                count += 1;
            } else {
                new.push(count);
                new.push(cur);
                count = 1;
                cur = v[i];
            }
        }
        new.push(count);
        new.push(cur);
        v = new;
    }
    v.len()
}
