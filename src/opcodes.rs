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
   Accumulator,
   Relative
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
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref OPCODES: [Option<OpCode>; 256] = {
        let mut table: [Option<OpCode>; 256] = [(); 256].map(|_| None);

        table[0x00] = Some(OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing));

        //arithmetic
        table[0x69] = Some(OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate));
        table[0x65] = Some(OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage));
        table[0x75] = Some(OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X));
        table[0x6D] = Some(OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute));
        table[0x7D] = Some(OpCode::new(0x7D, "ADC", 3, 4 + 1, AddressingMode::Absolute_X));
        table[0x79] = Some(OpCode::new(0x79, "ADC", 3, 4 + 1, AddressingMode::Absolute_Y));
        table[0x61] = Some(OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X));
        table[0x71] = Some(OpCode::new(0x71, "ADC", 2, 5 + 1, AddressingMode::Indirect_Y));

        table[0x29] = Some(OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate));
        table[0x25] = Some(OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage));
        table[0x35] = Some(OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X));
        table[0x2D] = Some(OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute));
        table[0x3D] = Some(OpCode::new(0x3D, "AND", 3, 4 + 1, AddressingMode::Absolute_X));
        table[0x39] = Some(OpCode::new(0x39, "AND", 3, 4 + 1, AddressingMode::Absolute_Y));
        table[0x21] = Some(OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X));
        table[0x31] = Some(OpCode::new(0x31, "AND", 2, 5 + 1, AddressingMode::Indirect_Y));

        table[0x0A] = Some(OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::Accumulator));
        table[0x06] = Some(OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage));
        table[0x16] = Some(OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X));
        table[0x0E] = Some(OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute));
        table[0x1E] = Some(OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::Absolute_X));

        table[0x24] = Some(OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage));
        table[0x2C] = Some(OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute));

        table[0xC6] = Some(OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage));
        table[0xD6] = Some(OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X));
        table[0xCE] = Some(OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute));
        table[0xDE] = Some(OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X));

        table[0xCA] = Some(OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::NoneAddressing));

        table[0x88] = Some(OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing));

        table[0xE6] = Some(OpCode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage));
        table[0xF6] = Some(OpCode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPage_X));
        table[0xEE] = Some(OpCode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute));
        table[0xFE] = Some(OpCode::new(0xFE, "INC", 3, 7, AddressingMode::Absolute_X));

        table[0xE8] = Some(OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing));

        table[0xC8] = Some(OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing));

        //branch
        table[0x90] = Some(OpCode::new(0x90, "BCC", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0xB0] = Some(OpCode::new(0xB0, "BCS", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0xF0] = Some(OpCode::new(0xF0, "BEQ", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0x30] = Some(OpCode::new(0x30, "BMI", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0xD0] = Some(OpCode::new(0xD0, "BNE", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0x10] = Some(OpCode::new(0x10, "BPL", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0x50] = Some(OpCode::new(0x50, "BVC", 2, 2 + 1 + 2, AddressingMode::Relative));
        table[0x70] = Some(OpCode::new(0x70, "BVS", 2, 2 + 1 + 2, AddressingMode::Relative));

        //clear register
        table[0x18] = Some(OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing));
        table[0xD8] = Some(OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing));
        table[0x58] = Some(OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing));
        table[0xB8] = Some(OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::NoneAddressing));

        //compare
        table[0xC9] = Some(OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate));
        table[0xC5] = Some(OpCode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage));
        table[0xD5] = Some(OpCode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPage_X));
        table[0xCD] = Some(OpCode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute));
        table[0xDD] = Some(OpCode::new(0xDD, "CMP", 3, 4 + 1, AddressingMode::Absolute_X));
        table[0xD9] = Some(OpCode::new(0xD9, "CMP", 3, 4 + 1, AddressingMode::Absolute_Y));
        table[0xC1] = Some(OpCode::new(0xC1, "CMP", 2, 6, AddressingMode::Indirect_X));
        table[0xD1] = Some(OpCode::new(0xD1, "CMP", 2, 5 + 1, AddressingMode::Indirect_Y));

        table[0xE0] = Some(OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate));
        table[0xE4] = Some(OpCode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage));
        table[0xEC] = Some(OpCode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute));

        table[0xC0] = Some(OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate));
        table[0xC4] = Some(OpCode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage));
        table[0xCC] = Some(OpCode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute));

        //load
        table[0xA9] = Some(OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate));
        table[0xA5] = Some(OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage));
        table[0xB5] = Some(OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X));
        table[0xAD] = Some(OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute));
        table[0xBD] = Some(OpCode::new(0xBD, "LDA", 3, 4 + 1, AddressingMode::Absolute_X));
        table[0xB9] = Some(OpCode::new(0xB9, "LDA", 3, 4 + 1, AddressingMode::Absolute_Y));
        table[0xA1] = Some(OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X));
        table[0xB1] = Some(OpCode::new(0xB1, "LDA", 2, 5 + 1, AddressingMode::Indirect_Y));
        
        table[0xA2] = Some(OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate));
        table[0xA6] = Some(OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage));
        table[0xB6] = Some(OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y));
        table[0xAE] = Some(OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute));
        table[0xBE] = Some(OpCode::new(0xBE, "LDX", 3, 4 + 1, AddressingMode::Absolute_Y));

        table[0xA0] = Some(OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate));
        table[0xA4] = Some(OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage));
        table[0xB4] = Some(OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X));
        table[0xAC] = Some(OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute));
        table[0xBC] = Some(OpCode::new(0xBC, "LDY", 3, 4 + 1, AddressingMode::Absolute_X));

        //transfer
        table[0xAA] = Some(OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing));

        table
    };
}