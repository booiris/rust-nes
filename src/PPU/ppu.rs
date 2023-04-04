use crate::CONST::{SYSTEM_PALLETE, WIDTH};
use crate::{bus::BUS, memory::PpuMemory, ROM::ROM};
use serde::{Deserialize, Serialize};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

use super::address::AddrRegister;
use super::control::ControlRegister;

pub struct Frame {
    pub data: Vec<u8>,
}
impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Frame {
            data: vec![0; (width) * (height) * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let base = y * 3 * WIDTH + x * 3;
        if base + 2 < self.data.len() {
            self.data[base] = rgb.0;
            self.data[base + 1] = rgb.1;
            self.data[base + 2] = rgb.2;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PPU {
    pub mem: PpuMemory,
    addr: AddrRegister,
    ctrl: ControlRegister,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            mem: PpuMemory::new(),
            addr: AddrRegister::new(),
            ctrl: ControlRegister::new(),
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

    pub fn load_bus(&mut self, sender: Sender<(u16, u8)>, receiver: Receiver<(u16, u8)>) {
        self.mem.bus = Some(BUS::new(sender, receiver));
    }
}

impl PPU {
    pub fn run(&mut self, frame: &mut Frame) {
        loop {
            self.render(frame);
            println!("run ppu!");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

impl PPU {
    fn write_to_ppu_addr(&mut self) {
        let data = self.get_data(0x2006);
        self.addr.update(data);
    }

    fn write_to_ctrl(&mut self) {
        let data = self.get_data(0x2000);
        self.ctrl.update(data);
    }

    fn write_data(&mut self) {
        let addr = self.addr.get();
        let data = self.get_data(0x2007);
        self.mem.storeb(addr, data);
        self.increment_vram_addr();
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    fn read_data(&mut self) -> u8 {
        let mut addr = self.addr.get();
        self.increment_vram_addr();
        self.mem.loadb(&mut addr)
    }

    fn get_data(&self, expect_addr: u16) -> u8 {
        self.mem
            .bus
            .as_ref()
            .expect("not load bus")
            .receive_data(expect_addr)
    }
}

// impl PPU {
//     // Horizontal:
//     //   [ A ] [ a ]
//     //   [ B ] [ b ]

//     // Vertical:
//     //   [ A ] [ B ]
//     //   [ a ] [ b ]
//     pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
//         let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
//         let vram_index = mirrored_vram - 0x2000; // to vram vector
//         let name_table = vram_index / 0x400; // to the name table index
//         match (&self.mirroring, name_table) {
//             (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
//             (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
//             (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
//             (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
//             _ => vram_index,
//         }
//     }
// }

impl PPU {
    pub fn render(&mut self, frame: &mut Frame) {
        let bank = 0;

        for i in 0..0x03c0 {
            let mut addr = i as u16;
            let tile = self.mem.loadb(&mut addr);
            let tile_x = i % 32;
            let tile_y = i / 32;
            let tile = &self.mem.rom.as_ref().unwrap().chr.as_ref().unwrap()
                [(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & upper) << 1 | (1 & lower);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => SYSTEM_PALLETE[0x01],
                        1 => SYSTEM_PALLETE[0x23],
                        2 => SYSTEM_PALLETE[0x27],
                        3 => SYSTEM_PALLETE[0x30],
                        _ => panic!("can't be"),
                    };
                    frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb)
                }
            }
        }
    }
}
