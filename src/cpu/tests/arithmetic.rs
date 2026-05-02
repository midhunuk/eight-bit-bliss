mod inx {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn x_value_incremented_by_1() {
        let mut cpu = Cpu::new();

        cpu.load_and_run(vec![0xA9, 0x11, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.register_x, 0x12);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 5);
    }

    #[test]
    fn x_value_incremented_by_1_zero_flag_set() {
        let mut cpu = Cpu::new();

        cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.status.bits(), 0b0000_0010);
    }

    #[test]
    fn x_value_incremented_by_1_negative_flag_set() {
        let mut cpu = Cpu::new();

        cpu.load_and_run(vec![0xA9, 0x7F, 0xAA, 0xE8, 0x00]);

        assert_eq!(cpu.status.bits(), 0b1000_0000);
    }

    #[test]
    fn x_value_incremented_over_flow() {
        let mut cpu = Cpu::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_a, 0xFF);
        assert_eq!(cpu.register_x, 0x01);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 6);
        assert_eq!(cpu.status.bits(), 0b0000_0000);
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
