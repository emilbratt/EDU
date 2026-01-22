#![allow(unused)]

use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;

const PNG_SIGNATURE: [u8; 8] = [137, b'P', b'N', b'G', 13, 10, 26, 10];

mod png_clr;
mod png_unfilter;

use png_unfilter::unfilter;
use png_clr::{CHRM, GAMA, PHYS, SRGB};

#[derive(Default, Debug)]
enum InterlaceMethod {
    #[default]
    NoInterlace = 0,
    Adam7 = 1,
}

impl TryFrom<u8> for InterlaceMethod {
    type Error = &'static str;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        if b <= 1 {
            Ok(unsafe { std::mem::transmute(b) })
        } else {
            Err("Interlace method byte value must be between 0 and 1")
        }
    }
}

#[derive(Default, Debug)]
pub struct PngImage {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: InterlaceMethod,
    pixels: Vec<u8>,
}

impl PngImage {
    pub fn rows(&self) -> Vec<&[u8]> {
        let width = self.width as usize;
        let height = self.height as usize;
        let row_len = width * 4; // there are 4 channels (RGBA) for every pixel..

        let mut rows = Vec::with_capacity(height);
        for y in 0..height {
            let start = y * row_len;
            let end = start + row_len;
            rows.push(&self.pixels[start..end]);
        }

        rows
    }
}

#[derive(Debug)]
struct Chunk {
    c_type: [u8; 4],
    data: Vec<u8>,
    crc: u32, // FIXME: implement cyclic redundency check..
}

// just to print out the hex values..
fn hex(bytes: &[u8]) {
    println!("bytes: {:?}", bytes);
    let mut s = String::with_capacity(bytes.len());
    for b in bytes {
        let fmt = format!("{:X}", b);
        s.push_str( format!("{:0>2} ", fmt).as_str() );
    }
    println!("Hex: {}\n", s);
}

fn decode_ihdr(bytes: [u8; 13]) -> io::Result<PngImage> {
    let mut cursor = std::io::Cursor::new(bytes);

    let width = cursor.read_u32::<BigEndian>()?;
    let height = cursor.read_u32::<BigEndian>()?;

    // single-byte integer giving the number of bits per sample
    // Valid values are 0, 2, 3, 4, 8, and 16.
    let bit_depth = cursor.read_u8()?;
    assert!([0,2,4,8,16].contains(&bit_depth)); // FIXME: handle gracefully..,

    // Color type is a single-byte integer that describes the interpretation of the image data. Color type codes
    // represent sums of the following values:
    //  - 1 (palette used),
    //  - 2 (color used)
    //  - 4 (alpha channel used).
    // Valid values are  0, 2, 3, 4, or 6.
    let color_type = cursor.read_u8()?;
    assert!([0,2,3,4,6].contains(&color_type)); // FIXME: handle gracefully..,

    // Compression method is a single-byte integer that indicates the method used to compress the image data.
    // At present, only compression method 0 (deflate/inflate compression with a sliding window of at most 32768 bytes)
    // is defined. All standard PNG images must be compressed with this scheme.
    let compression_method = cursor.read_u8()?;
    assert_eq!(compression_method, 0); // FIXME: handle gracefully..

    // Filter method is a single-byte integer that indicates the preprocessing method applied to the image data before
    // compression. At present, only filter method 0 (adaptive filtering with five basic filter types) is defined.
    let filter_method = cursor.read_u8()?;
    assert_eq!(filter_method, 0);

    // Interlace method is a single-byte integer that indicates the transmission order of the image data. Two values
    // are currently defined: 0 (no interlace) or 1 (Adam7 interlace).
    let b = cursor.read_u8()?;
    let interlace_method = match InterlaceMethod::try_from(b) {
        Ok(v) => v,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
    };

    Ok(PngImage {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
        pixels: Vec::new(),
    })
}

fn next_chunk<R: Read>(rdr: &mut R) -> io::Result<Chunk> {
    let chunk_len = match rdr.read_u32::<BigEndian>() {
        Ok(len) => len as usize,
        Err(e) => return Err(e),
    };

    // CHUNK TYPE
    // reads 4 bytes into chunk_type.
    let mut c_type = [0u8; 4];
    rdr.read_exact(&mut c_type)?;

    // CHUNK DATA
    let mut data = vec![0u8; chunk_len];
    rdr.read_exact(&mut data)?;

    // CHUNK CRC (Cyclic Redundancy Check)
    let crc = rdr.read_u32::<BigEndian>()?;

    Ok(Chunk { c_type, data, crc })
}

