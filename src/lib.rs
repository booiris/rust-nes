#[allow(non_snake_case)]
pub mod CONST;
#[allow(non_snake_case)]
pub mod ROM;
mod bus;
pub mod cpu;
mod memory;
pub mod ppu_impl;
mod register;
mod utils;

use std::sync::atomic::AtomicU8;

use cpu::CPU;
use ppu_impl::ppu::PPU;
use rand::{rngs::ThreadRng, Rng};
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
pub struct BackEnd {
    width: u32,
    height: u32,
    screen: Vec<u8>,
    cpu: CPU,
    ppu: PPU,
    tick: u32,
    rng: ThreadRng,
}

static ACTION: AtomicU8 = AtomicU8::new(0);

impl BackEnd {
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

    fn construct(data: Vec<u8>) -> Self {
        let mut cpu = CPU::new();
        cpu.load_rom(data.clone());
        cpu.reset();
        let mut ppu = PPU::new();
        ppu.load_rom(data);
        let mut rng = rand::thread_rng();
        cpu.mem.storeb(0xfe, rng.gen_range(1, 16));

        add_key_board_listener();

        let width = 32;
        let height = 32;
        BackEnd {
            width,
            height,
            screen: vec![0; (width * height * 3) as usize],
            cpu,
            ppu,
            tick: 0,
            rng,
        }
    }
}

impl Default for BackEnd {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl BackEnd {
    pub fn new() -> Self {
        utils::set_panic_hook();
        wasmLog!("asdfasdfasdf");
        let data: Vec<u8> = vec![];
        BackEnd::construct(data)
    }

    pub fn new_with_data(data: &[u8]) -> Self {
        utils::set_panic_hook();
        let data: Vec<u8> = data.into();
        BackEnd::construct(data)
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
        }
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
