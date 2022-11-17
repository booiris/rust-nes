use rust_nes::{frame::show_tile, ppu::PPU};
use sdl2::pixels::PixelFormatEnum;

#[test]
fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let data: Vec<u8> = std::fs::read("./tests/pacman.nes").unwrap();
    let mut ppu = PPU::new();
    ppu.load_rom(data.clone());
    let tile_frame = show_tile(&ppu.mem.rom.unwrap().chr.unwrap(), 1, 0);

    texture.update(None, &tile_frame.data, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop {}
}