pub fn decode(path: &Path) -> io::Result<PngImage> {
    let f = File::open(path)?;
    let mut rdr = BufReader::new(f);

    // PNG SIGNATURE
    let mut first_eight_bytes = [0u8; 8];
    rdr.read_exact(&mut first_eight_bytes);
    hex(&first_eight_bytes);

    if first_eight_bytes != PNG_SIGNATURE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,"not a PNG file (did not start with png magic bytes")
        );
    }

    // PNG HEADER
    // next chunk must be the "IHDR" type.
    let ihdr_data_len = rdr.read_u32::<BigEndian>()?;
    if ihdr_data_len != 13 {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data length for IHDR chunk. Expected length is 13"
            )
        );
    }
    // Verify that it is in fact "IHDR"
    let mut next_four_bytes = [0u8; 4];
    rdr.read_exact(&mut next_four_bytes);
    if &next_four_bytes != b"IHDR" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "First PNG chunk is not of type \"IHDR\"")
        );
    }

    // data len = 13 bytes, crc len = 4 bytes, total = 17 bytes.
    let mut ihdr_buf = [0u8; 13];
    rdr.read_exact(&mut ihdr_buf);
    let _ihdr_crc = rdr.read_u32::<BigEndian>(); // ignored for now..
    let mut png = decode_ihdr(ihdr_buf)?;

    if png.bit_depth != 8 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Only 8-bit PNG supported")
        );
    }
    if png.color_type != 2 && png.color_type != 6 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Only RGB/RGBA PNG supported")
        );
    }

    let mut idat: Vec<u8> = Vec::new();

    // FIXME: these are not in use, they are only loaded when their respective chunk is found..
    let mut srgb: Option<SRGB> = None;
    let mut gama: Option<GAMA> = None;
    let mut chrm: Option<CHRM> = None;
    let mut phys: Option<PHYS> = None;
    let mut plte: Option<&str> = None; // FIXME: implement proper type..

    // All good up until here, lets decode the remaining png chunks
    loop {
        let chunk = next_chunk(&mut rdr)?;
        let name = str::from_utf8(&chunk.c_type).unwrap();
        println!("{name} -> data length: {}", chunk.data.len());

        match &chunk.c_type {
            b"PLTE" => {
                // FIXME: handle gracefully (remove assertion)
                assert!(idat.is_empty()); // ..must precede the first IDAT chunk
                plte = Some("please implement me :)");
            }
            b"pHYs" => {
                // FIXME: handle gracefully (remove assertion)
                assert!(idat.is_empty()); // ..must precede the first IDAT chunk
                match PHYS::try_from(chunk.data.as_slice()) {
                    Ok(v) => phys = Some(v),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
                }
            }
            b"cHRM" => {
                // FIXME: handle gracefully (remove assertion)
                assert!(idat.is_empty()); // ..must precede the first IDAT chunk
                assert!(plte.is_none());  // ..must precede PLTE chunk
                match CHRM::try_from(chunk.data.as_slice()) {
                    Ok(v) => chrm = Some(v),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
                }
            }
            b"sRGB" => {
                // FIXME: handle gracefully (remove assertion)
                assert!(idat.is_empty()); // ..must precede the first IDAT chunk
                assert!(plte.is_none());  // ..must precede PLTE chunk
                match SRGB::try_from(chunk.data.as_slice()) {
                    Ok(v) => srgb = Some(v),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
                }
            }
            b"gAMA" => {
                // FIXME: handle gracefully (remove assertion)
                assert!(idat.is_empty()); // ..must precede the first IDAT chunk
                assert!(plte.is_none());  // ..must precede PLTE chunk
                match GAMA::try_from(chunk.data.as_slice()) {
                    Ok(v) => gama = Some(v),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
                }
            }
            b"IDAT" => idat.extend_from_slice(&chunk.data),
            b"IEND" => break,
            _ => println!("FIXME: chunk type '{name}' is not implemented!"),
        }
    }
    println!("decode complete!\n");

    let bpp = match png.color_type {
        2 => 3,
        6 => 4,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData,"Unsupported color type")),
    };

    let mut idat_raw = Vec::new();
    let mut decoder = ZlibDecoder::new(&*idat);
    let mut out = Vec::new();
    match decoder.read_to_end(&mut out).map_err(|e| e.to_string()) {
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
        Ok(_) => idat_raw = out,
    }

    let pixels = match unfilter(&idat_raw, png.width as usize, png.height as usize, bpp) {
        Ok(raw) => raw,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
    };

    if let Some(srgb) = srgb { println!("srgb: {:?}", srgb); }
    if let Some(gama) = gama { println!("gama: {:?}", gama); }
    if let Some(chrm) = chrm { println!("chrm: {:?}", chrm); }
    if let Some(phys) = phys { println!("phys: {:?}", phys); }

    println!("width: {}", png.width);
    println!("height: {}", png.height);
    println!("bit_depth: {}", png.bit_depth);
    println!("color_type: {}", png.color_type);
    println!("compression_method: {}", png.compression_method);
    println!("filter_method: {}", png.filter_method);
    println!("interlace_method: {:?}", png.interlace_method);

    println!("\nSize: {}x{} | bit depth: {} | clr type: {}", png.width, png.height, png.bit_depth, png.color_type);

    let image = match png.color_type {
        2 => add_alpha_channel(&pixels),
        6 => pixels,
        _ => unreachable!(),
    };

    png.pixels = image;

    Ok(png)
}

// if RGB, convert to RGBA by adding full opacity (255) on all pixels
fn add_alpha_channel(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() / 3 * 4);
    for chunk in data.chunks_exact(3) {
        out.extend_from_slice(chunk);
        out.push(255);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_by_two() {
        let png = decode(Path::new("images/3x2.png")).unwrap();
        let rows = png.rows();
        let mut c = 1;
        for row in rows.iter() {
            println!("row: {c}");
            c+=1;
            let mut i = 0;
            while i < row.len() {
                print!("(R-{:0>3}) ", row[i]);
                print!("(G-{:0>3}) ", row[i+1]);
                print!("(B-{:0>3}) ", row[i+2]);
                print!("(A-{:0>3}) ", row[i+3]);
                print!("| ");
                i += 4;
            }
            println!();
        }
    }
}
