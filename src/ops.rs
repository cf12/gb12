use crate::registers::{Register, RegisterPair};

pub enum Target {
    Register(Register),
    RegisterPair(RegisterPair),
    Addr(u16),
    Value(u8),
}

