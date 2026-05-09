use crate::cpu::tests::common::*;
use crate::cpu::*;

mod sta {
    use super::*;

    #[test]
    fn sta_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x85, 0x10, 0x00]);
        cpu.register_a = 0x42;
        cpu.run();

        assert_eq!(cpu.mem_read(0x10), 0x42);
    }

    #[test]
    fn sta_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x95, 0x10, 0x00]);
        cpu.register_a = 0x42;
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.mem_read(0x15), 0x42);
    }

    #[test]
    fn sta_absolute() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x8D, 0x34, 0x12, 0x00]);
        cpu.register_a = 0x42;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), 0x42);
    }

    #[test]
    fn sta_absolute_x() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x9D, 0x34, 0x12, 0x00]);
        cpu.register_a = 0x42;
        cpu.register_x = 0x10;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1244), 0x42);
    }

    #[test]
    fn sta_absolute_y() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x99, 0x34, 0x12, 0x00]);
        cpu.register_a = 0x42;
        cpu.register_y = 0x10;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1244), 0x42);
    }

    #[test]
    fn sta_indirect_x() {
        let mut cpu = set_up_cpu();
        cpu.mem_write_u16(0x24, 0x1234);
        cpu.load_and_reset(vec![0x81, 0x20, 0x00]);
        cpu.register_a = 0x42;
        cpu.register_x = 0x04;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), 0x42);
    }

    #[test]
    fn sta_indirect_y() {
        let mut cpu = set_up_cpu();
        cpu.mem_write_u16(0x20, 0x1234);
        cpu.load_and_reset(vec![0x91, 0x20, 0x00]);
        cpu.register_a = 0x42;
        cpu.register_y = 0x04;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1238), 0x42);
    }
}

mod stx {
    use super::*;

    #[test]
    fn stx_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x86, 0x10, 0x00]);
        cpu.register_x = 0x42;
        cpu.run();

        assert_eq!(cpu.mem_read(0x10), 0x42);
    }

    #[test]
    fn stx_zeropage_y() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x96, 0x10, 0x00]);
        cpu.register_x = 0x42;
        cpu.register_y = 0x05;
        cpu.run();

        assert_eq!(cpu.mem_read(0x15), 0x42);
    }

    #[test]
    fn stx_absolute() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x8E, 0x34, 0x12, 0x00]);
        cpu.register_x = 0x42;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), 0x42);
    }
}

mod sty {
    use super::*;

    #[test]
    fn sty_zeropage() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x84, 0x10, 0x00]);
        cpu.register_y = 0x42;
        cpu.run();

        assert_eq!(cpu.mem_read(0x10), 0x42);
    }

    #[test]
    fn sty_zeropage_x() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x94, 0x10, 0x00]);
        cpu.register_y = 0x42;
        cpu.register_x = 0x05;
        cpu.run();

        assert_eq!(cpu.mem_read(0x15), 0x42);
    }

    #[test]
    fn sty_absolute() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x8C, 0x34, 0x12, 0x00]);
        cpu.register_y = 0x42;
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), 0x42);
    }
}