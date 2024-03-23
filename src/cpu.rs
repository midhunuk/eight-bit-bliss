#![allow(unused_variables)]
pub struct Cpu {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        validate_program(&program);

        self.program_counter = 0;
        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;
            match opscode {
                0x00 => return,
                _ => todo!(),
            }
        }
    }
}

fn validate_program(program: &Vec<u8>) {
    let program_length = program.len();
    if program_length == 0 {
        panic!("program is empty")
    }
    let last_byte = program[program_length-1];
    if last_byte != 0x00 {
        panic!("program should end with 0x00")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "program is empty")]
    fn empty_program() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![]);
    }

    #[test]
    #[should_panic(expected = "program should end with 0x00")]
    fn program_without_0x00_ending() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0x01, 0x03]);
    }
    #[test]
    fn program_0x00_program_counter_updates_to_1_and_exits() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0x00]);
        assert_eq!(cpu.program_counter, 1);
    }
}
