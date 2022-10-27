use crate::cartridge::Mirroring;

use super::{Rom, CHR_ROM_PAGE_SIZE, PRG_ROM_PAGE_SIZE};

struct TestRom {
    header: Vec<u8>,
    trainer: Option<Vec<u8>>,
    pgp_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

fn create_rom(rom: TestRom) -> Vec<u8> {
    let mut res = Vec::with_capacity(
        rom.header.len()
            + rom.trainer.as_ref().map_or(0, |t| t.len())
            + rom.pgp_rom.len()
            + rom.chr_rom.len(),
    );
    res.extend(&rom.header);
    if let Some(t) = rom.trainer {
        res.extend(t);
    }
    res.extend(&rom.pgp_rom);
    res.extend(&rom.chr_rom);
    res
}

pub fn gen_test_rom() -> Rom {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![1; 2 * CHR_ROM_PAGE_SIZE],
    });
    Rom::new(&test_rom).unwrap()
}

#[test]
fn test() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });

    let rom: Rom = Rom::new(&test_rom).unwrap();

    assert_eq!(rom.chr_rom, vec!(2; 1 * CHR_ROM_PAGE_SIZE));
    assert_eq!(rom.prg_rom, vec!(1; 2 * PRG_ROM_PAGE_SIZE));
    assert_eq!(rom.mapper, 3);
    assert_eq!(rom.screen_mirroring, Mirroring::VERTICAL);
}

#[test]
fn test_with_trainer() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E,
            0x45,
            0x53,
            0x1A,
            0x02,
            0x01,
            0x31 | 0b100,
            00,
            00,
            00,
            00,
            00,
            00,
            00,
            00,
            00,
        ],
        trainer: Some(vec![0; 512]),
        pgp_rom: vec![1; 2 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });

    let rom: Rom = Rom::new(&test_rom).unwrap();

    assert_eq!(rom.chr_rom, vec!(2; 1 * CHR_ROM_PAGE_SIZE));
    assert_eq!(rom.prg_rom, vec!(1; 2 * PRG_ROM_PAGE_SIZE));
    assert_eq!(rom.mapper, 3);
    assert_eq!(rom.screen_mirroring, Mirroring::VERTICAL);
}

#[test]
fn test_nes2_is_not_supported() {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x01, 0x01, 0x31, 0x8, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 1 * PRG_ROM_PAGE_SIZE],
        chr_rom: vec![2; 1 * CHR_ROM_PAGE_SIZE],
    });
    let rom = Rom::new(&test_rom);
    match rom {
        Result::Ok(_) => assert!(false, "should not load rom"),
        Result::Err(str) => assert_eq!(str, "iNES2.0 is not supported yet"),
    }
}
