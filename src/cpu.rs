use crate::opcodes::*;
use bitflags::bitflags;

bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuFlags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | lo
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for Cpu {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: CpuFlags::empty(),
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn load_and_reset(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        validate_program(&program);
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = CpuFlags::empty();

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let opcode = OPCODES[code as usize]
                .as_ref()
                .unwrap_or_else(|| panic!("Unknown opcode: {:02X}", code));

            match code {
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(opcode),
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(opcode),
                0x0A => self.asl_accumalator(),
                0x06 | 0x16 | 0x0E | 0x1E => self.asl(opcode),
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(opcode),
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(opcode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(opcode),
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        let first_operand = self.program_counter;

        match mode {
            AddressingMode::Immediate => first_operand,

            AddressingMode::ZeroPage => self.mem_read(first_operand) as u16,

            AddressingMode::Absolute => self.mem_read_u16(first_operand),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(first_operand);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(first_operand);
                pos.wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(first_operand);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(first_operand);
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(first_operand);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(first_operand);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing | AddressingMode::Accumulator => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn adc(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let memory_value = self.mem_read(addr);

        let sum = self.register_a as u16
            + memory_value as u16
            + self.status.contains(CpuFlags::CARRY) as u16;

        let carry = sum > 0xff;

        if carry {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let result = sum as u8;

        if (memory_value ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status.insert(CpuFlags::OVERFLOW);
        } else {
            self.status.remove(CpuFlags::OVERFLOW)
        }

        self.register_a = result;

        self.update_zero_and_negative_flags(self.register_a);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn and(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_a &= value;

        self.update_zero_and_negative_flags(self.register_a);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn asl_accumalator(&mut self) {
        let value = self.register_a;
        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        let result = value << 1;
        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    fn asl(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        let result= value << 1;

        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn lda(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_a = value;

        self.update_zero_and_negative_flags(self.register_a);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn ldx(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_x = value;

        self.update_zero_and_negative_flags(self.register_x);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn ldy(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        self.register_y = value;

        self.update_zero_and_negative_flags(self.register_y);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x += 1;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.set_negative_flag();
        } else {
            self.clear_negative_flag();
        }
    }

    fn set_zero_flag(&mut self) {
        self.status.insert(CpuFlags::ZERO);
    }

    fn clear_zero_flag(&mut self) {
        self.status.remove(CpuFlags::ZERO);
    }

    fn set_carry_flag(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }

    fn clear_carry_flag(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }
    fn set_negative_flag(&mut self) {
        self.status.insert(CpuFlags::NEGATIVE);
    }

    fn clear_negative_flag(&mut self) {
        self.status.remove(CpuFlags::NEGATIVE);
    }
}

fn validate_program(program: &[u8]) {
    let program_length = program.len();
    if program_length == 0 {
        panic!("program is empty")
    }
    let last_byte = program[program_length - 1];
    if last_byte != 0x00 {
        panic!("program should end with 0x00")
    }
}

#[cfg(test)]
mod tests;
