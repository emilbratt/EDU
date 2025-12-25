use std::fs;

pub const INPUT: &str = "y2015_d04.in";

pub fn main() {
    let mut input = fs::read(INPUT).unwrap();
    if *input.last().unwrap() == b'\n' {
        input.pop();
    }

    let mut i: u64 = 0;
    let mut c_in: Vec<u8> = Vec::with_capacity(70);
    loop {
        i += 1;

        for b in input.iter() {
            c_in.push(*b);
        }
        for c in i.to_string().chars() {
            c_in.push(c as u8);
        }

        let hash = md5_from_google_gemini(&c_in);
        if hash.starts_with("00000") {
            break;
        } else {
            c_in.clear();
        }
    }

    print!("{}", i);
}

fn md5_from_google_gemini(input: &[u8]) -> String {
    // 1. Initial State (Magic Numbers)
    let mut a: u32 = 0x67452301;
    let mut b: u32 = 0xefcdab89;
    let mut c: u32 = 0x98badcfe;
    let mut d: u32 = 0x10325476;

    // 2. Pre-processing: Padding
    let mut msg = input.to_vec();
    let bit_len = (msg.len() as u64) * 8;

    // Append '1' bit (0x80 byte)
    msg.push(0x80);

    // Pad with zeros until length mod 64 is 56 bytes (448 bits)
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }

    // Append length as 64-bit little-endian
    msg.extend_from_slice(&bit_len.to_le_bytes());

    // 3. Constants (Sine Table) and Shift Amounts
    let s: [u32; 64] = [
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
    ];

    let k: [u32; 64] = (0..64).map(|i| {
        (((i + 1) as f64).sin().abs() * 4294967296.0) as u32
    }).collect::<Vec<u32>>().try_into().unwrap();

    // 4. Main Loop: Process 512-bit blocks
    for chunk in msg.chunks(64) {
        let mut words = [0u32; 16];
        for i in 0..16 {
            words[i] = u32::from_le_bytes(chunk[i*4..i*4+4].try_into().unwrap());
        }

        let (mut aa, mut bb, mut cc, mut dd) = (a, b, c, d);

        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((bb & cc) | (!bb & dd), i),
                16..=31 => ((bb & dd) | (cc & !dd), (5 * i + 1) % 16),
                32..=47 => (bb ^ cc ^ dd, (3 * i + 5) % 16),
                48..=63 => (cc ^ (bb | !dd), (7 * i) % 16),
                _ => unreachable!(),
            };

            let temp = dd;
            dd = cc;
            cc = bb;
            bb = bb.wrapping_add(
                (aa.wrapping_add(f).wrapping_add(k[i]).wrapping_add(words[g]))
                .rotate_left(s[i])
            );
            aa = temp;
        }

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    // 5. Output (Little-endian hex)
    format!("{:08x}{:08x}{:08x}{:08x}",
        a.swap_bytes(), b.swap_bytes(), c.swap_bytes(), d.swap_bytes())
}
