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
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF],
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

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
            stack_pointer: STACK_RESET,
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
        self.stack_pointer = STACK_RESET;

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
                0x90 => self.bcc(),
                0xB0 => self.bcs(),
                0xF0 => self.beq(),
                0x24 | 0x2C => self.bit(opcode),
                0x30 => self.bmi(),
                0xD0 => self.bne(),
                0x10 => self.bpl(),
                0x50 => self.bvc(),
                0x70 => self.bvs(),
                0x18 => self.clc(),
                0xD8 => self.cld(),
                0x58 => self.cli(),
                0xB8 => self.clv(),
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(opcode),
                0xE0 | 0xE4 | 0xEC => self.cpx(opcode),
                0xC0 | 0xC4 | 0xCC => self.cpy(opcode),
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(opcode),
                0xCA => self.dex(),
                0x88 => self.dey(),
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(opcode),
                0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(opcode),
                0xE8 => self.inx(),
                0xC8 => self.iny(),
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(opcode),
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(opcode),
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(opcode),
                0x4C | 0x6C => self.jmp(opcode),
                0x20 => self.jsr(),
                0x4A => self.lsr_accumalator(),
                0x46 | 0x56 | 0x4E | 0x5E => self.lsr(opcode),
                0xEA => {} //NOP do nothing
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(opcode),
                0x48 => self.pha(),
                0x08 => self.php(),
                0x68 => self.pla(),
                0x28 => self.plp(),
                0x2A => self.rol_accumalator(),
                0x26 | 0x36 | 0x2E | 0x3E => self.rol(opcode),
                0x6A => self.ror_accumalator(),
                0x66 | 0x76 | 0x6E | 0x7E => self.ror(opcode),
                0x40 => self.rti(),
                0x60 => self.rts(),
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(opcode),
                0x38 => self.sec(),
                0xF8 => self.sed(),
                0x78 => self.sei(),
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(opcode),
                0x86 | 0x96 | 0x8E => self.stx(opcode),
                0x84 | 0x94 | 0x8C => self.sty(opcode),
                0xAA => self.tax(),
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
            AddressingMode::Indirect => {
                let ptr = self.mem_read_u16(first_operand);
                let lo = self.mem_read(ptr);
                let hi = self.mem_read(ptr.wrapping_add(1));
                (hi as u16) << 8 | (lo as u16)
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

            _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        (hi << 8) | lo
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = STACK + self.stack_pointer as u16;
        self.mem_read(addr)
    }

    fn stack_push(&mut self, data: u8) {
        let addr = STACK + self.stack_pointer as u16;
        self.mem_write(addr, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
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
        let result = value << 1;

        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn bcc(&mut self) {
        let condition = !self.status.contains(CpuFlags::CARRY);
        self.branch(condition);
    }

    fn bcs(&mut self) {
        let condition = self.status.contains(CpuFlags::CARRY);
        self.branch(condition);
    }

    fn beq(&mut self) {
        let condition = self.status.contains(CpuFlags::ZERO);
        self.branch(condition);
    }

    fn bit(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = self.register_a & value;
        self.update_zero_flag(result);

        self.status.set(CpuFlags::NEGATIVE, value & 0b1000_0000 > 0);
        self.status.set(CpuFlags::OVERFLOW, value & 0b0100_0000 > 0);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn bmi(&mut self) {
        let condition = self.status.contains(CpuFlags::NEGATIVE);
        self.branch(condition);
    }

    fn bne(&mut self) {
        let condition = !self.status.contains(CpuFlags::ZERO);
        self.branch(condition);
    }

    fn bpl(&mut self) {
        let condition = !self.status.contains(CpuFlags::NEGATIVE);
        self.branch(condition);
    }

    fn bvc(&mut self) {
        let condition = !self.status.contains(CpuFlags::OVERFLOW);
        self.branch(condition);
    }

    fn bvs(&mut self) {
        let condition = self.status.contains(CpuFlags::OVERFLOW);
        self.branch(condition);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump = self.mem_read(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(1);

            let base = self.program_counter as i16;
            let jump_addr = base.wrapping_add(jump as i16) as u16;
            self.program_counter = jump_addr;
        } else {
            self.program_counter = self.program_counter.wrapping_add(1);
        }
    }

    fn clc(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }

    fn cld(&mut self) {
        self.status.remove(CpuFlags::DECIMAL_MODE);
    }

    fn cli(&mut self) {
        self.status.remove(CpuFlags::INTERRUPT_DISABLE);
    }

    fn clv(&mut self) {
        self.status.remove(CpuFlags::OVERFLOW);
    }

    fn cmp(&mut self, opcode: &OpCode) {
        self.compare(opcode, self.register_a);
    }

    fn cpx(&mut self, opcode: &OpCode) {
        self.compare(opcode, self.register_x);
    }

    fn cpy(&mut self, opcode: &OpCode) {
        self.compare(opcode, self.register_y);
    }

    fn compare(&mut self, opcode: &OpCode, register: u8) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = register.wrapping_sub(value);
        self.update_negative_flag(result);

        if register >= value {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        if register == value {
            self.set_zero_flag();
        } else {
            self.clear_zero_flag();
        }

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn dec(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);

        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = self.register_a ^ value;
        self.update_zero_and_negative_flags(result);
        self.register_a = result;

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn inc(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_add(1);
        self.mem_write(addr, result);

        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
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

    fn jmp(&mut self, opcode: &OpCode) {
        let addr = self.get_operand_address(&opcode.mode);
        self.program_counter = addr;
    }

    fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_address = self.get_operand_address(&AddressingMode::Absolute);
        self.program_counter = target_address;
    }

    fn lsr_accumalator(&mut self) {
        let value = self.register_a;
        if value & 0b0000_0001 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let result = value >> 1;
        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    fn lsr(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(address);

        if value & 0b0000_0001 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let result = value >> 1;
        self.mem_write(address, result);
        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn ora(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(address);

        let result = self.register_a | value;
        self.register_a = result;
        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn php(&mut self) {
        self.stack_push(self.status.bits());
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        let status = self.stack_pop();
        self.status = CpuFlags::from_bits_truncate(status);
    }

    fn rol_accumalator(&mut self) {
        let value = self.register_a;
        let old_carry = self.status.contains(CpuFlags::CARRY);

        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        let mut result = value << 1;
        if old_carry {
            result |= 0b0000_0001;
        }
        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    fn rol(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(address);

        let old_carry = self.status.contains(CpuFlags::CARRY);

        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        let mut result = value << 1;
        if old_carry {
            result |= 0b0000_0001;
        }
        self.mem_write(address, result);

        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn ror_accumalator(&mut self) {
        let value = self.register_a;
        let old_carry = self.status.contains(CpuFlags::CARRY);

        if value & 0x01 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        let mut result = value >> 1;
        if old_carry {
            result |= 0b1000_0000;
        }
        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    fn ror(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(address);
        let old_carry = self.status.contains(CpuFlags::CARRY);

        if value & 0x01 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }
        let mut result = value >> 1;
        if old_carry {
            result |= 0b1000_0000;
        }
        self.mem_write(address, result);
        self.update_zero_and_negative_flags(result);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn rti(&mut self) {
        let status = self.stack_pop();
        self.status = CpuFlags::from_bits_truncate(status);
        self.program_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    fn sbc(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        let value = self.mem_read(address);
        let carry = self.status.contains(CpuFlags::CARRY) as u16;

        let result = self.register_a as u16 - value as u16 - (1 - carry);

        if result > 0xFF {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        let result = result as u8;

        if (self.register_a ^ value) & (self.register_a ^ result) & 0x80 != 0 {
            self.status.insert(CpuFlags::OVERFLOW);
        } else {
            self.status.remove(CpuFlags::OVERFLOW);
        }

        self.register_a = result;

        self.update_zero_and_negative_flags(self.register_a);

        self.program_counter += (opcode.len - 1) as u16;
    }

    fn sec(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }

    fn sed(&mut self) {
        self.status.insert(CpuFlags::DECIMAL_MODE);
    }

    fn sei(&mut self) {
        self.status.insert(CpuFlags::INTERRUPT_DISABLE);
    }

    fn sta(&mut self, opcode: &OpCode){
        let address = self.get_operand_address(&opcode.mode);
        self.mem_write(address, self.register_a);
        self.program_counter += (opcode.len - 1) as u16;
    }

    fn stx(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        self.mem_write(address, self.register_x);
        self.program_counter += (opcode.len - 1) as u16;
    }

    fn sty(&mut self, opcode: &OpCode) {
        let address = self.get_operand_address(&opcode.mode);
        self.mem_write(address, self.register_y);
        self.program_counter += (opcode.len - 1) as u16;
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
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
