#[derive(Debug)]
pub struct GAMA(u32);

impl TryFrom<&[u8]> for GAMA {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 4 {
            Err("gAMA chunk must be exactly 4 bytes")
        } else {
            Ok(GAMA
                (u32::from_be_bytes(bytes[0..4].try_into().unwrap()))
            )
        }
    }
}

#[derive(Debug)]
pub struct CHRM {
   white_point_x: u32,
   white_point_y: u32,
   red_x:         u32,
   red_y:         u32,
   green_x:       u32,
   green_y:       u32,
   blue_x:        u32,
   blue_y:        u32,
}

impl TryFrom<&[u8]> for CHRM {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 32 {
            return Err("cHRM chunk must be exactly 32 bytes");
        }

        // helper closure for clarity
        let read_u32 = |i: usize| -> u32 {
            u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap())
        };

        let chrm = Self {
            white_point_x: read_u32(0),
            white_point_y: read_u32(4),
            red_x:         read_u32(8),
            red_y:         read_u32(12),
            green_x:       read_u32(16),
            green_y:       read_u32(20),
            blue_x:        read_u32(24),
            blue_y:        read_u32(28),
        };

        if chrm.white_point_x == 0 || chrm.white_point_y == 0 {
            // FIXME: The PNG spec allows decoders to ignore invalid cHRM chunks.
            Err("cHRM chunk contains an invalid white point")
        } else {
            Ok(chrm)
        }
    }
}

#[derive(Debug)]
pub struct PHYS {
    pixel_per_unit_x: u32,
    pixel_per_unit_y: u32,
    unit_specifier: u8,
}
impl TryFrom<&[u8]> for PHYS {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 9 {
            return Err("pHYs chunk must be exactly 9 bytes");
        }

        let phys = Self {
            pixel_per_unit_x: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            pixel_per_unit_y: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            unit_specifier: u8::from_be_bytes(bytes[8..9].try_into().unwrap()),
        };

        if phys.unit_specifier > 1 {
            Err("pHYs chunk contains an invalid unit specifier")
        } else {
            Ok(phys)
        }
    }
}

#[derive(Debug)]
pub enum SRGB {
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl TryFrom<&[u8]> for SRGB {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 1 {
            return Err("sRGB chunk must be exactly 1 byte");
        }

        let b = u8::from_be(bytes[0]);
        if b <= 3 {
            Ok(unsafe { std::mem::transmute(b) })
        } else {
            Err("sRGB chunk must contain a byte value between 0 and 3")
        }
    }
}
