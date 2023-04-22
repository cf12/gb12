use crate::{memory::{Memory}, registers::{Registers, Flags}, ops::Target};


pub struct Cpu {
    regs: Registers,
    flags: Flags,
    mem: Memory,
    pc: u16,
}

impl Cpu {
    fn nb(&mut self) -> u8 {
        self.pc += 1;
        self.mem.read_byte(self.pc - 1)
    }

    fn nw(&mut self) -> u16 {
        self.pc += 2;
        (self.mem.read_byte(self.pc - 1) << 8) as u16 | (self.mem.read_byte(self.pc - 2) as u16)
    }

    fn op_ld(&mut self, t1: Target, t2: Target) {
        let value = match t2 {
            Value()
        }
        match t1 {
            R::B => self.regs.b = t2,
            R::C => self.regs.c = t2,
            R::D => self.regs.d = t2,
            R::E => self.regs.e = t2,
            R::H => self.regs.h = t2,
            R::L => self.regs.l = t2,
            _ => panic!("invalid op_ld")
        }
    }

    fn cycle(&mut self) {
        let op = self.nb();

        match op {
            // LD nn, n
            0x06 => self.regs.b = self.nb(),
            0x0E => self.regs.c = self.nb(),
            0x16 => self.regs.d = self.nb(),
            0x1E => self.regs.e = self.nb(),
            0x26 => self.regs.h = self.nb(),
            0x2E => self.regs.l = self.nb(),
            _ => panic!("invalid op {:02X}", op)
        }
    }
}
