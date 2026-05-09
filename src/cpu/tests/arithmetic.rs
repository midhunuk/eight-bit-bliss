mod inx {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn inx_value_incremented_by_1() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA2, 0x11, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0x12);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn inx_zero_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA2, 0xFF, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn inx_wrapping_and_negative_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA2, 0x7F, 0xE8, 0x00]);

        assert_eq!(cpu.register_x, 0x80);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod iny {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn iny_value_incremented_by_1() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA0, 0x11, 0xC8, 0x00]);

        assert_eq!(cpu.register_y, 0x12);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn iny_zero_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA0, 0xFF, 0xC8, 0x00]);

        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn iny_wrapping_and_negative_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA0, 0x7F, 0xC8, 0x00]);

        assert_eq!(cpu.register_y, 0x80);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod dex {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn dex_value_decremented_by_1() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA2, 0x05, 0xCA, 0x00]);

        assert_eq!(cpu.register_x, 0x04);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn dex_zero_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA2, 0x01, 0xCA, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn dex_wrapping_and_negative_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA2, 0x00, 0xCA, 0x00]);

        assert_eq!(cpu.register_x, 0xFF);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }
}

mod dey {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn dey_value_decremented_by_1() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA0, 0x05, 0x88, 0x00]);

        assert_eq!(cpu.register_y, 0x04);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn dey_zero_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA0, 0x01, 0x88, 0x00]);

        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn dey_wrapping_and_negative_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA0, 0x00, 0x88, 0x00]);

        assert_eq!(cpu.register_y, 0xFF);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }
}

mod dec {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn dec_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x04);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn dec_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xD6, 0x10, 0x00]);
        cpu.register_x = 0x02;
        cpu.run();

        assert_eq!(cpu.mem_read(0x12), 0x04);
    }

    #[test]
    fn dec_absolute() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x05);
        cpu.load_and_run(vec![0xCE, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.mem_read(0x1234), 0x04);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn dec_absolute_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xDE, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x02;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1236), 0x04);
    }

    #[test]
    fn dec_zero_flag() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x01);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn dec_wrapping_and_negative_flag() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x00);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0xFF);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod inc {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn inc_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xE6, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x06);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn inc_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xF6, 0x10, 0x00]);
        cpu.register_x = 0x02;
        cpu.run();

        assert_eq!(cpu.mem_read(0x12), 0x06);
    }

    #[test]
    fn inc_absolute() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x05);
        cpu.load_and_run(vec![0xEE, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.mem_read(0x1234), 0x06);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn inc_absolute_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x02, 0x05);
        cpu.load_and_reset(vec![0xFE, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x02;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1236), 0x06);
    }

    #[test]
    fn inc_zero_flag() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0xFF);
        cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn inc_negative_flag() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0x7F);
        cpu.load_and_run(vec![0xE6, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0x80);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod adc {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn immediate_value_added_to_register_a() {
        let mut cpu = set_up_cpu();

        cpu.load_and_reset(vec![0x69, 0x36, 0x00]);
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn value_added_to_register_a_carry_flag_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_reset(vec![0x69, 0xFF, 0x00]);
        cpu.register_a = 0xFF;

        cpu.run();

        assert_eq!(cpu.register_a, 0xFE);
        assert_eq!(cpu.status.contains(CpuFlags::CARRY), true);
    }

    #[test]
    fn value_added_to_register_a_result_is_zero_zero_flag_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_reset(vec![0x69, 0xFF, 0x00]);
        cpu.register_a = 0x01;

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status.contains(CpuFlags::ZERO), true);
    }

    #[test]
    fn carry_present_carry_and_value_added_to_register_a() {
        let mut cpu = set_up_cpu();

        cpu.load_and_reset(vec![0x69, 0x01, 0x00]);
        cpu.register_a = 0x01;
        cpu.status.insert(CpuFlags::CARRY);

        cpu.run();

        assert_eq!(cpu.register_a, 0x03);
        assert_eq!(cpu.status.contains(CpuFlags::CARRY), false);
    }

    #[test]
    fn value_added_to_register_a_result_is_negative_value_negative_flag_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_reset(vec![0x69, 0xF6, 0x00]);
        cpu.register_a = 0x01;

        cpu.run();

        assert_eq!(cpu.register_a, 0xF7);
        assert_eq!(cpu.status.contains(CpuFlags::NEGATIVE), true);
    }

    #[test]
    fn value_added_to_register_a_result_is_invalid_overflow_flag_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_reset(vec![0x69, 0x70, 0x00]);
        cpu.register_a = 0x70;

        cpu.run();

        assert_eq!(cpu.register_a, 0xE0);
        assert_eq!(cpu.status.contains(CpuFlags::OVERFLOW), true);
    }

    #[test]
    fn zero_page_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x11, 0x36);
        cpu.load_and_reset(vec![0x65, 0x11, 0x00]);
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn zero_page_x_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x11 + 0x05, 0x36);
        cpu.load_and_reset(vec![0x75, 0x11, 0x00]);
        cpu.register_x = 0x05;
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn absolute_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x36);
        cpu.load_and_reset(vec![0x6D, 0x34, 0x12, 0x00]);
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn absolute_x_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x10, 0x36);

        cpu.load_and_reset(vec![0x7D, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x10;
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn absolute_y_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x10, 0x36);

        cpu.load_and_reset(vec![0x79, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x10;
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn indirect_x_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0x36);
        cpu.mem_write(0x44, 0x34);
        cpu.mem_write(0x45, 0x12);

        cpu.load_and_reset(vec![0x61, 0x40, 0x00]);
        cpu.register_x = 0x04;
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn indirect_y_param_value_added_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x04, 0x36);
        cpu.mem_write(0x44, 0x34);
        cpu.mem_write(0x45, 0x12);

        cpu.load_and_reset(vec![0x71, 0x44, 0x00]);
        cpu.register_y = 0x04;
        cpu.register_a = 0x54;

        cpu.run();

        assert_eq!(cpu.register_a, 0x8A);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }
}

