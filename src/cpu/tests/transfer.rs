use super::super::*;
use super::common::*;

mod tax {
    use super::*;

    #[test]
    fn tax_moves_accumulator_to_register_x() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA9, 0x11, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.register_x, 0x11);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn tax_accumulator_value_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA9, 0x00, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn tax_accumulator_value_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA9, 0xA0, 0xAA, 0x00]);

        cpu.run();

        assert_eq!(cpu.register_a, 0xA0);
        assert_eq!(cpu.register_x, 0xA0);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
    }
}

mod tay {
    use super::*;

    #[test]
    fn tay_moves_accumulator_to_register_y() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA9, 0x11, 0xA8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.register_y, 0x11);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn tay_accumulator_value_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA9, 0x00, 0xA8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn tay_accumulator_value_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA9, 0xA0, 0xA8, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_a, 0xA0);
        assert_eq!(cpu.register_y, 0xA0);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
    }
}

mod tsx {
    use super::*;

    #[test]
    fn tsx_moves_stack_pointer_to_register_x() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xBA, 0x00]);
        cpu.stack_pointer = 0x55;
        cpu.run();
        assert_eq!(cpu.register_x, 0x55);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn tsx_stack_pointer_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xBA, 0x00]);
        cpu.stack_pointer = 0x00;
        cpu.run();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn tsx_stack_pointer_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xBA, 0x00]);
        cpu.stack_pointer = 0x80;
        cpu.run();
        assert_eq!(cpu.register_x, 0x80);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
    }
}

mod txa {
    use super::*;

    #[test]
    fn txa_moves_register_x_to_accumulator() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA2, 0x11, 0x8A, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_x, 0x11);
        assert_eq!(cpu.register_a, 0x11);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod txs {
    use super::*;

    #[test]
    fn txs_moves_register_x_to_stack_pointer() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA2, 0x55, 0x9A, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_x, 0x55);
        assert_eq!(cpu.stack_pointer, 0x55);
        // TXS does not affect flags
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod tya {
    use super::*;

    #[test]
    fn tya_moves_register_y_to_accumulator() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0xA0, 0x11, 0x98, 0x00]);
        cpu.run();
        assert_eq!(cpu.register_y, 0x11);
        assert_eq!(cpu.register_a, 0x11);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }
}