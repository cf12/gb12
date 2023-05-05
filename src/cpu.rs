use crate::{flags::Flags, registers::Registers};

const MEMORY_SIZE: usize = 0xFFFF;

pub struct Cpu {
    regs: Registers,
    flags: Flags,
    mem: [u8; MEMORY_SIZE],
    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: 0,
                h: 0,
                l: 0,
            },
            flags: Flags {
                z: false,
                n: false,
                h: false,
                c: false,
            },
            mem: [0; MEMORY_SIZE],
            pc: 0,
            sp: 0,
        }
    }

    fn rb(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn wb(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value
    }

    fn nb(&mut self) -> u8 {
        self.pc += 1;
        self.rb(self.pc - 1)
    }

    fn nw(&mut self) -> u16 {
        self.pc += 2;
        (self.rb(self.pc - 1) << 8) as u16 | (self.rb(self.pc - 2) as u16)
    }

    // fn op_ld(&mut self, t1: Target, t2: Target) {
    //     let value = match t2 {
    //         Target::Register(r) => match r {
    //             A => self.regs.a,
    //             B => self.regs.b,
    //             C => self.regs.c,
    //             D => self.regs.d,
    //             E => self.regs.e,
    //             F => self.regs.f,
    //             H => self.regs.h,
    //             L => self.regs.l,
    //         }
    //         Target::Value(n) => n,
    //         _ => panic!("invalid op_ld t2")
    //     };

    //     match t1 {
    //         Register(A) => self.regs.a = value,
    //         _ => panic!("invalid op_ld t1")
    //     }
    // }

    fn op_add(&mut self, b: u8, carry: bool) {
        let a = self.regs.a;
        let c = (carry && self.flags.c) as u8;
        let res = a.wrapping_add(b).wrapping_add(c);

        self.regs.a = res;
        self.flags.z = res == 0;
        self.flags.n = false;
        self.flags.h = (a & 0xF) + (b & 0xF) + c > 0xF;
        self.flags.c = (a as u16) + (b as u16) + (c as u16) > 0xFF;
    }

    fn op_sub(&mut self, b: u8, carry: bool) {
        let a = self.regs.a;
        let c = (carry && self.flags.c) as u8;
        let res = a.wrapping_add(b).wrapping_sub(c);

        self.regs.a = res;
        self.flags.z = res == 0;
        self.flags.n = true;
        self.flags.h = (a & 0xF) < (b & 0xF) + c;
        self.flags.c = (a as u16) < (b as u16) + (c as u16);
    }

    fn op_and(&mut self, b: u8) {
        let res = self.regs.a & b;

        self.regs.a = res;
        self.flags.z = res == 0;
        self.flags.n = false;
        self.flags.h = true;
        self.flags.c = false;
    }

    fn op_or(&mut self, b: u8) {
        let res = self.regs.a | b;

        self.regs.a = res;
        self.flags.z = res == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }

    fn op_xor(&mut self, b: u8) {
        let res = self.regs.a ^ b;

        self.regs.a = res;
        self.flags.z = res == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }

    // fn op_cmp(&mut self, rhs: u8) {
    //     let (res, cy) = self.a.overflowing_sub(rhs);

    //     self.cc.set_szp(res);
    //     self.cc.a = !(self.a ^ res ^ rhs) & 0b10000 == 1;
    //     self.cc.c = cy;
    // }

    // fn op_jump(&mut self, cond: bool) {
    //     let addr = self.nw();

    //     if cond {
    //         self.pc = addr;
    //     }
    // }

    // fn op_call_addr(&mut self, addr: u16) {
    //     self.push(self.pc);
    //     self.pc = addr;
    // }

    // fn op_call(&mut self, cond: bool) {
    //     let addr = self.nw();

    //     if cond {
    //         self.op_call_addr(addr);
    //     }
    // }

    // fn op_ret(&mut self, cond: bool) {
    //     if cond {
    //         self.pc = self.pop();
    //     }
    // }

    // fn pop(&mut self) -> u16 {
    //     self.sp += 2;
    //     return ((self.mem[(self.sp - 2) as usize] as u16) << 8)
    //         | self.mem[(self.sp - 1) as usize] as u16;
    // }

    // fn push(&mut self, val: u16) {
    //     self.sp -= 2;
    //     self.mem[self.sp as usize] = (val >> 8) as u8;
    //     self.mem[(self.sp + 1) as usize] = (val & 0xFF) as u8;
    // }

    pub fn cycle(&mut self) {
        let op = self.nb();

        match op {
            0x00 => {}

            0x01 => {
                let nn = self.nw();
                self.regs.set_bc(nn)
            }
            0x11 => {
                let nn = self.nw();
                self.regs.set_de(nn)
            }
            0x21 => {
                let nn = self.nw();
                self.regs.set_hl(nn)
            }
            0x31 => self.sp = self.nw(),

            // LD nn, n
            0x06 => self.regs.b = self.nb(),
            0x0E => self.regs.c = self.nb(),
            0x16 => self.regs.d = self.nb(),
            0x1E => self.regs.e = self.nb(),
            0x26 => self.regs.h = self.nb(),
            0x2E => self.regs.l = self.nb(),

            // LD A, n
            0x7F => self.regs.a = self.regs.a,
            0x78 => self.regs.a = self.regs.b,
            0x79 => self.regs.a = self.regs.c,
            0x7A => self.regs.a = self.regs.d,
            0x7B => self.regs.a = self.regs.e,
            0x7C => self.regs.a = self.regs.h,
            0x7D => self.regs.a = self.regs.l,
            0x0A => self.regs.a = self.rb(self.regs.bc()),
            0x1A => self.regs.a = self.rb(self.regs.de()),
            0x7E => self.regs.a = self.rb(self.regs.hl()),
            0xFA => {
                let nn = self.nw();
                self.regs.a = self.rb(nn)
            }

            // LD n, A
            0x02 => self.wb(self.regs.bc(), self.regs.a),
            0x12 => self.wb(self.regs.de(), self.regs.a),
            0x77 => self.wb(self.regs.hl(), self.regs.a),
            0xEA => {
                let nn = self.nw();
                self.wb(nn, self.regs.a);
            }

            // LD A, ($FF00 + C)
            0xF2 => self.regs.a = self.rb(0xFF00 + self.regs.c as u16),

            // LD ($FF00 + C), A
            0xE2 => self.wb(0xFF00 + self.regs.c as u16, self.regs.a),

            // LD r1, r2
            0x40 => self.regs.b = self.regs.b,
            0x41 => self.regs.b = self.regs.c,
            0x42 => self.regs.b = self.regs.d,
            0x43 => self.regs.b = self.regs.e,
            0x44 => self.regs.b = self.regs.h,
            0x45 => self.regs.b = self.regs.l,
            0x46 => self.regs.b = self.rb(self.regs.hl()),
            0x47 => self.regs.b = self.regs.a,

            0x48 => self.regs.c = self.regs.b,
            0x49 => self.regs.c = self.regs.c,
            0x4A => self.regs.c = self.regs.d,
            0x4B => self.regs.c = self.regs.e,
            0x4C => self.regs.c = self.regs.h,
            0x4D => self.regs.c = self.regs.l,
            0x4E => self.regs.c = self.rb(self.regs.hl()),
            0x4F => self.regs.c = self.regs.a,

            0x50 => self.regs.d = self.regs.b,
            0x51 => self.regs.d = self.regs.c,
            0x52 => self.regs.d = self.regs.d,
            0x53 => self.regs.d = self.regs.e,
            0x54 => self.regs.d = self.regs.h,
            0x55 => self.regs.d = self.regs.l,
            0x56 => self.regs.d = self.rb(self.regs.hl()),
            0x57 => self.regs.d = self.regs.a,

            0x58 => self.regs.e = self.regs.b,
            0x59 => self.regs.e = self.regs.c,
            0x5A => self.regs.e = self.regs.d,
            0x5B => self.regs.e = self.regs.e,
            0x5C => self.regs.e = self.regs.h,
            0x5D => self.regs.e = self.regs.l,
            0x5E => self.regs.e = self.rb(self.regs.hl()),
            0x5F => self.regs.e = self.regs.a,

            0x60 => self.regs.h = self.regs.b,
            0x61 => self.regs.h = self.regs.c,
            0x62 => self.regs.h = self.regs.d,
            0x63 => self.regs.h = self.regs.e,
            0x64 => self.regs.h = self.regs.h,
            0x65 => self.regs.h = self.regs.l,
            0x66 => self.regs.h = self.rb(self.regs.hl()),
            0x67 => self.regs.h = self.regs.a,

            0x68 => self.regs.l = self.regs.b,
            0x69 => self.regs.l = self.regs.c,
            0x6A => self.regs.l = self.regs.d,
            0x6B => self.regs.l = self.regs.e,
            0x6C => self.regs.l = self.regs.h,
            0x6D => self.regs.l = self.regs.l,
            0x6E => self.regs.l = self.rb(self.regs.hl()),
            0x6F => self.regs.l = self.regs.a,

            0x70 => self.regs.set_hl(self.regs.b as u16),
            0x71 => self.regs.set_hl(self.regs.c as u16),
            0x72 => self.regs.set_hl(self.regs.d as u16),
            0x73 => self.regs.set_hl(self.regs.e as u16),
            0x74 => self.regs.set_hl(self.regs.h as u16),
            0x75 => self.regs.set_hl(self.regs.l as u16),
            0x36 => {
                let n = self.nb() as u16;
                self.regs.set_hl(n)
            }

            _ => panic!("invalid op {:02X}", op),
        }
    }
}
