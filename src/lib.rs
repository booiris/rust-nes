#[allow(non_snake_case)]
pub mod ROM;
mod bus;
#[allow(non_snake_case)]
pub mod consts;
pub mod cpu;
mod memory;
pub mod ppu_impl;
mod register;
mod utils;

use std::sync::mpsc::{Receiver, Sender};

use cpu::CPU;
use ppu_impl::ppu::PPU;
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
    width: usize,
    height: usize,
    screen: Vec<u8>,
    cpu: CPU,
    ppu: PPU,
    action_receiver: Receiver<u8>,
}

impl BackEnd {
    fn handle_user_input(&mut self) {
        while let Ok(action) = self.action_receiver.try_recv() {
            match action {
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

    fn construct(data: Vec<u8>) -> Self {
        let mut cpu = CPU::new();
        cpu.load_rom(data.clone());
        cpu.reset();
        let mut ppu = PPU::new();
        ppu.load_rom(data);

        let (action_sender, action_receiver) = std::sync::mpsc::channel();
        add_key_board_listener(action_sender);

        let width = consts::WIDTH;
        let height = consts::HEIGHT;
        BackEnd {
            width,
            height,
            screen: vec![0; width * height * 3],
            cpu,
            ppu,
            action_receiver,
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
        let data: Vec<u8> = vec![];
        BackEnd::construct(data)
    }

    pub fn new_with_data(data: &[u8]) -> Self {
        utils::set_panic_hook();
        let data: Vec<u8> = data.into();
        BackEnd::construct(data)
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn screen(&self) -> *const u8 {
        self.screen.as_ptr()
    }

    pub fn run(&mut self) {
        loop {
            self.handle_user_input();
            self.cpu.clock();
        }
    }
}

fn add_key_board_listener(sender: Sender<u8>) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let call_back = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let key = event.key();
        let action = match key.as_str() {
            "w" => 1,
            "s" => 2,
            "a" => 3,
            "d" => 4,
            _ => 0,
        };
        sender.send(action).unwrap();
    }) as Box<dyn FnMut(_)>);

    document
        .add_event_listener_with_callback("keydown", call_back.as_ref().unchecked_ref())
        .unwrap();
    call_back.forget();
}
