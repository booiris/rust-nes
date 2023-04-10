use log::debug;
use rust_nes::cpu::*;
use rust_nes::ppu_impl::ppu::PPU;
use std::fs;
use std::sync::mpsc;

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

    let cpu_bus = mpsc::channel();
    let ppu_bus = mpsc::channel();
    cpu.load_bus(cpu_bus.0, ppu_bus.1);
    ppu.load_bus(ppu_bus.0, cpu_bus.1);

    cpu.run();
}

fn _init() {
    // let _controller: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let _mask: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let _status: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let _oma_addr: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let _oma_data: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let _scroll: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let addr: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let data: (Sender<u8>, Receiver<u8>) = mpsc::channel();
}
