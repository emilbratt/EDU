<pre>
PNG spec. in short.. - https://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html

File structure has two main parts:
    PNG 8 byte signature (in decimal): "137 80 78 71 13 10 26 10"
    Then followed by a series of chunks

Each chunk has 4 parts:
Chunk layout:
    Length:
        A 4-byte unsigned integer giving the number of bytes in the chunk's data field.
        The length counts only the data field, not itself, the chunk type code, or the CRC.
        Zero is a valid length. Although encoders and decoders should treat the length as unsigned,
        its value must not exceed 231 bytes.
    Chunk Type:
        A 4-byte chunk type code. For convenience in description and in examining PNG files,
        type codes are restricted to consist of uppercase and lowercase ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal).
        However, encoders and decoders must treat the codes as fixed binary values, not character strings.
        For example, it would not be correct to represent the type code IDAT by the EBCDIC equivalents of those letters.
    Chunk Data:
        The data bytes appropriate to the chunk type, if any. This field can be of zero length.
    CRC:
        A 4-byte "Cyclic Redundancy Check" (CRC) calculated on the preceding bytes in the chunk,
        including the chunk type code and chunk data fields, but not including the length field.
        The CRC is always present, even for chunks containing no data.
        CRC algorithm -> https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#CRC-algorithm

    Note: all integer fields in the PNG file format are stored in big endian order.

First chunk is always IHDR (image header)
Last chunk is always IEND (image end of file)
</pre>

run tests..
```sh
cargo test -- --nocapture
```
