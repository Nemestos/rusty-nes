use cpu::CPU;
use games::{load_and_run_snake, snake_game};

mod cpu;
mod games;
mod opcodes;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;
fn main() {
    let mut cpu = CPU::new();
    load_and_run_snake(&mut cpu);
}
