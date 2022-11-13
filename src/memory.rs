use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::ROM::ROM;

#[derive(Serialize, Deserialize)]
pub struct CpuMemory {
    #[serde(with = "BigArray")]
    pub ram: [u8; 2048],
    #[serde(skip)]
    pub rom: Option<ROM>,
}

impl CpuMemory {
    pub fn new() -> Self {
        CpuMemory {
            ram: [0; 2048],
            rom: None,
        }
    }
}

impl CpuMemory {
    pub fn storeb(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram[(address & 0x07FF) as usize] = data;
            }
            0x2000..=0x3FFF => match address {
                0x2000 => {}
                0x2006 => {}
                0x2007 => {}
                _ => {
                    let mirror_down_addr = address & 0b00100000_00000111;
                    self.storeb(mirror_down_addr, data)
                }
            },
            0x4000..=0x7FFF => {
                todo!("RAM PPU on ROM not impl!")
            }
            0x8000..=0xFFFF => {
                self.rom
                    .as_mut()
                    .expect("not load rom!")
                    .write(address, data);
            }
        }
    }

    pub fn storew(&mut self, address: u16, data: u16) {
        self.storeb(address, (data & 0xFF) as u8);
        self.storeb(address + 1, ((data >> 8) & 0xFF) as u8)
    }

    pub fn loadb(&self, address: &mut u16) -> u8 {
        let res = match address {
            0x0000..=0x1FFF => self.ram[(*address & 0x07FF) as usize],
            0x2000..=0x3FFF => match address {
                0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                    panic!("Attempt to read from write-only PPU address {:x}", address);
                }
                0x2002 => {
                    todo!()
                }
                0x2007 => {
                    todo!()
                }
                _ => {
                    let mut mirror_down_addr = *address & 0b00100000_00000111;
                    self.loadb(&mut mirror_down_addr)
                }
            },
            0x4000..=0x7FFF => {
                todo!("RAM PPU on ROM not impl!")
            }
            0x8000..=0xFFFF => self.rom.as_ref().expect("not load rom!").read(address),
        };
        *address += 1;
        res
    }

    pub fn loadw(&self, address: &mut u16) -> u16 {
        let low = self.loadb(address) as u16;
        let high = (self.loadb(address) as u16) << 8;
        high | low
    }
}

#[derive(Serialize, Deserialize)]
pub struct PpuMemory {
    #[serde(with = "BigArray")]
    pub ram: [u8; 2048],
    #[serde(skip)]
    pub rom: Option<ROM>,
    palette_table: [u8; 32],
}

impl PpuMemory {
    pub fn new() -> Self {
        PpuMemory {
            ram: [0; 2048],
            rom: None,
            palette_table: [0; 32],
        }
    }
}

impl PpuMemory {
    pub fn storeb(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram[(address & 0x07FF) as usize] = data;
        } else if address < 0x6000 {
            todo!()
        } else {
            self.rom
                .as_mut()
                .expect("not load rom!")
                .write(address, data);
        }
    }

    pub fn storew(&mut self, address: u16, data: u16) {
        self.storeb(address, (data & 0xFF) as u8);
        self.storeb(address + 1, ((data >> 8) & 0xFF) as u8)
    }

    pub fn loadb(&self, address: &mut u16) -> u8 {
        let res;
        if *address < 0x2000 {
            res = self.ram[(*address & 0x07FF) as usize];
        } else if *address < 0x6000 {
            todo!()
        } else {
            res = self.rom.as_ref().expect("not load rom!").read(address);
        }
        *address += 1;
        res
    }

    pub fn loadw(&self, address: &mut u16) -> u16 {
        let low = self.loadb(address) as u16;
        let high = (self.loadb(address) as u16) << 8;
        high | low
    }
}
