#![allow(unused_variables)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lower_byte = self.mem_read(pos) as u16;
        let higher_byte = self.mem_read(pos + 1) as u16;
        (higher_byte << 8) | (lower_byte as u16)
    }
 
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let higher_byte = (data >> 8) as u8;
        let lower_byte = (data & 0xff) as u8;
        self.mem_write(pos, lower_byte);
        self.mem_write(pos + 1, higher_byte);
    }
 

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        validate_program(&program);
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
 
        self.program_counter = self.mem_read_u16(0xFFFC);
    }
 
    pub fn run(&mut self) {

        loop {
            let operation_code = self.get_next_byte_and_update_program_counter();
            match operation_code {
                0xA9 => self.lda(),
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }
        }
    }

    fn get_next_byte_and_update_program_counter(&mut self,) -> u8 {
        let byte_code = self.mem_read(self.program_counter);
        self.program_counter += 1;
        byte_code
    }

    fn lda(&mut self) {
        let param = self.get_next_byte_and_update_program_counter();
        self.register_a = param;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x +=1;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

fn validate_program(program: &Vec<u8>) {
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
