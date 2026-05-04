use crate::cpu::tests::common::*;
use crate::cpu::*;

#[test]
fn bcc_condition_not_met_program_counter_not_moved() {
    let mut cpu = set_up_cpu();
    cpu.load_and_reset(vec![0x90, 0x20, 0x00]);
    cpu.status.insert(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 + 1);
}

#[test]
fn bcc_condition_met_program_counter_moved_forward() {
    let mut cpu = set_up_cpu();
    cpu.load_and_reset(vec![0x90, 0x20, 0x00]);
    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 + 0x20 + 1);
}

#[test]
fn bcc_condition_met_program_counter_moved_backward() {
    let mut cpu = set_up_cpu();
    cpu.load_and_reset(vec![0x90, 0xE0, 0x00]);
    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 - 0x20 + 1);
}

#[test]
fn bcc_skips_next_instruction_when_taken() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![
        0x90, 0x02, // BCC +2 (skip LDA)
        0xA9, 0xFF, // LDA #$FF (should be skipped)
        0xA9, 0x42, // LDA #$42
        0x00,
    ]);

    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.register_a, 0x42);
}

#[test]
fn bcc_does_not_branch_when_carry_set() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![
        0x90, 0x02,
        0xA9, 0x11,
        0x00,
    ]);

    cpu.status.insert(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.register_a, 0x11);
}

#[test]
fn bcc_zero_offset_no_jump() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![
        0x90, 0x00,
        0xA9, 0x99,
        0x00,
    ]);

    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.register_a, 0x99);
}
#[test]
fn bcc_max_forward_branch() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![
        0x90, 0x7F, 0x00,
    ]);

    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 + 0x7F + 1);
}

#[test]
fn bcc_max_backward_branch() {
    let mut cpu = set_up_cpu();

    cpu.load_and_reset(vec![
        0x90, 0x80, 0x00,
    ]);

    cpu.status.remove(CpuFlags::CARRY);

    cpu.run();

    assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 - 0x80 + 1);
}