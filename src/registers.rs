pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum RegisterPair {
    BC,
    DE,
    HL,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

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

impl Registers {
    fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0xFF) as u8
    }

    fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc & 0xFF) as u8
    }

    fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = (de & 0xFF) as u8
    }

    pub fn hl(&self) -> u16 {
        return (self.h as u16) << 8 | self.l as u16;
    }

    pub fn bc(&self) -> u16 {
        return (self.b as u16) << 8 | self.c as u16;
    }

    pub fn de(&self) -> u16 {
        return (self.d as u16) << 8 | self.e as u16;
    }
}
