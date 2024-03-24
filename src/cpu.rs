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
            let operation_code = self.get_next_byte_and_update_program_counter(&program);
            match operation_code {
                0xA9 => {
                    let param = self.get_next_byte_and_update_program_counter(&program);
                    if self.program_counter as usize == program.len() {
                        panic!("wrong argument for load accumulator")
                    }
                    self.register_a = param;
                    
                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }
    
                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }

                }
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

    #[test]
    fn lda_param_is_loaded_to_register_a(){
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xA9, 0x11, 0x00]);
        assert_eq!(cpu.register_a,0x11);
        assert_eq!(cpu.program_counter, 3);
    }

    #[test]
    #[should_panic(expected = "wrong argument for load accumulator")]
    fn lda_param_argument_wrong(){
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xA9, 0x00]);
    }

    #[test]
    fn lda_param_is_0_zero_flag_is_set(){
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xA9,0x00, 0x00]);
        assert_eq!(cpu.register_a,0x00);
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.status, 0b0000_0010)
    }

    #[test]
    fn lda_param_is_0_zero_flag_is_set_without_affecting_other_flags(){
        let mut cpu = Cpu::new();
        cpu.status = 0b0110_1101;
        cpu.interpret(vec![0xA9,0x00, 0x00]);
        assert_eq!(cpu.register_a,0x00);
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.status, 0b0110_1111)
    }

    #[test]
    fn lda_param_is_negative_negative_flag_is_set(){
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xA9,0xA0, 0x00]);
        assert_eq!(cpu.register_a,0xA0);
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.status, 0b1000_0000)
    }

    #[test]
    fn lda_param_is_negative_negative_flag_is_set_without_affecting_other_flags(){
        let mut cpu = Cpu::new();
        cpu.status
        cpu.interpret(vec![0xA9,0xA0, 0x00]);
        assert_eq!(cpu.register_a,0xA0);
        assert_eq!(cpu.program_counter, 3);
        assert_eq!(cpu.status, 0b1000_0000)
    }

}
