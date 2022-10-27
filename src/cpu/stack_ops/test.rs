use crate::{
    bus::Bus,
    cartridge::test::gen_test_rom,
    cpu::{CpuFlags, CPU},
};
#[test]
fn test_pha() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x05, 0x48, 0x00]);
    let pushed = cpu.stack_pull();
    assert_eq!(pushed, 0x05);
}

#[test]
fn test_php() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0x38, 0xf8, 0x08, 0x00]);
    let pushed = cpu.stack_pull();
    assert_eq!(pushed, 0b0011_1101);
}

#[test]
fn test_pla() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0xa9, 0x05, 0x48, 0xa9, 0x07, 0x68, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
}

#[test]
fn test_plp() {
    let bus = Bus::new(gen_test_rom());
    let mut cpu = CPU::new(bus);
    cpu.load_and_run(vec![0x38, 0xf8, 0x08, 0x18, 0x28, 0x00]);
    assert_eq!(cpu.status.bits, 0b0000_1101);
}
