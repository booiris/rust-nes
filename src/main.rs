use log::debug;
use rust_nes::cpu::*;
use rust_nes::PPU::ppu::PPU;
use std::fs;

fn main() {
    env_logger::init();
    let data = fs::read("./tests/pacman.nes").expect("can't read file");
    debug!("{:X?}", &data[0..16]);
    let mut cpu = CPU::new();
    cpu.load_rom(data.clone());
    cpu.reset();
    let mut ppu = PPU::new();
    ppu.load_rom(data);
}
