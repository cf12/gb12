/**
 * z    bit 7   Zero flag
 * n    bit 6   Subtraction flag (BCD)
 * h    bit 5   Half Carry flag (BCD)
 * c    bit 4   Carry flag
 */
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        (flags.z as u8) << 7 | (flags.n as u8) << 6 | (flags.h as u8) << 5 | (flags.c as u8) << 4
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags {
            z: (byte >> 7) & 0b1 == 1,
            n: (byte >> 6) & 0b1 == 1,
            h: (byte >> 5) & 0b1 == 1,
            c: (byte >> 4) & 0b1 == 1,
        }
    }
}
