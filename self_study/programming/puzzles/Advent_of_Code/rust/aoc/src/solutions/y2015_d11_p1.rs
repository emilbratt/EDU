pub fn main() {
    let mut pwd: Vec<u8> = std::fs::read("y2015_d11.in").unwrap();

    // remove trailing new lines if present..
    while *pwd.last().unwrap() == b'\n' {
        pwd.pop();
    }

    while !check(&pwd) {
        'inner: for i in (0..pwd.len()).rev() {
            if pwd[i] == b'z' {
                pwd[i] = b'a';
                continue 'inner;
            } else {
                pwd[i] += 1;
                while [b'i', b'o', b'l'].contains(&pwd[i]) {
                    pwd[i] += 1;
                }
                break 'inner;
            }
        }
    }

    let ans = unsafe {
        String::from_utf8_unchecked(pwd)
    };

    print!("{ans}");
}

fn check(pwd: &[u8]) -> bool {
    let mut has_straight = false;
    let mut has_two_pairs = false;

    let mut pairs = 0;
    let mut found_pair = false;
    for i in 0..pwd.len()-1 {
        if !has_two_pairs && !found_pair {
            if pwd[i] == pwd[i+1] {
                pairs += 1;
                found_pair = true;
            }
            if pairs == 2 {
                has_two_pairs = true;
            }
        } else {
            found_pair = false;
        }

        if !has_straight && i < pwd.len()-3 {
            let b1 = pwd[i];
            let b2 = pwd[i+1];
            let b3 = pwd[i+2];
            if b1+1 == b2 && b2+1 == b3 {
                has_straight = true;
            }
        }
    }

    has_straight && has_two_pairs
}