mod and {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn immediate_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x29, 0xCC, 0x00]);
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn result_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x29, 0x00, 0x00]);
        cpu.register_a = 0x69;

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.status.contains(CpuFlags::ZERO), true);
    }

    #[test]
    fn result_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x29, 0xF0, 0x00]);
        cpu.register_a = 0xF9;

        cpu.run();

        assert_eq!(cpu.status.contains(CpuFlags::NEGATIVE), true);
    }

    #[test]
    fn zero_page_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x11, 0xCC);
        cpu.load_and_reset(vec![0x25, 0x11, 0x00]);
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn zero_page_x_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x11 + 0x05, 0xCC);

        cpu.load_and_reset(vec![0x35, 0x11, 0x00]);
        cpu.register_x = 0x05;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn absolute_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0xCC);

        cpu.load_and_reset(vec![0x2D, 0x34, 0x12, 0x00]);
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn absolute_x_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x10, 0xCC);

        cpu.load_and_reset(vec![0x3D, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x10;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn absolute_y_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x10, 0xaa);

        cpu.load_and_reset(vec![0x39, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x10;
        cpu.register_a = 0xCC;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn indirect_x_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0xaa);
        cpu.mem_write(0x44, 0x34);
        cpu.mem_write(0x45, 0x12);

        cpu.load_and_reset(vec![0x21, 0x40, 0x00]);
        cpu.register_x = 0x04;
        cpu.register_a = 0xCC;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn indirect_y_vaue_and_operation_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x04, 0xaa);
        cpu.mem_write(0x44, 0x34);
        cpu.mem_write(0x45, 0x12);

        cpu.load_and_reset(vec![0x31, 0x44, 0x00]);
        cpu.register_y = 0x04;
        cpu.register_a = 0xCC;

        cpu.run();

        assert_eq!(cpu.register_a, 0x88);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }
}

