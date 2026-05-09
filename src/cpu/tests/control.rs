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

mod jmp{
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn jmp_absolute() {
        let mut cpu = set_up_cpu();
        // JMP $1234. Memory at $1234 is 0 (BRK) by default.
        cpu.load_and_run(vec![0x4C, 0x34, 0x12, 0x00]);
        assert_eq!(cpu.program_counter, 0x1234 + 1);
    }

    #[test]
    fn jmp_indirect() {
        let mut cpu = set_up_cpu();
        cpu.mem_write_u16(0x0010, 0x1234);
        // JMP ($0010). Memory at $0010 points to $1234.
        cpu.load_and_run(vec![0x6C, 0x10, 0x00, 0x00]);
        assert_eq!(cpu.program_counter, 0x1234 + 1);
    }
}

mod jsr{
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn jsr_absolute() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0x20, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.program_counter, 0x1234 + 1);
        assert_eq!(cpu.stack_pointer, 0xFD - 2);

        assert_eq!(cpu.mem_read(0x01FD), 0x80); // High byte of return address
        assert_eq!(cpu.mem_read(0x01FC), 0x02); // Low byte of return address
    }
}

mod nop{
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn nop_does_nothing() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xEA, 0x00]);

        cpu.register_a = 0x11;
        cpu.register_x = 0x22;
        cpu.register_y = 0x33;
        cpu.status = CpuFlags::from_bits_truncate(0b1010_1010);

        cpu.run();

        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.register_x, 0x22);
        assert_eq!(cpu.register_y, 0x33);
        assert_eq!(cpu.status.bits(), 0b1010_1010);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2);
    }
}

mod rts {
    use crate::cpu::tests::common::*;

    #[test]
    fn rts_returns_from_subroutine() {
        let mut cpu = set_up_cpu();
        // Program: JSR $8006, LDA #$11, BRK, RTS, BRK
        // $8000: 20 06 80 -> JSR $8006
        // $8003: A9 11    -> LDA #$11
        // $8005: 00       -> BRK
        // $8006: 60       -> RTS
        cpu.load_and_run(vec![
            0x20, 0x06, 0x80, 0xA9, 0x11, 0x00, 0x60, 0x00
        ]);

        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 6);
    }
}

mod rti {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn rti_restores_status_and_pc() {
        let mut cpu = set_up_cpu();
        let return_pc: u16 = 0x1234;
        cpu.mem_write(return_pc, 0x00); // Write BRK at target

        cpu.load_and_reset(vec![0x40, 0x00]); // RTI
        cpu.stack_push_u16(return_pc);
        // Push status with Negative, Overflow, and Carry set
        cpu.stack_push(0b1100_0001); 
        cpu.run();

        assert_eq!(cpu.program_counter, return_pc + 1);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(cpu.status.contains(CpuFlags::OVERFLOW));
        assert!(cpu.status.contains(CpuFlags::CARRY));
    }
}