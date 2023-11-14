use rust_nes::consts::{HEIGHT, SYSTEM_PALLETE, WIDTH};
use rust_nes::ppu_impl::ppu::{Frame, PPU};

#[test]
fn main() {
    let data: Vec<u8> = std::fs::read("./tests/pacman.nes").unwrap();
    let mut ppu = PPU::new();
    ppu.load_rom(data.clone());
    let tile_frame = show_tile(&ppu.mem.rom.unwrap().chr.unwrap(), 0);
}

pub fn show_tile(chr_rom: &Vec<u8>, bank: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new(WIDTH, HEIGHT);
    let mut tile_y = 0;
    let mut tile_x = 0;
    let bank = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

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
                frame.set_pixel(tile_x + x, tile_y + y, rgb)
            }
        }

        tile_x += 10;
    }
    frame
}
