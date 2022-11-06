use super::{Mapper, ROM};

#[allow(dead_code)]
pub struct Mapper0 {}

impl Mapper for Mapper0 {
    fn read(&self, rom: &ROM, address: &mut u16) -> u8 {
        if *address < 0x2000 {
            rom.chr[*address as usize]
        } else if *address >= 0x8000 {
            let mut address = *address - 0x8000;
            if rom.prg.len() == 0x4000 && address >= 0x4000 {
                address -= 0x4000;
            }
            rom.prg[address as usize]
        } else {
            panic!("{:x?} address out of range!", address)
        }
    }

    fn write(&mut self, _ram: &mut Vec<u8>, _address: u16, _value: u8) {
        panic!("Attempt to write to Cartridge ROM space")
    }
}
