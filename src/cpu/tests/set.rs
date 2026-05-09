use crate::cpu::tests::common::*;
use crate::cpu::*;

#[test]
fn sec_carry_flag_is_0_and_carry_flag_is_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x38, 0x00]);
    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn sec_carry_flag_is_1_and_carry_flag_remains_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x38, 0x00]);
    cpu.status.insert(CpuFlags::CARRY);

    cpu.run();

    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn sed_decimal_mode_flag_is_0_and_decimal_mode_flag_is_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0xF8, 0x00]);
    cpu.status.remove(CpuFlags::DECIMAL_MODE);

    cpu.run();

    assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn sed_decimal_mode_flag_is_1_and_decimal_mode_flag_is_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0xF8, 0x00]);
    cpu.status.insert(CpuFlags::DECIMAL_MODE);

    cpu.run();

    assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn sei_interrupt_disable_flag_is_0_and_interrupt_disable_flag_is_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x78, 0x00]);
    cpu.status.remove(CpuFlags::INTERRUPT_DISABLE);

    cpu.run();

    assert!(cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn sei_interrupt_disable_flag_is_1_and_interrupt_disable_flag_is_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x78, 0x00]);
    cpu.status.insert(CpuFlags::INTERRUPT_DISABLE);

    cpu.run();

    assert!(cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}