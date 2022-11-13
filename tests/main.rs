use log::debug;
use rust_nes::cpu::*;
use rust_nes::ppu::PPU;
use std::fs;

#[test]
fn main() {
    env_logger::init();
    let data = fs::read("./tests/nestest.nes").expect("can't read file");
    debug!("{:X?}", &data[0..16]);
    let mut cpu = CPU::new();
    cpu.load_rom(data.clone());
    cpu.reset();
    let mut ppu = PPU::new();
    ppu.load_rom(data);
    cpu.run();
}
