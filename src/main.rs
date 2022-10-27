use std::fs;

use cartridge::Rom;
use cpu::CPU;
use games::{load_and_run_snake, run_snake, SNAKE_GAME};

pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod games;
pub mod mem;
pub mod opcodes;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;
fn main() {
    // let bus = Bus::new(gen_test_rom());
    // let mut cpu: CPU = CPU::new();
    // load_and_run_snake(&mut cpu);

    Rom::create_fake_rom("roms/snake.nes".to_string(), SNAKE_GAME.to_vec());
    let bytes: Vec<u8> = std::fs::read("roms/snake_2.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let bus = bus::Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();
    run_snake(&mut cpu);
}
