use crate::cpu::tests::common::*;
use crate::cpu::*;

#[test]
fn clc_carry_flag_is_0_and_carry_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x18, 0x00]);
    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn clc_carry_flag_is_1_and_carry_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x18, 0x00]);
    cpu.status.insert(CpuFlags::CARRY);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn cld_decimal_mode_flag_is_0_and_decimal_mode_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0xD8, 0x00]);
    cpu.status.remove(CpuFlags::DECIMAL_MODE);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::DECIMAL_MODE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn cld_decimal_mode_flag_is_1_and_decimal_mode_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0xD8, 0x00]);
    cpu.status.insert(CpuFlags::DECIMAL_MODE);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::DECIMAL_MODE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn cli_interrupt_disable_flag_is_0_and_interrupt_disable_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x58, 0x00]);
    cpu.status.remove(CpuFlags::INTERRUPT_DISABLE);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn cli_interrupt_disable_flag_is_1_and_interrupt_disable_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0x58, 0x00]);
    cpu.status.insert(CpuFlags::INTERRUPT_DISABLE);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn clv_overflow_flag_is_0_and_overflow_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0xB8, 0x00]);
    cpu.status.remove(CpuFlags::OVERFLOW);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}

#[test]
fn clv_overflow_flag_is_1_and_overflow_flag_is_cleared() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![0xB8, 0x00]);
    cpu.status.insert(CpuFlags::OVERFLOW);

    cpu.run();

    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
}