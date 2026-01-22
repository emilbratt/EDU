
pub fn unfilter(data: &[u8], width: usize, height: usize, bpp: usize) -> Result<Vec<u8>, &'static str> {
    let stride = width * bpp;
    let expected_len = height * (stride + 1);

    if data.len() != expected_len {
        return Err("Invalid filtered data length");
    }

    let mut out = vec![0u8; height * stride];
    let mut i = 0;

    for y in 0..height {
        let filter = data[i];
        i += 1;

        let (above, current_and_below) = out.split_at_mut(y * stride);

        let prev_row = if y == 0 {
            None
        } else {
            Some(&above[(y - 1) * stride..y * stride])
        };

        let cur_row = &mut current_and_below[..stride];

        // Copy filtered bytes
        cur_row.copy_from_slice(&data[i..i + stride]);

        match filter {
            0 => {}
            1 => filter_sub(cur_row, bpp),
            2 => filter_up(cur_row, prev_row),
            3 => filter_avg(cur_row, prev_row, bpp),
            4 => filter_paeth(cur_row, prev_row, bpp),
            _ => return Err("Unknown PNG filter"),
        }

        i += stride;
    }

    Ok(out)
}

fn filter_sub(row: &mut [u8], bpp: usize) {
    for i in bpp..row.len() {
        row[i] = row[i].wrapping_add(row[i - bpp]);
    }
}

fn filter_up(row: &mut [u8], prev: Option<&[u8]>) {
    if let Some(prev) = prev {
        for i in 0..row.len() {
            row[i] = row[i].wrapping_add(prev[i]);
        }
    }
}

fn filter_avg(row: &mut [u8], prev: Option<&[u8]>, bpp: usize) {
    for i in 0..row.len() {
        let left = if i >= bpp { row[i - bpp] } else { 0 };
        let up = prev.map_or(0, |p| p[i]);
        row[i] = row[i].wrapping_add(((left as u16 + up as u16) / 2) as u8);
    }
}

fn filter_paeth(row: &mut [u8], prev: Option<&[u8]>, bpp: usize) {
    for i in 0..row.len() {
        let a = if i >= bpp { row[i - bpp] } else { 0 };
        let b = prev.map_or(0, |p| p[i]);
        let c = if i >= bpp {
            prev.map_or(0, |p| p[i - bpp])
        } else {
            0
        };

        row[i] = row[i].wrapping_add(paeth(a, b, c));
    }
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let a = a as i32;
    let b = b as i32;
    let c = c as i32;

    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();

    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}
