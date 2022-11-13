use log::debug;
use rust_nes::cpu::*;
use rust_nes::ppu::PPU;
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

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

    init();

}

fn init(){
    let _controller: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _mask: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _status: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _oma_addr: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _oma_data: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _scroll: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let addr: (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let data: (Sender<u8>, Receiver<u8>) = mpsc::channel();
}