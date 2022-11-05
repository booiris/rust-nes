use log::debug;

use crate::const_v::{NES_TAG, PRG_ROM_PAGE_SIZE, CHR_ROM_PAGE_SIZE};

use self::mapper0::Mapper0;

pub mod mapper0;

pub trait Mapper {
    fn read(&self, rom: &ROM, address: u16) -> u8;
    fn write(&mut self, rom: &mut Vec<u8>, address: u16, value: u8);
}

// #[allow(non_camel_case_types)]
// enum Mirror {
//     HORIZONTAL,
//     VERTICAL,
//     FOUR_SCREEN,
//     SINGLE_SCREEN_LOWER_BANK,
//     SINGLE_SCREEN_UPPER_BANK,
// }

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOUR_SCREEN,
}

pub struct Header {
    prg_rom_start: usize,
    prg_rom_size: usize,
    chr_rom_start: usize,
    chr_rom_size: usize,
    ram_size: usize,
    mapper: u8,
    screen_mirroring: Mirroring,
}

pub struct ROM {
    prg: Vec<u8>,
    chr: Vec<u8>,
    ram: Vec<u8>,
    mapper: Box<dyn Mapper + 'static>,
    #[allow(dead_code)]
    screen_mirroring: Mirroring,
}

impl ROM {
    pub fn new(data: Vec<u8>) -> Self {
        let header = parse_header(&data);
        let mapper = match header.mapper {
            0 => Box::new(Mapper0 {}),
            _ => panic!("Unknown mapper"),
        };

        debug!(
            "base rom size: {}; load rom size {}",
            data.len(),
            header.chr_rom_start + header.chr_rom_size
        );

        ROM {
            prg: data[header.prg_rom_start..(header.prg_rom_start + header.prg_rom_size)].to_vec(),
            chr: data[header.chr_rom_start..(header.chr_rom_start + header.chr_rom_size)].to_vec(),
            ram: vec![0; header.ram_size],
            mapper,
            screen_mirroring: header.screen_mirroring,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mapper.read(self, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mapper.write(&mut self.ram, address, value);
    }
}

fn parse_header(data: &Vec<u8>) -> Header {
    let header = &data[0..16];
    if header[0..4] != NES_TAG {
        panic!("File is not in iNES file format");
    }
    let mapper = (header[7] & 0b1111_0000) | (header[6] >> 4);

    let ines_ver = (header[7] >> 2) & 0b11;
    if ines_ver != 0 {
        panic!("NES2.0 format is not supported");
    }

    let four_screen = header[6] & 0b1000 != 0;
    let vertical_mirroring = header[6] & 0b1 != 0;
    let screen_mirroring = match (four_screen, vertical_mirroring) {
        (true, _) => Mirroring::FOUR_SCREEN,
        (false, true) => Mirroring::VERTICAL,
        (false, false) => Mirroring::HORIZONTAL,
    };

    let prg_rom_size = header[4] as usize * PRG_ROM_PAGE_SIZE;
    let chr_rom_size = header[5] as usize * CHR_ROM_PAGE_SIZE;

    let skip_trainer = header[6] & 0b100 != 0;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    let chr_rom_start = prg_rom_start + prg_rom_size;

    let prg_ram_size = header[8] as usize;

    Header {
        prg_rom_start,
        prg_rom_size,
        chr_rom_start,
        chr_rom_size,
        ram_size: prg_ram_size,
        mapper,
        screen_mirroring,
    }
}
