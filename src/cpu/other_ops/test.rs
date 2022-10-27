use crate::{
    cpu::{CpuFlags, CPU},
    mem::Mem,
};
#[test]
fn test_jsr() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x07, 0x38);
    cpu.mem_write(0x08, 0xf8);
    cpu.mem_write(0x09, 0x00);
    cpu.load_and_run(vec![0x20, 0x07, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));
    assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
}

// #[test]
// fn test_rts() {
//     let mut cpu = CPU::new();
//     cpu.mem_write(0x08, 0x38);
//     cpu.mem_write(0x09, 0x60);
//     cpu.load_and_run(vec![0x20, 0x08, 0xf8, 0x00]);
//     assert!(cpu.status.contains(CpuFlags::CARRY));
//     assert!(cpu.status.contains(CpuFlags::DECIMAL_MODE));
// }
