enum ScopeType {
    Object(bool), // bool => ignore if has value "red"
    Array,
}

pub fn main() {
    let input = std::fs::read("y2015_d12.in").unwrap();

    let mut ans = 0;
    let mut stack: Vec<(i32, ScopeType)> = Vec::new();
    let mut cur = 0i32;
    let mut sign = 1i32;
    let mut in_num = false; // did last iteration contain digit?
    let mut i = 0usize;
    while i < input.len() {
        match input[i] {
            b'{' => stack.push((0, ScopeType::Object(false))),
            b'[' => stack.push((0, ScopeType::Array)),
            b'-' => sign = -1,
            b'}' | b']' => {
                if in_num {
                    stack.last_mut().unwrap().0 += cur * sign;
                    cur = 0;
                    sign = 1;
                    in_num = false;
                }

                let (sum, s) = stack.pop().unwrap();
                let has_red = match s {
                    ScopeType::Object(b) if b => true,
                    _ => false,
                };
                if !has_red {
                    match stack.last_mut() {
                        Some((acc, _)) => *acc += sum,
                        None => ans = sum, // stack is empty => end of while loop..
                    }
                }
            }
            b'0'..=b'9' => {
                cur = cur * 10 + (input[i] - b'0') as i32;
                in_num = true;
            }
            b'"' => {
                i += 1;
                let start = i;
                while input[i] != b'"' {
                    i += 1;
                }
                if &input[start..i] == b"red" {
                    if let Some((_, scope)) = stack.last_mut() {
                        if let ScopeType::Object(b) = scope {
                            *scope = ScopeType::Object(true);
                        }
                    }
                }
            }
            _ => {
                if in_num {
                    stack.last_mut().unwrap().0 += cur * sign;
                    cur = 0;
                    sign = 1;
                    in_num = false;
                }
            }
        }

        i += 1;
    }

    print!("{ans}");
}
