use super::{Mapper, ROM};

#[allow(dead_code)]
pub struct Mapper0 {}

impl Mapper for Mapper0 {
    fn read(&self, rom: &ROM, address: &mut u16) -> u8 {
        if *address < 0x2000 {
            rom.chr
                .as_ref()
                .expect("chr is none, maybe you are using cpu to read chr")[*address as usize]
        } else if *address >= 0x8000 {
            let prg = rom
                .prg
                .as_ref()
                .expect("prg is none, maybe you are using ppu to read prg");
            let mut address = *address - 0x8000;
            if prg.len() == 0x4000 && address >= 0x4000 {
                address -= 0x4000;
            }
            prg[address as usize]
        } else {
            panic!("{:X?} address out of range!", address)
        }
    }

    fn write(&mut self, _ram: &mut Vec<u8>, address: u16, _value: u8) {
        panic!("{:X?} Attempt to write to Cartridge ROM space", address);
    }
}
