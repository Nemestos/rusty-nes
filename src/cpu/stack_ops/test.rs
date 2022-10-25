use crate::cpu::{CpuFlags, CPU};
#[test]
fn test_pha() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x48, 0x00]);
    let pushed = cpu.stack_pull();
    assert_eq!(pushed, 0x05);
}
