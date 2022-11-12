use log::debug;
use rust_nes::{cpu::*, ROM::*};
use std::fs;

fn main() {
    env_logger::init();
    let data = fs::read("./src/tests/nestest.nes").expect("can't read file");
    debug!("{:X?}", &data[0..16]);
    let rom = ROM::new(data);
    let mut cpu = CPU::new(rom);
    cpu.run();
}
