use crate::{
    bus::Bus,
    cartridge::test::gen_test_rom,
    cpu::{CpuFlags, CPU},
    mem::Mem,
};

#[test]
fn test_bcc() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0x18, 0x90, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);

    cpu.load_and_run(vec![0x38, 0x90, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x00)
}

#[test]
fn test_bcs() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0x18, 0xb0, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x00);

    cpu.load_and_run(vec![0x38, 0xb0, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05)
}

#[test]
fn test_beq() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x00, 0xf0, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);

    cpu.load_and_run(vec![0xa9, 0x07, 0xf0, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x07)
}

#[test]
fn test_bne() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x00, 0xd0, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x00);

    cpu.load_and_run(vec![0xa9, 0x07, 0xd0, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05)
}

#[test]
fn test_bmi() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0b1000_0000, 0x30, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);

    cpu.load_and_run(vec![0xa9, 0b0000_0101, 0x30, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0b0000_0101)
}

#[test]
fn test_bpl() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0b1000_0000, 0x10, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0b1000_0000);

    cpu.load_and_run(vec![0xa9, 0b0000_0101, 0x10, 0x01, 0x00, 0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05)
}

#[test]
fn test_bvc() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![
        0xa9, 0x06, 0x69, 0x02, 0x50, 0x01, 0x00, 0xa9, 0x05, 0x00,
    ]);
    assert_eq!(cpu.register_a, 0x05);

    cpu.load_and_run(vec![
        0xa9, 0x50, 0x69, 0x50, 0x50, 0x01, 0x00, 0xa9, 0x05, 0x00,
    ]);
    assert_ne!(cpu.register_a, 0x05)
}

#[test]
fn test_bvs() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![
        0xa9, 0x06, 0x69, 0x02, 0x70, 0x01, 0x00, 0xa9, 0x05, 0x00,
    ]);
    assert_eq!(cpu.register_a, 0x08);

    cpu.load_and_run(vec![
        0xa9, 0x50, 0x69, 0x50, 0x70, 0x01, 0x00, 0xa9, 0x05, 0x00,
    ]);
    assert_eq!(cpu.register_a, 0x05)
}

#[test]
fn test_jmp() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x07, 0xa9);
    cpu.mem_write(0x08, 0x05);
    cpu.mem_write(0x09, 0x00);
    cpu.load_and_run(vec![0x4c, 0x07, 0x00]);
    assert_eq!(cpu.register_a, 0x05)
}

#[test]
fn test_jmp_indirect() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x07, 0x09);
    cpu.mem_write(0x09, 0xa9);
    cpu.mem_write(0xA, 0x08);
    cpu.mem_write(0xB, 0x00);
    cpu.load_and_run(vec![0x6c, 0x07, 0x00]);
    assert_eq!(cpu.register_a, 0x08)
}

#[test]
fn test_jsr() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.mem_write(0x07, 0x38);
    cpu.mem_write(0x08, 0xf8);
    cpu.mem_write(0x09, 0x00);
    cpu.load_and_run(vec![0x20, 0x07, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
}

// #[test]
// fn test_rts() {
//     let bus = Bus::new(gen_test_rom());
// let mut cpu = CPU::new(bus);
//     cpu.mem_write(0x08, 0x38);
//     cpu.mem_write(0x09, 0x60);
//     cpu.load_and_run(vec![0x20, 0x08, 0xf8, 0x00]);
//     assert!(cpu.status.contains(CpuFlags::CARRY));
//     assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
// }