mod asl {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn asl_accumulator_basic_shift() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x0A, 0x00]);
        cpu.register_a = 0b0000_0011;

        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0110);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
    }

    #[test]
    fn asl_accumulator_sets_carry() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x0A, 0x00]);
        cpu.register_a = 0b1000_0001;

        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0010);
        assert!(cpu.status.contains(CpuFlags::CARRY));
    }

    #[test]
    fn asl_accumulator_zero_flag() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x0A, 0x00]);
        cpu.register_a = 0b0000_0000;

        cpu.run();

        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn asl_accumulator_negative_flag() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x0A, 0x00]);
        cpu.register_a = 0b0100_0000;

        cpu.run();

        assert_eq!(cpu.register_a, 0b1000_0000);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn asl_zeropage_basic() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0b0000_0011);

        cpu.load_and_run(vec![0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0b0000_0110);
    }

    #[test]
    fn asl_zeropage_zero_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0b1000_0000);

        cpu.load_and_run(vec![0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn asl_zeropage_negative_flag_set() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0b0100_0000);

        cpu.load_and_run(vec![0x06, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0b1000_0000);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn asl_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10 + 0x05, 0b0000_0010);
        cpu.load_and_reset(vec![0x16, 0x10, 0x00]);
        cpu.register_x = 0x05;

        cpu.run();

        assert_eq!(cpu.mem_read(0x15), 0b0000_0100);
    }

    #[test]
    fn asl_absolute() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0b0000_0011);

        cpu.load_and_run(vec![0x0E, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.mem_read(0x1234), 0b0000_0110);
    }

    #[test]
    fn asl_absolute_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x01, 0b0100_0000);
        cpu.load_and_reset(vec![0x1E, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x01;

        cpu.run();

        assert_eq!(cpu.mem_read(0x1235), 0b1000_0000);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod bit {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn bit_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0b1100_0000);
        cpu.load_and_reset(vec![0x24, 0x10, 0x00]);
        cpu.register_a = 0x00;
        cpu.run();

        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(cpu.status.contains(CpuFlags::OVERFLOW));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn bit_absolute() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0b1100_0000);
        cpu.load_and_reset(vec![0x2C, 0x34, 0x12, 0x00]);
        cpu.register_a = 0x00;
        cpu.run();

        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(cpu.status.contains(CpuFlags::OVERFLOW));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn bit_zero_flag() {
        let mut cpu = set_up_cpu();
        
        // A & M == 0 sets Zero flag
        cpu.mem_write(0x10, 0b0000_1111);
        cpu.load_and_reset(vec![0x24, 0x10, 0x00]);
        cpu.register_a = 0b1111_0000;
        cpu.run();
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn bit_negative_and_overflow_flags() {
        let mut cpu = set_up_cpu();

        // Bit 7 is 1 sets Negative flag, bit 6 is 0 clears Overflow
        cpu.mem_write(0x10, 0b1000_0000);
        cpu.load_and_reset(vec![0x24, 0x10, 0x00]);
        cpu.run();
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(!cpu.status.contains(CpuFlags::OVERFLOW));

        // Bit 7 is 0 clears Negative flag, bit 6 is 1 sets Overflow
        cpu.mem_write(0x10, 0b0100_0000);
        cpu.load_and_reset(vec![0x24, 0x10, 0x00]);
        cpu.run();
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(cpu.status.contains(CpuFlags::OVERFLOW));
    }
}

mod eor {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn eor_immediate() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x49, 0b1111_0000, 0x00]);
        cpu.register_a = 0b1010_1010;

        cpu.run();

        assert_eq!(cpu.register_a, 0b0101_1010);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn eor_zero_flag() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x49, 0xFF, 0x00]);
        cpu.register_a = 0xFF;

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn eor_negative_flag() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x49, 0x80, 0x00]);
        cpu.register_a = 0x00;

        cpu.run();

        assert_eq!(cpu.register_a, 0x80);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn eor_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0xCC);
        cpu.load_and_reset(vec![0x45, 0x10, 0x00]);
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn eor_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10 + 0x05, 0xCC);
        cpu.load_and_reset(vec![0x55, 0x10, 0x00]);
        cpu.register_x = 0x05;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn eor_absolute() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0xCC);
        cpu.load_and_reset(vec![0x4D, 0x34, 0x12, 0x00]);
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn eor_absolute_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x10, 0xCC);
        cpu.load_and_reset(vec![0x5D, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x10;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn eor_absolute_y() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x10, 0xCC);
        cpu.load_and_reset(vec![0x59, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x10;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn eor_indirect_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0xCC);
        cpu.mem_write(0x44, 0x34);
        cpu.mem_write(0x45, 0x12);

        cpu.load_and_reset(vec![0x41, 0x40, 0x00]);
        cpu.register_x = 0x04;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }

    #[test]
    fn eor_indirect_y() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x04, 0xCC);
        cpu.mem_write(0x44, 0x34);
        cpu.mem_write(0x45, 0x12);

        cpu.load_and_reset(vec![0x51, 0x44, 0x00]);
        cpu.register_y = 0x04;
        cpu.register_a = 0xAA;

        cpu.run();

        assert_eq!(cpu.register_a, 0x66);
    }
}

mod lsr {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn lsr_accumulator_basic_shift() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x4A, 0x00]);
        cpu.register_a = 0b0000_0110;

        cpu.run();

        assert_eq!(cpu.register_a, 0b0000_0011);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn lsr_accumulator_sets_carry() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x4A, 0x00]);
        cpu.register_a = 0b0000_0001;

        cpu.run();

        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.contains(CpuFlags::CARRY));
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn lsr_accumulator_negative_flag_cleared() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x4A, 0x00]);
        cpu.register_a = 0b1000_0000;

        cpu.run();

        assert_eq!(cpu.register_a, 0b0100_0000);
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(!cpu.status.contains(CpuFlags::CARRY));
    }

    #[test]
    fn lsr_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10, 0b0000_0110);
        cpu.load_and_run(vec![0x46, 0x10, 0x00]);

        assert_eq!(cpu.mem_read(0x10), 0b0000_0011);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn lsr_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x10 + 0x05, 0b0000_0110);
        cpu.load_and_reset(vec![0x56, 0x10, 0x00]);
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.mem_read(0x15), 0b0000_0011);
    }

    #[test]
    fn lsr_absolute() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0b0000_0110);
        cpu.load_and_run(vec![0x4E, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.mem_read(0x1234), 0b0000_0011);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn lsr_absolute_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234 + 0x01, 0b0000_0110);
        cpu.load_and_reset(vec![0x5E, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x01;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1235), 0b0000_0011);
    }
}
