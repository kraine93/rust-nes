#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

pub mod bus;
pub mod cpu;
pub mod opcodes;
pub mod ppu;
pub mod render;
pub mod rom;
pub mod trace;

use crate::bus::Bus;
use crate::cpu::{Mem, CPU};
use crate::ppu::NesPPU;
use crate::rom::Rom;
use crate::trace::trace;

fn main() {
    let bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let bus = Bus::new(rom, |ppu: &NesPPU| {});
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.program_counter = 0xc000;

    let mut screen_state = [0 as u8; 32 * 32 * 3];
    let mut rng = rand::thread_rng();

    cpu.run_with_callback(move |cpu| {
        println!("{}", trace(cpu));
    });
}
