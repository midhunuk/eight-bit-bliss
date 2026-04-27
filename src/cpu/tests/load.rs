mod lda {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn lda_immediate_param_is_loaded_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA9, 0x11, 0x00]);
        assert_eq!(cpu.register_a, 0x11);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn lda_param_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    fn lda_param_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();
        cpu.load_and_run(vec![0xA9, 0xA0, 0x00]);
        assert_eq!(cpu.register_a, 0xA0);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn lda_zero_page_param_is_loaded_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x11, 0xff);
        cpu.load_and_run(vec![0xA5, 0x11, 0x00]);
        assert_eq!(cpu.register_a, 0xff);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn lda_absolute_param_is_loaded_to_register_a() {
        let mut cpu = set_up_cpu();
        cpu.mem_write(0x1234, 0xaa);
        cpu.load_and_run(vec![0xAD, 0x34, 0x12, 0x00]);
        assert_eq!(cpu.register_a, 0xaa);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }
}

mod ldx {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn ldx_immediate_param_is_loaded_to_register_x() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0xA2, 0x11, 0x00]);

        assert_eq!(cpu.register_x, 0x11);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn ldx_param_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0xA2, 0x00, 0x00]);

        assert_eq!(cpu.register_x, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    fn ldx_param_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0xA2, 0xA0, 0x00]);

        assert_eq!(cpu.register_x, 0xA0);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn ldx_zero_page_param_is_loaded_to_register_x() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x11, 0xff);

        cpu.load_and_run(vec![0xA6, 0x11, 0x00]);

        assert_eq!(cpu.register_x, 0xff);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn ldx_zeropage_y_loads_data() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x0010 + 0x05, 0xAA);
        cpu.load_and_reset(vec![0xB6, 0x10, 0x00]);
        cpu.register_y = 0x05;

        cpu.run();

        assert_eq!(cpu.register_x, 0xAA);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }
    
    #[test]
    fn ldx_absolute_param_is_loaded_to_register_x() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x1234, 0xAA);

        cpu.load_and_run(vec![0xAE, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.register_x, 0xAA);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn ldx_absolute_y_param_is_loaded_to_register_x() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x1234 + 0x05, 0xBB);
        cpu.load_and_reset(vec![0xBE, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x05;

        cpu.run();

        assert_eq!(cpu.register_x, 0xBB);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }
}

mod ldy {
    use crate::cpu::tests::common::*;
    use crate::cpu::*;

    #[test]
    fn ldy_immediate_param_is_loaded_to_register_y() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0xA0, 0x11, 0x00]);

        assert_eq!(cpu.register_y, 0x11);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn ldy_param_is_0_zero_flag_is_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0xA0, 0x00, 0x00]);

        assert_eq!(cpu.register_y, 0x00);
        assert_eq!(cpu.status, 0b0000_0010);
    }

    #[test]
    fn ldy_param_is_negative_negative_flag_is_set() {
        let mut cpu = set_up_cpu();

        cpu.load_and_run(vec![0xA0, 0xA0, 0x00]);

        assert_eq!(cpu.register_y, 0xA0);
        assert_eq!(cpu.status, 0b1000_0000);
    }

    #[test]
    fn ldy_zero_page_param_is_loaded_to_register_y() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x11, 0xff);

        cpu.load_and_run(vec![0xA4, 0x11, 0x00]);

        assert_eq!(cpu.register_y, 0xff);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }

    #[test]
    fn ldy_zeropage_x_loads_data() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x0010 + 0x05, 0xAA);
        cpu.load_and_reset(vec![0xB4, 0x10, 0x00]);
        cpu.register_x = 0x05;

        cpu.run();

        assert_eq!(cpu.register_y, 0xAA);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 3);
    }
    
    #[test]
    fn ldy_absolute_param_is_loaded_to_register_y() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x1234, 0xAA);

        cpu.load_and_run(vec![0xAC, 0x34, 0x12, 0x00]);

        assert_eq!(cpu.register_y, 0xAA);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }

    #[test]
    fn ldy_absolute_x_param_is_loaded_to_register_y() {
        let mut cpu = set_up_cpu();

        cpu.mem_write(0x1234 + 0x05, 0xBB);
        cpu.load_and_reset(vec![0xBC, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x05;

        cpu.run();

        assert_eq!(cpu.register_y, 0xBB);
        assert_eq!(cpu.program_counter, PROGRAM_START_VALUE + 4);
    }
}