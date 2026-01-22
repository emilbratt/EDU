#![allow(unused)]

use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;

const PNG_SIGNATURE: [u8; 8] = [137, b'P', b'N', b'G', 13, 10, 26, 10];

mod png_clr;
use png_clr::{CHRM, PHYS, SRGB};

#[derive(Default, Debug)]
pub struct Png {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    data: Vec<u8>,
}

// FIXME: make a zero-copy (no un-needed mem-allocation) in the future..
#[derive(Debug)]
struct Chunk {
    c_type: [u8; 4],
    data: Vec<u8>,
    crc: u32,
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

// IHDR    multiple: No      Must be first
fn decode_ihdr(bytes: [u8; 13]) -> io::Result<Png> {

    println!("decode_ihdr()\n{:?}", bytes);

    let mut cursor = std::io::Cursor::new(bytes);

    let width = cursor.read_u32::<BigEndian>()?;
    let height = cursor.read_u32::<BigEndian>()?;
    println!("size {width}x{height}");

    // single-byte integer giving the number of bits per sample
    let bit_depth = cursor.read_u8()?;
    println!("bit_depth: {bit_depth}");

    // Color type is a single-byte integer that describes the interpretation of the image data. Color type codes
    // represent sums of the following values:
    //  - 1 (palette used),
    //  - 2 (color used)
    //  - 4 (alpha channel used).
    // Valid values are 0, 2, 3, 4, and 6.
    let color_type = cursor.read_u8()?;
    println!("color_type: {color_type}");

    // Compression method is a single-byte integer that indicates the method used to compress the image data.
    // At present, only compression method 0 (deflate/inflate compression with a sliding window of at most 32768 bytes)
    // is defined. All standard PNG images must be compressed with this scheme.
    let compression = cursor.read_u8()?;
    println!("compression: {compression}");

    // Filter method is a single-byte integer that indicates the preprocessing method applied to the image data before
    // compression. At present, only filter method 0 (adaptive filtering with five basic filter types) is defined.
    let filter = cursor.read_u8()?;
    println!("filter: {filter}");

    // Interlace method is a single-byte integer that indicates the transmission order of the image data. Two values
    // are currently defined: 0 (no interlace) or 1 (Adam7 interlace).
    let interlace = cursor.read_u8()?;
    println!("interlace: {interlace}");

    let png = Png {
        width,
        height,
        bit_depth,
        color_type,
        data: Vec::new(),
    };

    Ok(png)
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

pub fn decode(path: &Path) -> io::Result<Png> {
    let f = File::open(path)?;
    let mut rdr = BufReader::new(f);

    // PNG SIGNATURE
    let mut first_eight_bytes = [0u8; 8];
    rdr.read_exact(&mut first_eight_bytes);
    hex(&first_eight_bytes);

    if first_eight_bytes != PNG_SIGNATURE {
        let (kind, err) = (io::ErrorKind::InvalidData, "not a PNG file (did not start with png magic bytes");
        return Err(io::Error::new(kind,err));
    }

    // PNG HEADER
    // next chunk must be the "IHDR" type.
    let ihdr_data_len = rdr.read_u32::<BigEndian>()?;
    if ihdr_data_len != 13 {
        let (kind, err) = (
            io::ErrorKind::InvalidData,
            format!("Invalid data length for IHDR chunk. Data length = '{}', but expected length is 13", ihdr_data_len)
        );
        return Err(io::Error::new(kind,err));
    }
    // Verify that it is in fact "IHDR"
    let mut next_four_bytes = [0u8; 4];
    rdr.read_exact(&mut next_four_bytes);
    if &next_four_bytes != b"IHDR" {
        let (kind, err) = (io::ErrorKind::InvalidData, "First PNG chunk is not of type \"IHDR\"");
        return Err(io::Error::new(kind,err));
    }

    // data len = 13 bytes, crc len = 4 bytes, total = 17 bytes.
    let mut ihdr_buf = [0u8; 13];
    rdr.read_exact(&mut ihdr_buf);
    let _ihdr_crc = rdr.read_u32::<BigEndian>(); // ignored for now..
    let mut png = decode_ihdr(ihdr_buf)?;

    if png.bit_depth != 8 {
        let (kind, err) = (io::ErrorKind::InvalidData, "Only 8-bit PNG supported");
        return Err(io::Error::new(kind,err));
    }
    if png.color_type != 2 && png.color_type != 6 {
        let (kind, err) = (io::ErrorKind::InvalidData, "Only RGB/RGBA PNG supported");
        return Err(io::Error::new(kind,err));
    }

    let mut srgb: Option<SRGB> = None;
    let mut gama: Option<u32> = None;
    let mut chrm: Option<CHRM> = None;
    let mut phys: Option<PHYS> = None;

    // All good, lets decode the remaining png chunks
    loop {
        let chunk = next_chunk(&mut rdr)?;

        let name = str::from_utf8(&chunk.c_type).unwrap();
        println!("\n'{name}', length: {}, data: {:?}", chunk.data.len(), chunk.data);

        match &chunk.c_type {
            b"pHYs" => {
                match PHYS::try_from(chunk.data.as_slice()) {
                    Ok(v) => phys = Some(v),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
                }
            }
            b"cHRM" => {
                match CHRM::try_from(chunk.data.as_slice()) {
                    Ok(v) => chrm = Some(v),
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData,e)),
                }
            }
            b"gAMA" => {
                if chunk.data.len() != 4 {
                    let (kind, err) = (io::ErrorKind::InvalidData, "gAMA chunk: data field has wrong length");
                    return Err(io::Error::new(kind,err));
                }
                gama = Some(
                    u32::from_be_bytes(chunk.data[0..4].try_into().unwrap())
                );
            }
            b"sRGB" => {
                if chunk.data.len() != 1 {
                    let (kind, err) = (io::ErrorKind::InvalidData, "sRGB chunk: data field has wrong length");
                    return Err(io::Error::new(kind,err));
                }
                if let Ok(v) = SRGB::try_from(chunk.data[0]) {
                    srgb = Some(v);
                }
            }
            b"IDAT" => png.data.extend_from_slice(&chunk.data),
            b"IEND" => break,
            _ => println!("FIXME: chunk type '{name}' is not implemented!"),
        }
    }


    let mut decoder = ZlibDecoder::new(&*png.data);
    let mut out = Vec::new();
    if let Err(e) = decoder.read_to_end(&mut out).map_err(|e| e.to_string()) {
        return Err(io::Error::new(io::ErrorKind::InvalidData,e))
    }

    png.data = out;

    Ok(png)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_by_two() {
        let png = decode(Path::new("images/3x2.png")).unwrap();
        println!("\nSize: {}x{} | bit depth: {} | clr type: {}", png.width, png.height, png.bit_depth, png.color_type);
        println!("idat: {:?}", png.data);
    }
}
