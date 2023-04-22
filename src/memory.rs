const MEMORY_SIZE: usize = 0xFFFF;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
}
