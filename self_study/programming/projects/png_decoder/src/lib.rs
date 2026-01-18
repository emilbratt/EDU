#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::io::{self, BufReader, Read};

use byteorder::{BigEndian, ReadBytesExt};

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

struct Chunk {
    chunk_type: [u8; 4],
    data: Vec<u8>,
}

// just to print out the hex values..
fn hex(bytes: &[u8]) -> String {
    println!("bytes: {:?}", bytes);
    let mut s = String::with_capacity(bytes.len());
    for b in bytes {
        let fmt = format!("{:X}", b);
        s.push_str( format!("{:0>2} ", fmt).as_str() );
    }
    println!("Hex: {}\n", s);
    s
}

fn decode_ihdr(bytes: &[u8]) -> io::Result<()> {
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

    Ok(())
}

fn decode_plte(bytes: &[u8]) -> io::Result<()> {
    println!("decode_plte()\n{:?}", bytes);
    unimplemented!();
}

fn decode_idat(bytes: &[u8]) -> io::Result<()> {
    println!("decode_idat()\n{:?}", bytes);
    unimplemented!();
}

fn read_chunks<R: Read>(rdr: &mut R) -> io::Result<()> {
    let mut chunk_type = [0u8; 4];

    loop {
        // CHUNK LENGTH
        // reads 4 bytes into a u32 type -> read_u32 using the big-endian byte order.
        let length = match rdr.read_u32::<BigEndian>() {
            Ok(len) => {
                len as usize
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                println!("unexpected end of file..");
                return Err(e);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(e);
            }
        };

        // CHUNK TYPE
        // reads 4 bytes into chunk_type.
        rdr.read_exact(&mut chunk_type)?;
        let chunk_name = str::from_utf8(&chunk_type).unwrap_or("????");

        // CHUNK DATA
        let mut data = vec![0u8; length];
        rdr.read_exact(&mut data)?;

        // CHUNK CRC (Cyclic Redundancy Check)
        let crc = rdr.read_u32::<BigEndian>()?;

        match chunk_name {
            "IHDR" => decode_ihdr(&data),
            "PLTE" => decode_plte(&data),
            "IDAT" => decode_idat(&data),
            _ => {
                println!("{}", chunk_name);
                Ok(())
            }
        };

        // println!("{chunk_name} | {length} bytes | crc=0x{:08x})\ndata: {:?}\n", crc, data);
        if chunk_name == "IEND" {
            println!("IEND reached..");
            return Ok(());
        }

        println!();
    }

    panic!("No end of file marker..");
}

fn read_png<R: Read>(rdr: &mut R) -> io::Result<()> {
    let mut signature = [0u8; 8];
    rdr.read_exact(&mut signature);
    hex(&signature);

    if signature == PNG_SIGNATURE {
        read_chunks(rdr)
    } else {
        let (kind, err) = (io::ErrorKind::InvalidData, "not a valid PNG file");
        Err(io::Error::new(kind,err))
    }
}

pub fn load() -> io::Result<()> {
    let mut signature_buf = [0u8; 8];
    let mut chunk_buf = [0u8; 4];
    let f = File::open("images/2x2.png").unwrap();
    let mut rdr = BufReader::new(f);
    read_png(&mut rdr)
    // let n = rdr.read(&mut signature_buf)?;
    // assert_eq!(n, 8);
    // assert_eq!(PNG_SIGNATURE, signature_buf);
    // hex(&signature_buf);

    // rdr.read(&mut chunk_buf);
    // hex(&chunk_buf);

    // Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        load();
    }
}
