use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::{bus::Bus, ROM::ROM};

#[derive(Serialize, Deserialize)]
pub struct CpuMemory {
    #[serde(with = "BigArray")]
    pub ram: [u8; 2048],
    #[serde(skip)]
    pub rom: Option<ROM>,
    #[serde(skip)]
    pub bus: Option<Bus>,
}

impl CpuMemory {
    pub fn new() -> Self {
        CpuMemory {
            ram: [0; 2048],
            rom: None,
            bus: None,
        }
    }
}

impl Default for CpuMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuMemory {
    pub fn storeb(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram[(address & 0x07FF) as usize] = data;
            }
            0x2000..=0x3FFF => match address {
                0x2000 | 0x2006 | 0x2007 => self
                    .bus
                    .as_ref()
                    .expect("cpu has no bus!")
                    .send_data(address, data),
                _ => {
                    let mirror_down_addr = address & 0b0010_0000_0000_0111;
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
                0x2007 => self
                    .bus
                    .as_ref()
                    .expect("cpu has no bus!")
                    .receive_data(*address),
                _ => {
                    let mut mirror_down_addr = *address & 0b0010_0000_0000_0111;
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
    #[serde(skip)]
    pub bus: Option<Bus>,
    palette_table: [u8; 32],
    internal_data_buf: u8,
    #[serde(with = "BigArray")]
    pub oam_data: [u8; 256],
}

impl PpuMemory {
    pub fn new() -> Self {
        PpuMemory {
            ram: [0; 2048],
            rom: None,
            bus: None,
            palette_table: [0; 32],
            internal_data_buf: 0,
            oam_data: [0; 64 * 4],
        }
    }
}

impl Default for PpuMemory {
    fn default() -> Self {
        Self::new()
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

    pub fn loadb(&mut self, address: &mut u16) -> u8 {
        match *address {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.rom.as_ref().expect("not load chr").read(address);
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.ram[*address as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                *address
            ),
            0x3f00..=0x3fff => self.palette_table[(*address - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", *address),
        }
    }

    pub fn loadw(&mut self, address: &mut u16) -> u16 {
        let low = self.loadb(address) as u16;
        let high = (self.loadb(address) as u16) << 8;
        high | low
    }
}
