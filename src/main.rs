use log::debug;
use rust_nes::cpu::*;
use rust_nes::ppu::Frame;
use rust_nes::ppu::PPU;
use rust_nes::CONST::HEIGHT;
use rust_nes::CONST::WIDTH;
use std::fs;
use std::thread;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

const SCALE: usize = 3;

fn main() {
    env_logger::init();
    let data = fs::read("./tests/pacman.nes").expect("can't read file");
    debug!("{:X?}", &data[0..16]);
    let mut cpu = CPU::new();
    cpu.load_rom(data.clone());
    cpu.reset();
    let mut ppu = PPU::new();
    ppu.load_rom(data);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Snake game",
            (WIDTH as f32 * SCALE as f32) as u32,
            (HEIGHT as f32 * SCALE as f32) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut frame = Frame::new(WIDTH, HEIGHT);

    // thread::spawn(move || {
    //     handle_user_input(&mut cpu, &mut event_pump);
    //     // cpu.run_with_callback(move |cpu| {

    //     //     ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    //     // });
    // });

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                _ => { /* do nothing */ }
            }
        }
        ppu.render(&mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}

// fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
//     for event in event_pump.poll_iter() {
//         match event {
//             Event::Quit { .. }
//             | Event::KeyDown {
//                 keycode: Some(Keycode::Escape),
//                 ..
//             } => std::process::exit(0),
//             Event::KeyDown {
//                 keycode: Some(Keycode::W),
//                 ..
//             } => {
//                 cpu.mem.storeb(0xff, 0x77);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::S),
//                 ..
//             } => {
//                 cpu.mem.storeb(0xff, 0x73);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::A),
//                 ..
//             } => {
//                 cpu.mem.storeb(0xff, 0x61);
//             }
//             Event::KeyDown {
//                 keycode: Some(Keycode::D),
//                 ..
//             } => {
//                 cpu.mem.storeb(0xff, 0x64);
//             }
//             _ => { /* do nothing */ }
//         }
//     }
// }

// fn color(byte: u8) -> Color {
//     match byte {
//         0 => sdl2::pixels::Color::BLACK,
//         1 => sdl2::pixels::Color::WHITE,
//         2 | 9 => sdl2::pixels::Color::GREY,
//         3 | 10 => sdl2::pixels::Color::RED,
//         4 | 11 => sdl2::pixels::Color::GREEN,
//         5 | 12 => sdl2::pixels::Color::BLUE,
//         6 | 13 => sdl2::pixels::Color::MAGENTA,
//         7 | 14 => sdl2::pixels::Color::YELLOW,
//         _ => sdl2::pixels::Color::CYAN,
//     }
// }

// fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
//     let mut frame_idx = 0;
//     let mut update = false;
//     for i in 0x0200..0x600 {
//         let mut i = i;
//         let color_idx = cpu.mem.loadb(&mut i);
//         let (b1, b2, b3) = color(color_idx).rgb();
//         if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
//             frame[frame_idx] = b1;
//             frame[frame_idx + 1] = b2;
//             frame[frame_idx + 2] = b3;
//             update = true;
//         }
//         frame_idx += 3;
//     }
//     update
// }
