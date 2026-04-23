use lazy_static::lazy_static;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing, //Implied
}

pub struct OpCode {
    pub code:u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles:u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code: code,
            mnemonic: mnemonic,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }
}

lazy_static! {
    pub static ref OPCODES: [Option<OpCode>; 256] = {
        let mut table: [Option<OpCode>; 256] = [(); 256].map(|_| None);

        table[0x00] = Some(OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing));

        table[0xe8] = Some(OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing));

        table[0xA9] = Some(OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate));
        
        table[0xAA] = Some(OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing));

        table
    };
}