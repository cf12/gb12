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

impl Registers {
    pub fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = (hl & 0xFF) as u8
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = (bc & 0xFF) as u8
    }

    pub fn set_de(&mut self, de: u16) {
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
