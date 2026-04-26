use super::super::*;
use super::common::*;

#[test]
#[should_panic(expected = "program is empty")]
fn empty_program() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![]);
}

#[test]
#[should_panic(expected = "program should end with 0x00")]
fn program_without_0x00_ending() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0x01, 0x03]);
}

#[test]
fn program_0x00_program_counter_updates_to_1_and_exits() {
    let mut cpu = Cpu::new();
    cpu.load_and_run(vec![0x00]);
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 1);
}

#[test]
fn write_and_read_16_bit_memory(){
    let mut cpu = Cpu::new();
    cpu.mem_write_u16(0x2000, 0xFF00);
    let value = cpu.mem_read_u16(0x2000);
    assert_eq!(value, 0xFF00);
}