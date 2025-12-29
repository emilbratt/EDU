use std::collections::{HashMap, VecDeque};

pub const INPUT: &str = "y2015_d07.in";

#[derive(Clone)]
enum Instruction {
    Assign(String, String),
    Not(String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
}

pub fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let mut instructions_a: Vec<Instruction> = Vec::new();
    let mut instructions_b: Vec<Instruction> = Vec::new();
    for l in input.lines() {
        let instr = parse(l);
        instructions_a.push(instr.clone());
        instructions_b.push(instr);
    }

    let b = resolve("a", VecDeque::from(instructions_a));

    for instr in instructions_b.iter_mut() {
        if let Instruction::Assign(v, out) = instr {
            if out == "b" {
                *instr = Instruction::Assign(b.to_string(), out.to_string());
                break;
            }
        }
    }

    let ans = resolve("a", VecDeque::from(instructions_b));
    print!("{ans}");
}

fn parse(line: &str) -> Instruction {
    let mut split = line.split(" -> ");
    let instr = split.next().unwrap().split_whitespace().collect::<Vec<&str>>();
    let out = split.next().unwrap();

    match *instr.as_slice() {
        [a] => {
            Instruction::Assign(a.to_string(), out.to_string())
        }
        ["NOT", a] => {
            Instruction::Not(a.to_string(), out.to_string())
        }
        [a, "AND", b] => {
            Instruction::And(a.to_string(), b.to_string(), out.to_string())
        }
        [a, "OR", b] => {
            Instruction::Or(a.to_string(), b.to_string(), out.to_string())
        }
        [a, "LSHIFT", n] => {
            Instruction::LShift(a.to_string(), n.parse().unwrap(), out.to_string())
        }
        [a, "RSHIFT", n] => {
            Instruction::RShift(a.to_string(), n.parse().unwrap(), out.to_string())
        }
        _ => unreachable!("unknown Instructionuction: '{:?}'", instr),
    }
}

fn resolve(wire: &str, mut instructions: VecDeque<Instruction>) -> u16 {
    let mut signals: HashMap<String, u16> = HashMap::new();

    while let Some(instr) = instructions.pop_front() {
        let resolved = match &instr {
            Instruction::Assign(a, out) => {
                get_value(a, &signals).map(|v| (out, v))
            }
            Instruction::Not(a, out) => {
                get_value(a, &signals).map(|v| (out, !v))
            }
            Instruction::And(a, b, out) => {
                get_value(a, &signals)
                    .zip(get_value(b, &signals))
                    .map(|(x, y)| (out, x & y))
            }
            Instruction::Or(a, b, out) => {
                get_value(a, &signals)
                    .zip(get_value(b, &signals))
                    .map(|(x, y)| (out, x | y))
            }
            Instruction::LShift(a, n, out) => {
                get_value(a, &signals).map(|v| (out, v << n))
            }
            Instruction::RShift(a, n, out) => {
                get_value(a, &signals).map(|v| (out, v >> n))
            }
        };

        if let Some((out, val)) = resolved {
            // add signal to wire
            if out == wire {
                return val;
            }
            signals.insert(out.clone(), val);
        } else {
            // re-try later when values are available
            instructions.push_back(instr);
        }
    }

    panic!("Value for '{wire}' could not be resolved..");
}

fn get_value(s: &str, signals: &HashMap<String, u16>) -> Option<u16> {
    if let Some(n ) = signals.get(s) {
        Some(*n)
    } else if let Ok(n) = s.parse::<u16>() {
        Some(n)
    } else {
        None
    }
}
