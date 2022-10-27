use crate::{
    bus::Bus,
    cartridge::test::gen_test_rom,
    cpu::{CpuFlags, CPU},
    mem::Mem,
};

/*Arithmethic & logic */
#[test]
fn test_and_working() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0xEC, 0x29, 0xE0, 0x00]);
    assert_eq!(cpu.register_a, 0xe0);
}
#[test]
fn test_adc() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x10, 0x69, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x20);
    assert!(!cpu.status.contains(CpuFlags::CARRY));
    assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
}
#[test]
fn test_adc_carry() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY))
}
#[test]
fn test_adc_overflow_two_positiv() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x50, 0x69, 0x50, 0x00]);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW))
}

#[test]
fn test_adc_overflow_two_negativ() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0xd0, 0x69, 0x90, 0x00]);
    assert!(cpu.status.contains(CpuFlags::OVERFLOW))
}

#[test]
fn test_asl_acu() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x50, 0x0A, 0x00]);
    assert_eq!(cpu.register_a, 0x50 << 1);
}

#[test]
fn test_bit() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
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
fn test_dec() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xce, 0x10, 0x00]);
    assert_eq!(cpu.mem_read(0x10), 0x04);
}

#[test]
fn test_cmp() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0x010);
    cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_eor() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x09);
    cpu.load_and_run(vec![0xa9, 0x10, 0x4D, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x10 ^ 0x09);
}

#[test]
fn test_lsr() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x08);
    cpu.load_and_run(vec![0x4e, 0x10, 0x00]);
    assert_eq!(cpu.mem_read(0x10), 0x04)
}

#[test]
fn test_ora() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x10, 0x09);
    cpu.load_and_run(vec![0xa9, 0x05, 0x0d, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x05 | 0x09)
}

#[test]
fn test_rol() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0b1100_1111, 0x2a, 0x00]);
    assert_eq!(cpu.register_a, 0b1001_1111);
}

#[test]
fn test_ror() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0b1100_1111, 0x6a, 0x00]);
    assert_eq!(cpu.register_a, 0b1110_0111);
}

#[test]
fn test_sbc() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x50, 0xe9, 0x30, 0x00]);
    assert_eq!(cpu.register_a, 0x1f);
}
#[test]
fn test_sbc_carry() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x02, 0xe9, 0x01, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
}

/*End Arithmethic & logic */
