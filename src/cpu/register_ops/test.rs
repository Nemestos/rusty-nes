use crate::cpu::{CpuFlags, CPU};

/*A,X,Y Registers */
#[test]
fn test_cpx() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xa2, 0x10, 0xe0, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0x010);
    cpu.load_and_run(vec![0xa2, 0x10, 0xe0, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_cpy() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xa0, 0x10, 0xc0, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::CARRY));

    cpu.mem_write(0x10, 0x010);
    cpu.load_and_run(vec![0xa0, 0x10, 0xc0, 0x10, 0x00]);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}
#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(!cpu.status.contains(CpuFlags::ZERO));
    assert!(!cpu.status.contains(CpuFlags::NEGATIV));
}

#[test]
fn test_0xa5_lda_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);

    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn test_0xb5_lda_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x20, 0x55);

    cpu.load_and_run(vec![0xa9, 0x10, 0xaa, 0xb5, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status.contains(CpuFlags::ZERO));
}

#[test]
fn test_0xa9_lda_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x00]);

    assert!(cpu.status.contains(CpuFlags::NEGATIV));
}

#[test]
fn test_dex() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0xca, 0x00]);
    assert_eq!(cpu.register_x, 0x04);
}
#[test]
fn test_dey() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x88, 0x00]);
    assert_eq!(cpu.register_y, 0x04);
}

#[test]
fn test_ldx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0xff, 0x00]);

    assert_eq!(cpu.register_x, 0xff);
}
#[test]
fn test_ldy() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0xff, 0x00]);

    assert_eq!(cpu.register_y, 0xff);
}

#[test]
fn test_0xaa_tax_copy_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0x00]);

    assert_eq!(cpu.register_a, cpu.register_x);
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10)
}
#[test]
fn test_inc() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x05);
    cpu.load_and_run(vec![0xee, 0x10]);

    assert_eq!(cpu.mem_read(0x10), 0x06);
}

#[test]
fn test_inx() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0x06);
}
#[test]
fn test_iny() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0xc8, 0x00]);

    assert_eq!(cpu.register_y, 0x06);
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}

/*End A,X,Y Registers */
