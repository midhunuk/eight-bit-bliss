mod cmp {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn cmp_immediate_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xC9, 0x05, 0x00]);
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cmp_immediate_a_less_than_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xC9, 0x10, 0x00]);
        cpu.register_a = 0x05;

        cpu.run();

        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cmp_immediate_a_equal_to_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xC9, 0x10, 0x00]);
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cmp_zeropage_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_reset(vec![0xC5, 0x10, 0x00]);
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cmp_zeropage_x_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xD5, 0x10, 0x00]);
        cpu.register_x = 0x02;
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cmp_absolute_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x05);
        cpu.load_and_reset(vec![0xCD, 0x34, 0x12, 0x00]);
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn cmp_absolute_x_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xDD, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x02;
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn cmp_absolute_y_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xD9, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x02;
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn cmp_indirect_x_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write_u16(0x0010 + 0x02, 0x1234);
        cpu.mem_write(0x1234, 0x05);
        cpu.load_and_reset(vec![0xC1, 0x10, 0x00]);
        cpu.register_x = 0x02;
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cmp_indirect_y_a_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write_u16(0x0010, 0x1234);
        cpu.mem_write(0x1234 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xD1, 0x10, 0x00]);
        cpu.register_y = 0x02;
        cpu.register_a = 0x10;

        cpu.run();

        assert_eq!(cpu.register_a, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }
}

mod cpx {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn cpx_immediate_x_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xE0, 0x05, 0x00]);
        cpu.register_x = 0x10;

        cpu.run();

        assert_eq!(cpu.register_x, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpx_immediate_x_less_than_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xE0, 0x10, 0x00]);
        cpu.register_x = 0x05;

        cpu.run();

        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpx_immediate_x_equal_to_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xE0, 0x10, 0x00]);
        cpu.register_x = 0x10;

        cpu.run();

        assert_eq!(cpu.register_x, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpx_zeropage_x_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_reset(vec![0xE4, 0x10, 0x00]);
        cpu.register_x = 0x10;

        cpu.run();

        assert_eq!(cpu.register_x, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpx_absolute_x_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x05);
        cpu.load_and_reset(vec![0xEC, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x10;

        cpu.run();

        assert_eq!(cpu.register_x, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

}

mod cpy {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn cpy_immediate_y_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xC0, 0x05, 0x00]);
        cpu.register_y = 0x10;

        cpu.run();

        assert_eq!(cpu.register_y, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpy_immediate_y_less_than_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xC0, 0x10, 0x00]);
        cpu.register_y = 0x05;

        cpu.run();

        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpy_immediate_y_equal_to_m() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xC0, 0x10, 0x00]);
        cpu.register_y = 0x10;

        cpu.run();

        assert_eq!(cpu.register_y, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpy_zeropage_y_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_reset(vec![0xC4, 0x10, 0x00]);
        cpu.register_y = 0x10;

        cpu.run();

        assert_eq!(cpu.register_y, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn cpy_absolute_y_greater_than_m() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x05);
        cpu.load_and_reset(vec![0xCC, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x10;

        cpu.run();

        assert_eq!(cpu.register_y, 0x10);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }
}
