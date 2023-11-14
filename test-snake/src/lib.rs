mod utils;

use std::sync::atomic::AtomicU8;

use color::{consts::*, Rgb};
use rand::{rngs::ThreadRng, Rng};
use rust_nes::cpu::CPU;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};

#[allow(unused_macros)]
macro_rules! wasmLog {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen]
pub struct BackEndTest {
    width: u32,
    height: u32,
    screen: Vec<u8>,
    cpu: CPU,
    tick: u32,
    rng: ThreadRng,
}

static ACTION: AtomicU8 = AtomicU8::new(0);

impl BackEndTest {
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

    fn handle_user_input(&mut self) {
        match ACTION.load(std::sync::atomic::Ordering::SeqCst) {
            1 => {
                self.cpu.mem.storeb(0xff, 0x77);
            }
            2 => {
                self.cpu.mem.storeb(0xff, 0x73);
            }
            3 => {
                self.cpu.mem.storeb(0xff, 0x61);
            }
            4 => {
                self.cpu.mem.storeb(0xff, 0x64);
            }
            _ => {}
        }
    }
}

#[wasm_bindgen]
impl BackEndTest {
    pub fn new(data: &[u8]) -> BackEndTest {
        utils::set_panic_hook();
        let data: Vec<u8> = data.into();
        let mut cpu = CPU::new();
        cpu.load_rom(data.clone());
        cpu.reset();
        let mut rng = rand::thread_rng();
        cpu.mem.storeb(0xfe, rng.gen_range(1, 16));

        add_key_board_listener();

        let width = 32;
        let height = 32;
        BackEndTest {
            width,
            height,
            screen: vec![0; (width * height * 3) as usize],
            cpu,
            tick: 0,
            rng,
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

    pub fn run(&mut self) {
        loop {
            self.cpu.mem.storeb(0xfe, self.rng.gen_range(1, 16));
            self.tick = self.tick.wrapping_add(1);
            self.handle_user_input();
            self.cpu.clock();
            if self.read_screen_state() {
                return;
            }
        }
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

fn add_key_board_listener() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let call_back = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let key = event.key();
        match key.as_str() {
            "w" => {
                ACTION.store(1, std::sync::atomic::Ordering::SeqCst);
            }
            "s" => {
                ACTION.store(2, std::sync::atomic::Ordering::SeqCst);
            }
            "a" => {
                ACTION.store(3, std::sync::atomic::Ordering::SeqCst);
            }
            "d" => {
                ACTION.store(4, std::sync::atomic::Ordering::SeqCst);
            }
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_callback("keydown", call_back.as_ref().unchecked_ref())
        .unwrap();
    call_back.forget();
}
