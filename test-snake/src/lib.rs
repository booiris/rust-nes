mod utils;

use color::{consts::*, Rgb};
use log::debug;
use rand::Rng;
use rust_nes::{cpu::CPU, PPU::ppu::PPU};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Window {
    width: u32,
    height: u32,
    screen: Vec<u8>,
    cpu: CPU,
}

impl Window {
    fn read_screen_state(&mut self) -> bool {
        let frame = &mut self.screen;
        let mut frame_idx = 0;
        let mut update = false;
        for i in 0x0200..0x600 {
            let mut i = i;
            let color_idx = self.cpu.mem.loadb(&mut i);
            let color = color(color_idx);
            if frame[frame_idx] != color.r
                || frame[frame_idx + 1] != color.g
                || frame[frame_idx + 2] != color.b
            {
                frame[frame_idx] = color.r;
                frame[frame_idx + 1] = color.g;
                frame[frame_idx + 2] = color.b;
                update = true;
            }
            frame_idx += 3;
        }
        update
    }
}

#[wasm_bindgen]
impl Window {
    pub fn new(data: &[u8]) -> Window {
        utils::set_panic_hook();
        env_logger::init();
        let data: Vec<u8> = data.into();
        debug!("{:X?}", &data[0..16]);
        let mut cpu = CPU::new();
        cpu.load_rom(data.clone());
        cpu.reset();
        let mut ppu = PPU::new();
        ppu.load_rom(data);
        let mut rng = rand::thread_rng();
        let data = rng.gen_range(1, 16);
        gloo_console::log!("data: ", data);
        cpu.mem.storeb(0xfe, rng.gen_range(1, 200));

        let width = 256;
        let height = 240;

        Window {
            width,
            height,
            screen: vec![0; (width * height * 3) as usize],
            cpu,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn screen(&self) -> *const u8 {
        self.screen.as_ptr()
    }

    pub fn tick(&mut self) -> bool {
        self.cpu.clock();
        self.read_screen_state()
    }
}

fn color(byte: u8) -> Rgb {
    match byte {
        0 => BLACK,
        1 => WHITE,
        2 | 9 => GRAY,
        3 | 10 => RED,
        4 | 11 => GREEN,
        5 | 12 => BLUE,
        6 | 13 => MAGENTA,
        7 | 14 => YELLOW,
        _ => CYAN,
    }
}
