use crate::cpu::*;

pub const PROGRAM_START_VALUE: u16 = 0x8000;

pub fn set_up_cpu() -> Cpu{
    Cpu::new()
}