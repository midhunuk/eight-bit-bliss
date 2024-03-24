#![allow(unused_variables)]
pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        validate_program(&program);

        self.program_counter = 0;
        loop {
            let operation_code = self.get_next_byte_and_update_program_counter(&program);
            match operation_code {
                0xA9 => self.lda(&program),
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }
        }
    }

    fn get_next_byte_and_update_program_counter(&mut self, program: &Vec<u8>) -> u8 {
        let byte_code = program[self.program_counter as usize];
        self.program_counter += 1;
        byte_code
    }

    fn lda(&mut self, program: &Vec<u8>) {
        let param = self.get_next_byte_and_update_program_counter(program);
        if self.program_counter as usize == program.len() {
            panic!("wrong argument for load accumulator")
        }
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
