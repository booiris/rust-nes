#![allow(dead_code)]

use core::time;
use std::thread;

use crate::{
    const_v::STACK_BASE,
    memory::CpuMemory,
    register::{Register, RegisterWork},
    ROM::ROM,
};

#[derive(Debug)]
enum InstructionTypes {
    INV,
    BRK,
    JMP,
    LDX,
    STX,
    JSR,
}

#[derive(Debug)]
enum AddressingModes {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

struct Operation {
    instruction_type: InstructionTypes,
    cycle: u8,
    addressing_mode: AddressingModes,
    opc: u8,
}

pub struct CPU {
    program_counter: Register<u16>,
    register_a: Register<u8>,
    register_x: Register<u8>,
    register_y: Register<u8>,
    register_sp: Register<u8>,
    register_p: Register<u8>,

    mem: CpuMemory,
    defer_cycles: usize,
    now_cycles: usize,
}

impl CPU {
    pub fn new(rom: ROM) -> Self {
        CPU {
            program_counter: Register::<u16>::new(),
            register_a: Register::<u8>::new(),
            register_x: Register::<u8>::new(),
            register_y: Register::<u8>::new(),
            register_sp: Register::<u8>::new_with_data(0xfd),
            register_p: Register::<u8>::new(),
            mem: CpuMemory::new(rom),
            defer_cycles: 0,
            now_cycles: 0,
        }
    }
}

impl CPU {
    pub fn run(&mut self) {
        //TODO
        self.program_counter.set_data(0xC000);
        self.now_cycles = 6;

        loop {
            thread::sleep(time::Duration::from_millis(100));
            self.clock();
        }
    }

    fn clock(&mut self) {
        self.now_cycles += 1;
        if self.defer_cycles > 0 {
            self.defer_cycles -= 1;
        }
        if self.defer_cycles == 0 {
            self.step();
        }
    }

    fn step(&mut self) {
        print!("{:04X}  ", self.program_counter.data());
        let op_code = self.mem.read_byte(self.program_counter.data());
        self.program_counter += 1;

        let op = operation(op_code);
        self.exec(op);
        print!("         ");
        print!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            self.register_a.data(),
            self.register_x.data(),
            self.register_y.data(),
            self.register_p.data(),
            self.register_sp.data(),
            self.now_cycles
        );
        print!("\n");
    }
}

impl CPU {
    fn exec(&mut self, op: Operation) {
        self.defer_cycles += op.cycle as usize;
        match op.instruction_type {
            InstructionTypes::BRK => self.brk(op),
            InstructionTypes::JMP => self.jmp(op),
            InstructionTypes::LDX => self.ldx(op),
            InstructionTypes::STX => self.stx(op),
            InstructionTypes::JSR => self.jsr(op),
            _ => panic!("{:?} Operation not implement!", op.instruction_type),
        }
    }

    fn debug(&self, op: Operation, op_address: String, op_value: Option<String>) {
        print!("{:02x} ", op.opc);
        print!(" {:?} ", op.instruction_type);
        match op.addressing_mode {
            AddressingModes::Immediate => print!("#"),
            AddressingModes::Absolute => print!("$"),
            AddressingModes::ZeroPage => print!("$"),
            _ => print!("?"),
        }
        print!("{}", op_address);
        if let Some(v) = op_value {
            print!(" = {}", v);
        }
    }
}

impl CPU {
    fn brk(&self, _op: Operation) {}

    fn jmp(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Absolute => {
                let address = self.mem.read_word(self.program_counter.data());
                self.program_counter.set_data(address);
                self.debug(op, format!("{:04X}", address), None);
            }
            _ => {
                panic!("{:?} error AddressingModes!", op.addressing_mode)
            }
        }
    }

    fn ldx(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Immediate => {
                let data = self.mem.read_byte(self.program_counter.data());
                self.program_counter += 1;
                self.register_x.set_data(data);
                self.debug(op, format!("{:02X}", data), None);
            }
            _ => {
                panic!("{:?} error AddressingModes!", op.addressing_mode)
            }
        }
    }

    fn stx(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::ZeroPage => {
                let address = self.mem.read_byte(self.program_counter.data());
                self.program_counter += 1;
                self.mem.write_byte(address as u16, self.register_x.data());
                self.debug(
                    op,
                    format!("{:02X}", address),
                    Some(format!("{:02X}", self.register_x.data())),
                );
            }
            _ => {
                panic!("{:?} error AddressingModes!", op.addressing_mode)
            }
        }
    }

    fn jsr(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Absolute => {
                let address = self.mem.read_word(self.program_counter.data());
                self.program_counter += 2;
                self.mem.write_word(
                    STACK_BASE + self.register_sp.data() as u16,
                    self.program_counter.data(),
                );
                self.register_sp -= 2;
                self.program_counter.set_data(address);
                self.debug(op, format!("{:02X}", address), None);
            }
            _ => {
                panic!("{:?} error AddressingModes!", op.addressing_mode)
            }
        }
    }
}

fn operation(opc: u8) -> Operation {
    match opc {
        0x00 => Operation {
            instruction_type: InstructionTypes::BRK,
            cycle: 7,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x20 => Operation {
            instruction_type: InstructionTypes::JSR,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x4c => Operation {
            instruction_type: InstructionTypes::JMP,
            cycle: 3,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x86 => Operation {
            instruction_type: InstructionTypes::STX,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xa2 => Operation {
            instruction_type: InstructionTypes::LDX,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        _ => panic!("{:x?} Operation Code not implement!", opc),
    }
}
