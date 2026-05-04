mod branch_common {

    macro_rules! branch_test_suite {
        (
        $opcode:expr,
        $set_condition:expr,
        $clear_condition:expr
    ) => {
            use crate::cpu::tests::common::*;
            #[test]
            fn not_taken_pc() {
                let mut cpu = set_up_cpu();
                cpu.load_and_reset(vec![$opcode, 0x20, 0x00]);

                $set_condition(&mut cpu); // condition NOT satisfied

                cpu.run();

                assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
            }

            #[test]
            fn taken_forward() {
                let mut cpu = set_up_cpu();
                cpu.load_and_reset(vec![$opcode, 0x20, 0x00]);

                $clear_condition(&mut cpu); // condition satisfied

                cpu.run();

                assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 + 0x20 + 1);
            }

            #[test]
            fn taken_backward() {
                let mut cpu = set_up_cpu();
                cpu.load_and_reset(vec![$opcode, 0xE0, 0x00]);

                $clear_condition(&mut cpu);

                cpu.run();

                assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 - 0x20 + 1);
            }

            #[test]
            fn skip_instruction() {
                let mut cpu = set_up_cpu();

                cpu.load_and_reset(vec![$opcode, 0x02, 0xA9, 0xFF, 0xA9, 0x42, 0x00]);

                $clear_condition(&mut cpu);

                cpu.run();

                assert_eq!(cpu.register_a, 0x42);
            }

            #[test]
            fn not_taken_executes_next() {
                let mut cpu = set_up_cpu();

                cpu.load_and_reset(vec![$opcode, 0x02, 0xA9, 0x11, 0x00]);

                $set_condition(&mut cpu);

                cpu.run();

                assert_eq!(cpu.register_a, 0x11);
            }

            #[test]
            fn zero_offset() {
                let mut cpu = set_up_cpu();

                cpu.load_and_reset(vec![$opcode, 0x00, 0xA9, 0x99, 0x00]);

                $clear_condition(&mut cpu);

                cpu.run();

                assert_eq!(cpu.register_a, 0x99);
            }

            #[test]
            fn max_forward() {
                let mut cpu = set_up_cpu();

                cpu.load_and_reset(vec![$opcode, 0x7F, 0x00]);

                $clear_condition(&mut cpu);

                cpu.run();

                assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 + 0x7F + 1);
            }

            #[test]
            fn max_backward() {
                let mut cpu = set_up_cpu();

                cpu.load_and_reset(vec![$opcode, 0x80, 0x00]);

                $clear_condition(&mut cpu);

                cpu.run();

                assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 2 - 0x80 + 1);
            }
        };
    }
    pub(crate) use branch_test_suite;
}

mod bcc {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0x90,
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::CARRY),
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::CARRY)
    );
}

mod bcs {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0xB0,
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::CARRY),
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::CARRY)
    );
}

mod beq {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0xF0,
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::ZERO),
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::ZERO)
    );
}

mod bne {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0xD0,
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::ZERO),
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::ZERO)
    );
}

mod bmi {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0x30,
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::NEGATIVE),
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::NEGATIVE)
    );
}

mod bpl {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0x10,
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::NEGATIVE),
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::NEGATIVE)
    );
}

mod bvc {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0x50,
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::OVERFLOW),
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::OVERFLOW)
    );
}

mod bvs {
    use super::branch_common::*;
    use crate::cpu::*;
    branch_test_suite!(
        0x70,
        |cpu: &mut Cpu| cpu.status.remove(CpuFlags::OVERFLOW),
        |cpu: &mut Cpu| cpu.status.insert(CpuFlags::OVERFLOW)
    );
}