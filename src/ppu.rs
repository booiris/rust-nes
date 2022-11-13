use std::thread;

use serde::{Deserialize, Serialize};

use crate::{memory::PpuMemory, ROM::ROM};

#[derive(Serialize, Deserialize)]
pub struct PPU {
    pub mem: PpuMemory,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            mem: PpuMemory::new(),
        }
    }

    pub fn load(self, data: Vec<u8>) -> Self {
        let mut save_data: PPU = serde_json::from_slice(&data).expect("decode archive failed!");
        save_data.mem.rom = self.mem.rom;
        save_data
    }

    pub fn save(&mut self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.mem.rom = Some(ROM::new(data, "ppu"));
    }
}

impl PPU {
    pub fn run(&mut self) {
        loop {
            println!("run ppu!");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
