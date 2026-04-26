use super::super::*;
use super::common::*;

#[test]
fn inx_value_incremented_by_1() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0x11, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0x11);
    assert_eq!(cpu.register_x, 0x12);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 5);
}

#[test]
fn inx_value_incremented_by_1_zero_flag_set() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0xFF);
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 5);
    assert_eq!(cpu.status, 0b0000_0010);
}

#[test]
fn inx_value_incremented_by_1_negative_flag_set() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0x7F, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0x7F);
    assert_eq!(cpu.register_x, 0x80);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 5);
    assert_eq!(cpu.status, 0b1000_0000);
}

#[test]
fn inx_value_incremented_over_flow() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);
    assert_eq!(cpu.register_a, 0xFF);
    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 6);
    assert_eq!(cpu.status, 0b0000_0000);
}