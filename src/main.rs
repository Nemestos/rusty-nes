use cpu::CPU;
use games::load_and_run_snake;

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
}
