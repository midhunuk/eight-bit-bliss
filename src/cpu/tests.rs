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
fn lda_param_is_loaded_to_register_a() {
    let mut cpu = Cpu::new();
    cpu.interpret(vec![0xA9, 0x11, 0x00]);
    assert_eq!(cpu.register_a, 0x11);
    assert_eq!(cpu.program_counter, 3);
}

#[test]
#[should_panic(expected = "wrong argument for load accumulator")]
fn lda_param_argument_wrong() {
    let mut cpu = Cpu::new();
    cpu.interpret(vec![0xA9, 0x00]);
}

#[test]
fn lda_param_is_0_zero_flag_is_set() {
    let mut cpu = Cpu::new();
    cpu.interpret(vec![0xA9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.program_counter, 3);
    assert_eq!(cpu.status, 0b0000_0010);
}

#[test]
fn lda_param_is_0_zero_flag_is_set_without_affecting_other_flags() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0110_1101;
    cpu.interpret(vec![0xA9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.program_counter, 3);
    assert_eq!(cpu.status, 0b0110_1111);
}

#[test]
fn lda_param_is_negative_negative_flag_is_set() {
    let mut cpu = Cpu::new();
    cpu.interpret(vec![0xA9, 0xA0, 0x00]);
    assert_eq!(cpu.register_a, 0xA0);
    assert_eq!(cpu.program_counter, 3);
    assert_eq!(cpu.status, 0b1000_0000);
}

#[test]
fn lda_param_is_negative_negative_flag_is_set_without_affecting_other_flags() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0110_1111;
    cpu.interpret(vec![0xA9, 0xA0, 0x00]);
    assert_eq!(cpu.register_a, 0xA0);
    assert_eq!(cpu.program_counter, 3);
    assert_eq!(cpu.status, 0b1110_1101);
}

#[test]
fn tax_moves_accumulator_to_register_x() {
    let mut cpu = Cpu::new();
    cpu.interpret(vec![0xA9, 0x11, 0xAA, 0x00]);
    assert_eq!(cpu.register_a, 0x11);
    assert_eq!(cpu.register_x, 0x11);
    assert_eq!(cpu.program_counter, 4);
}

#[test]
fn tax_accumalator_value_is_0_zero_flag_is_set_without_affecting_other_flags() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0110_1101;
    cpu.interpret(vec![0xA9, 0x00, 0xAA, 0x00]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.program_counter, 4);
    assert_eq!(cpu.status, 0b0110_1111);
}

#[test]
fn tax_accumalator_value_is_negative_negative_flag_is_set_without_affecting_other_flags() {
    let mut cpu = Cpu::new();
    cpu.status = 0b0110_1111;
    cpu.interpret(vec![0xA9, 0xA0, 0xAA, 0x00]);
    assert_eq!(cpu.register_a, 0xA0);
    assert_eq!(cpu.register_x, 0xA0);
    assert_eq!(cpu.program_counter, 4);
    assert_eq!(cpu.status, 0b1110_1101);
}

#[test]
fn inx_value_incremented_by_1(){
    let mut cpu = Cpu::new();
    cpu.interpret(vec![0xA9, 0x11, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0x11);
    assert_eq!(cpu.register_x, 0x12);
    assert_eq!(cpu.program_counter, 5); 
}

#[test]
fn inx_value_incremented_by_1_zero_flag_set(){
    let mut cpu = Cpu::new();
    cpu.status = 0b0110_1101;
    cpu.interpret(vec![0xA9, 0xFF, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0xFF);
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.program_counter, 5); 
    assert_eq!(cpu.status, 0b0110_1111);
}

#[test]
fn inx_value_incremented_by_1_negative_flag_set(){
    let mut cpu = Cpu::new();
    cpu.status = 0b0110_1111;
    cpu.interpret(vec![0xA9, 0x7F, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0x7F);
    assert_eq!(cpu.register_x, 0x80);
    assert_eq!(cpu.program_counter, 5); 
    assert_eq!(cpu.status, 0b1110_1101);
}