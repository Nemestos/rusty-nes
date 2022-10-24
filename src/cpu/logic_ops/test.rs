use crate::cpu::{CpuFlags, CPU};

/*Arithmethic & logic */
#[test]
fn test_and_working() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xEC, 0x29, 0xE0, 0x00]);
    assert_eq!(cpu.register_a, 0xe0);
}
#[test]
fn test_adc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x10, 0x69, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
}
#[test]
fn test_adc_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY))
}
#[test]
fn test_adc_overflow_two_positiv() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x50, 0x69, 0x50, 0x00]);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW))
}

#[test]
fn test_adc_overflow_two_negativ() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xd0, 0x69, 0x90, 0x00]);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW))
}
#[test]
fn test_dec() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xce, 0x10, 0x00]);
    assert_eq!(cpu.mem_read(0x10), 0x04);
}

#[test]
fn test_sbc() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x50, 0xe9, 0x30, 0x00]);
    println!("{:?}", cpu.register_a);
    assert_eq!(cpu.register_a, 0x20);
}
#[test]
fn test_sbc_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x02, 0xe9, 0x01, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}

#[test]
fn test_asl_acu() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x50, 0x0A, 0x00]);
    assert_eq!(cpu.register_a, 0x50 << 1);
}

#[test]
fn test_bit() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x50, 0x2c, 0x00, 0x00]);
    assert!(cpu.status.contains(CpuFlags::ZERO));

    cpu.mem_write(0x10, 0b1000_0000);
    cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x2c, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::NEGATIV));

    cpu.mem_write(0x10, 0b0100_0000);
    cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x2c, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW))
}

#[test]
fn test_cmp() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0x010);
    cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_eor() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x10, 0x49, 0xff]);
    assert_eq!(cpu.register_a, 0x10 ^ 0xff);
}

/*End Arithmethic & logic */