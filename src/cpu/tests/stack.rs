use crate::cpu::tests::common::*;
use crate::cpu::*;

mod pha {
    use super::*;

    #[test]
    fn pha_pushes_accumulator() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x48, 0x00]);
        cpu.register_a = 0x42;
        cpu.run();

        assert_eq!(cpu.stack_pointer, 0xFC);
        assert_eq!(cpu.mem_read(0x01FD), 0x42);
    }
}

mod pla {
    use super::*;

    #[test]
    fn pla_pops_accumulator() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x68, 0x00]);
        cpu.stack_push(0x77);
        cpu.run();

        assert_eq!(cpu.register_a, 0x77);
        assert_eq!(cpu.stack_pointer, 0xFD);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIVE));
    }

    #[test]
    fn pla_sets_zero_flag() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x68, 0x00]);
        cpu.stack_push(0x00);
        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn pla_sets_negative_flag() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x68, 0x00]);
        cpu.stack_push(0x80);
        cpu.run();

        assert_eq!(cpu.register_a, 0x80);
        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
    }
}

mod php {
    use super::*;

    #[test]
    fn php_pushes_status() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x08, 0x00]);
        cpu.status = CpuFlags::CARRY | CpuFlags::OVERFLOW;
        cpu.run();

        assert_eq!(cpu.mem_read(0x01FD), (CpuFlags::CARRY | CpuFlags::OVERFLOW).bits());
    }
}

mod plp {
    use super::*;

    #[test]
    fn plp_pops_status() {
        let mut cpu = set_up_cpu();
        cpu.load_and_reset(vec![0x28, 0x00]);
        let flags = CpuFlags::NEGATIVE | CpuFlags::ZERO | CpuFlags::INTERRUPT_DISABLE;
        cpu.stack_push(flags.bits());
        cpu.run();

        assert!(cpu.status.contains(CpuFlags::NEGATIVE));
        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert!(cpu.status.contains(CpuFlags::INTERRUPT_DISABLE));
        assert!(!cpu.status.contains(CpuFlags::CARRY));
    }
}