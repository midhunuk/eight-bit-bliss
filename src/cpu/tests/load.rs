use super::super::*;
use super::common::*;

#[test]
fn lda_param_is_loaded_to_register_a() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0x11, 0x00]);
    assert_eq!(cpu.register_a, 0x11);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
}

#[test]
fn lda_param_is_0_zero_flag_is_set() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    assert_eq!(cpu.status, 0b0000_0010);
}


#[test]
fn lda_param_is_negative_negative_flag_is_set() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0xA0, 0x00]);
    assert_eq!(cpu.register_a, 0xA0);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    assert_eq!(cpu.status, 0b1000_0000);
}