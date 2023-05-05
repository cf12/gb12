mod cpu;
mod registers;
mod flags;
mod ops;

use crate::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.cycle();
}
