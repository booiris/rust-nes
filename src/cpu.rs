#![allow(dead_code)]

// use core::time;
// use std::thread;

use crate::{
    memory::CpuMemory,
    register::{Flags, Register, RegisterWork},
    ROM::ROM,
};

#[derive(Debug)]
enum InstructionTypes {
    BRK,
    JMP,
    LDX,
    STX,
    JSR,
    NOP,
    SEC,
    BCS,
    CLC,
    BCC,
    LDA,
    BEQ,
    BNE,
    STA,
    BIT,
    BVS,
    BVC,
    BPL,
    RTS,
    SEI,
    SED,
    PHP,
    PLA,
    AND,
    CMP,
    CLD,
    PHA,
    PLP,
    BMI,
    ORA,
    CLV,
    EOR,
    ADC,
    LDY,
    CPY,
    CPX,
    SBC,
    INY,
    INX,
    DEY,
    DEX,
    TAY,
    TAX,
    TYA,
    TXA,
    TSX,
    TXS,
    RTI,
    LSR,
    ASL,
    ROR,
    ROL,
    STY,
    INC,
    DEC,
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
    IndirectX,
    IndirectY,
    Empty,
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
            register_p: Register::<u8>::new_with_data(0x24),
            mem: CpuMemory::new(rom),
            defer_cycles: 0,
            now_cycles: 0,
        }
    }
}

impl CPU {
    fn get_data(&mut self, op: &Operation) -> u8 {
        match op.addressing_mode {
            AddressingModes::Accumulator => self.register_a.data(),
            AddressingModes::Immediate => self.mem.loadb(self.program_counter.mut_data()),
            AddressingModes::Relative => self.mem.loadb(self.program_counter.mut_data()),
            AddressingModes::Absolute => {
                let mut addr = self.mem.loadw(self.program_counter.mut_data());
                self.mem.loadb(&mut addr)
            }
            AddressingModes::ZeroPage => {
                let mut addr = self.mem.loadb(self.program_counter.mut_data()) as u16;
                self.mem.loadb(&mut addr)
            }
            AddressingModes::IndirectX => {
                let mut addr = self.get_addr(op);
                self.mem.loadb(&mut addr)
            }
            AddressingModes::IndirectY => {
                let mut addr = self.get_addr(op);
                self.mem.loadb(&mut addr)
            }
            _ => panic!(
                "get data: {:?} not invalid address mode",
                op.addressing_mode
            ),
        }
    }

    fn get_addr(&mut self, op: &Operation) -> u16 {
        match op.addressing_mode {
            AddressingModes::Accumulator => 0,
            AddressingModes::ZeroPage => self.mem.loadb(self.program_counter.mut_data()) as u16,
            AddressingModes::Absolute => self.mem.loadw(self.program_counter.mut_data()),
            AddressingModes::IndirectX => {
                let mut addr1 = self
                    .mem
                    .loadb(self.program_counter.mut_data())
                    .wrapping_add(self.register_x.data()) as u16;
                let mut addr2 = (addr1 & 0xFF00) | ((addr1 + 1) & 0x00FF);
                self.mem.loadb(&mut addr1) as u16 | (self.mem.loadb(&mut addr2) as u16) << 8
            }
            AddressingModes::IndirectY => {
                let mut addr1 = self.mem.loadb(self.program_counter.mut_data()) as u16;
                let mut addr2 = (addr1 & 0xFF00) | ((addr1 + 1) & 0x00FF);
                let addr =
                    self.mem.loadb(&mut addr1) as u16 | (self.mem.loadb(&mut addr2) as u16) << 8;
                let old_addr = addr;
                let addr = addr.wrapping_add(self.register_y.data() as u16) as u16;
                if self.new_page(old_addr, addr) {
                    self.defer_cycles += 1;
                }
                addr
            }
            AddressingModes::Indirect => {
                let mut addr1 = self.mem.loadw(self.program_counter.mut_data());
                let mut addr2 = (addr1 & 0xFF00) | ((addr1 + 1) & 0x00FF);
                self.mem.loadb(&mut addr1) as u16 | (self.mem.loadb(&mut addr2) as u16) << 8
            }
            _ => panic!(
                "get addr: {:?} not invalid address mode",
                op.addressing_mode
            ),
        }
    }
}

impl CPU {
    pub fn run(&mut self) {
        //TODO
        self.program_counter.set_data(0xC000);
        self.now_cycles = 6;

        loop {
            // thread::sleep(time::Duration::from_millis(100));
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
        let op_code = self.mem.loadb(self.program_counter.mut_data());

        let op = operation(op_code);
        self.exec(op);
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
            InstructionTypes::NOP => self.nop(op),
            InstructionTypes::SEC => self.sec(op),
            InstructionTypes::BCS => self.bcs(op),
            InstructionTypes::CLC => self.clc(op),
            InstructionTypes::BCC => self.bcc(op),
            InstructionTypes::LDA => self.lda(op),
            InstructionTypes::BEQ => self.beq(op),
            InstructionTypes::BNE => self.bne(op),
            InstructionTypes::STA => self.sta(op),
            InstructionTypes::BIT => self.bit(op),
            InstructionTypes::BVS => self.bvs(op),
            InstructionTypes::BVC => self.bvc(op),
            InstructionTypes::BPL => self.bpl(op),
            InstructionTypes::RTS => self.rts(op),
            InstructionTypes::SEI => self.sei(op),
            InstructionTypes::SED => self.sed(op),
            InstructionTypes::PHP => self.php(op),
            InstructionTypes::PLA => self.pla(op),
            InstructionTypes::AND => self.and(op),
            InstructionTypes::CMP => self.cmp(op),
            InstructionTypes::CLD => self.cld(op),
            InstructionTypes::PHA => self.pha(op),
            InstructionTypes::PLP => self.plp(op),
            InstructionTypes::BMI => self.bmi(op),
            InstructionTypes::ORA => self.ora(op),
            InstructionTypes::CLV => self.clv(op),
            InstructionTypes::EOR => self.eor(op),
            InstructionTypes::ADC => self.adc(op),
            InstructionTypes::LDY => self.ldy(op),
            InstructionTypes::CPY => self.cpy(op),
            InstructionTypes::CPX => self.cpx(op),
            InstructionTypes::SBC => self.sbc(op),
            InstructionTypes::INY => self.iny(op),
            InstructionTypes::INX => self.inx(op),
            InstructionTypes::DEY => self.dey(op),
            InstructionTypes::DEX => self.dex(op),
            InstructionTypes::TAY => self.tay(op),
            InstructionTypes::TAX => self.tax(op),
            InstructionTypes::TYA => self.tya(op),
            InstructionTypes::TXA => self.txa(op),
            InstructionTypes::TSX => self.tsx(op),
            InstructionTypes::TXS => self.txs(op),
            InstructionTypes::RTI => self.rti(op),
            InstructionTypes::LSR => self.lsr(op),
            InstructionTypes::ASL => self.asl(op),
            InstructionTypes::ROR => self.ror(op),
            InstructionTypes::ROL => self.rol(op),
            InstructionTypes::STY => self.sty(op),
            InstructionTypes::INC => self.inc(op),
            InstructionTypes::DEC => self.dec(op),
        }
    }

    fn debug(&self, op: &Operation, op_address: String, op_value: Option<String>) {
        print!("{:02X} ", op.opc);
        print!(" {:?} ", op.instruction_type);
        match op.addressing_mode {
            AddressingModes::Immediate => print!("#"),
            AddressingModes::Absolute => print!("$"),
            AddressingModes::ZeroPage => print!("$"),
            AddressingModes::Relative => print!("$"),
            _ => print!("?"),
        }
        print!("{}", op_address);
        if let Some(v) = op_value {
            print!(" = {}", v);
        }
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
    fn set_zn(&mut self, val: u8) {
        self.register_p.set_flag(Flags::Z, val == 0);
        self.register_p.set_flag(Flags::N, (val & 0x80) != 0);
    }

    fn new_page(&self, old_addr: u16, new_addr: u16) -> bool {
        (new_addr & 0xFF00) != (old_addr & 0xFF00)
    }

    fn jmp_by_flag(&mut self, op: &Operation, flag: Flags, on: bool) {
        let data = self.get_data(&op);
        if self.register_p.check_flag(flag) == on {
            self.defer_cycles += 1;
            let old_addr = self.program_counter.data();
            if (data as i8) < 0 {
                self.program_counter -= data as u16;
            } else {
                self.program_counter += data as u16;
            }
            let new_addr = self.program_counter.data();
            if self.new_page(old_addr, new_addr) {
                self.defer_cycles += 1;
            }
        }
        self.debug(
            op,
            format!("{:04X}", self.program_counter.data()),
            Some(format!("{:02X}", data)),
        );
    }

    fn brk(&self, _op: Operation) {}

    fn jmp(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        self.program_counter.set_data(addr);
        self.debug(&op, format!("{:04X}", addr), None);
    }

    fn ldx(&mut self, op: Operation) {
        let data = self.get_data(&op);
        self.register_x.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn stx(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        self.mem.storeb(addr, self.register_x.data());
        self.debug(
            &op,
            format!("{:02X}", addr),
            Some(format!("{:02X}", self.register_x.data())),
        );
    }

    fn jsr(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        self.register_sp
            .stack_push_word(&mut self.mem, self.program_counter.data() - 1);
        self.program_counter.set_data(addr);
        self.debug(&op, format!("{:02X}", addr), None);
    }

    fn nop(&mut self, op: Operation) {
        self.debug(&op, format!("{:02X}", 0x00), None);
    }

    fn sec(&mut self, op: Operation) {
        self.register_p.set_flag(Flags::C, true);
        self.debug(&op, format!("{:02X}", 0x00), None);
    }

    fn bcs(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::C, true);
    }

    fn clc(&mut self, op: Operation) {
        self.register_p.set_flag(Flags::C, false);
        self.debug(&op, format!("{:02X}", 0x00), None);
    }

    fn bcc(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::C, false);
    }

    fn lda(&mut self, op: Operation) {
        let data = self.get_data(&op);
        self.register_a.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn beq(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::Z, true);
    }

    fn bne(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::Z, false);
    }

    fn sta(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        self.mem.storeb(addr, self.register_a.data());
        self.debug(
            &op,
            format!("{:02X}", addr),
            Some(format!("{:02X}", self.register_a.data())),
        );
    }

    fn bit(&mut self, op: Operation) {
        let mut addr = self.get_addr(&op);
        let data = self.mem.loadb(&mut addr);
        let temp = data & self.register_a.data();
        self.register_p.set_flag(Flags::Z, temp == 0);
        self.register_p
            .set_flag(Flags::N, data & Flags::N as u8 != 0);
        self.register_p
            .set_flag(Flags::V, data & Flags::V as u8 != 0);
        self.debug(&op, format!("{:02X}", addr), Some(format!("{:02X}", data)));
    }

    fn bvs(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::V, true);
    }

    fn bvc(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::V, false);
    }

    fn bpl(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::N, false);
    }

    fn rts(&mut self, op: Operation) {
        let addr = self.register_sp.stack_pop_word(&mut self.mem) + 1;
        self.program_counter.set_data(addr);
        self.debug(&op, format!("{:04X}", addr), None);
    }

    fn sei(&mut self, op: Operation) {
        self.register_p.set_flag(Flags::I, true);
        self.debug(&op, format!("{:02X}", 0), None);
    }

    fn sed(&mut self, op: Operation) {
        self.register_p.set_flag(Flags::D, true);
        self.debug(&op, format!("{:02X}", 0), None);
    }

    fn php(&mut self, op: Operation) {
        let data = self.register_p.data() | 0x30;
        self.register_sp.stack_push_byte(&mut self.mem, data);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn pla(&mut self, op: Operation) {
        let data = self.register_sp.stack_pop_byte(&mut self.mem);
        self.register_a.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn and(&mut self, op: Operation) {
        let data = self.get_data(&op);
        *self.register_a.mut_data() &= data;
        self.set_zn(self.register_a.data());
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn cmp(&mut self, op: Operation) {
        let data = self.register_a.data() as i32 - self.get_data(&op) as i32;
        self.register_p.set_flag(Flags::C, data >= 0);
        self.set_zn(data as u8);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn cld(&mut self, op: Operation) {
        self.register_p.set_flag(Flags::D, false);
        self.debug(&op, format!("{:02X}", 0), None);
    }

    fn pha(&mut self, op: Operation) {
        self.register_sp
            .stack_push_byte(&mut self.mem, self.register_a.data());
        self.debug(&op, format!("{:02X}", self.register_a.data()), None);
    }

    fn plp(&mut self, op: Operation) {
        let data = self.register_sp.stack_pop_byte(&mut self.mem);
        self.register_p.set_data((data | 0x30) - 0x10);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn bmi(&mut self, op: Operation) {
        self.jmp_by_flag(&op, Flags::N, true);
    }

    fn ora(&mut self, op: Operation) {
        let data = self.get_data(&op);
        *self.register_a.mut_data() |= data;
        self.set_zn(self.register_a.data());
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn clv(&mut self, op: Operation) {
        self.register_p.set_flag(Flags::V, false);
        self.debug(&op, format!("{:02X}", 0), None)
    }

    fn eor(&mut self, op: Operation) {
        let data = self.get_data(&op);
        *self.register_a.mut_data() ^= data;
        self.set_zn(self.register_a.data());
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn adc(&mut self, op: Operation) {
        let src1 = self.get_data(&op) as u16;
        let src2 = self.register_a.data() as u16;
        let res = src1
            .wrapping_add(src2)
            .wrapping_add(self.register_p.check_flag(Flags::C) as u16);
        self.register_p.set_flag(Flags::C, res & 0x100 != 0);
        let res = res as u8;
        let flag = (((src1 as u8 ^ src2 as u8) & 0x80) == 0) && ((src1 as u8 ^ res) & 0x80 != 0);
        self.register_p.set_flag(Flags::V, flag);
        self.set_zn(res);
        self.register_a.set_data(res);
        self.debug(&op, format!("{:02X}", src1), Some(format!("{:02X}", res)));
    }

    fn ldy(&mut self, op: Operation) {
        let data = self.get_data(&op);
        self.register_y.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn cpy(&mut self, op: Operation) {
        let data = self.register_y.data() as i32 - self.get_data(&op) as i32;
        self.register_p.set_flag(Flags::C, data >= 0);
        self.set_zn(data as u8);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn cpx(&mut self, op: Operation) {
        let data = self.register_x.data() as i32 - self.get_data(&op) as i32;
        self.register_p.set_flag(Flags::C, data >= 0);
        self.set_zn(data as u8);
        self.debug(&op, format!("{:02X}", data), None);
    }

    fn sbc(&mut self, op: Operation) {
        let src1 = self.register_a.data() as u16;
        let src2 = self.get_data(&op) as u16;
        let res = src1
            .wrapping_sub(src2)
            .wrapping_sub(1 - self.register_p.check_flag(Flags::C) as u16);
        self.register_p.set_flag(Flags::C, res & 0x100 == 0);
        let res = res as u8;
        let flag = (((src1 as u8 ^ src2 as u8) & 0x80) != 0) && ((src1 as u8 ^ res) & 0x80 != 0);
        self.register_p.set_flag(Flags::V, flag);
        self.set_zn(res);
        self.register_a.set_data(res);
        self.debug(&op, format!("{:02X}", src1), Some(format!("{:02X}", res)));
    }

    fn iny(&mut self, op: Operation) {
        let data = self.register_y.mut_data();
        *data = data.wrapping_add(1);
        self.set_zn(self.register_y.data());
        self.debug(&op, format!("{:02X}", self.register_y.data()), None);
    }

    fn inx(&mut self, op: Operation) {
        let data = self.register_x.mut_data();
        *data = data.wrapping_add(1);
        self.set_zn(self.register_x.data());
        self.debug(&op, format!("{:02X}", self.register_x.data()), None);
    }

    fn dey(&mut self, op: Operation) {
        let data = self.register_y.mut_data();
        *data = data.wrapping_sub(1);
        self.set_zn(self.register_y.data());
        self.debug(&op, format!("{:02X}", self.register_y.data()), None);
    }

    fn dex(&mut self, op: Operation) {
        let data = self.register_x.mut_data();
        *data = data.wrapping_sub(1);
        self.set_zn(self.register_x.data());
        self.debug(&op, format!("{:02X}", self.register_x.data()), None);
    }

    fn tay(&mut self, op: Operation) {
        let data = self.register_a.data();
        self.register_y.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", self.register_y.data()), None);
    }

    fn tax(&mut self, op: Operation) {
        let data = self.register_a.data();
        self.register_x.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", self.register_x.data()), None);
    }

    fn tya(&mut self, op: Operation) {
        let data = self.register_y.data();
        self.register_a.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", self.register_y.data()), None);
    }

    fn txa(&mut self, op: Operation) {
        let data = self.register_x.data();
        self.register_a.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", self.register_x.data()), None);
    }

    fn tsx(&mut self, op: Operation) {
        let data = self.register_sp.data();
        self.register_x.set_data(data);
        self.set_zn(data);
        self.debug(&op, format!("{:02X}", self.register_x.data()), None);
    }

    fn txs(&mut self, op: Operation) {
        let data = self.register_x.data();
        self.register_sp.set_data(data);
        self.debug(&op, format!("{:02X}", self.register_sp.data()), None);
    }

    fn rti(&mut self, op: Operation) {
        let data = self.register_sp.stack_pop_byte(&mut self.mem);
        self.register_p.set_data((data | 0x30) - 0x10);
        let addr = self.register_sp.stack_pop_word(&mut self.mem);
        self.program_counter.set_data(addr);
        self.debug(&op, format!("{:04X}", addr), Some(format!("{:02X}", data)));
    }

    fn lsr(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Accumulator => {
                let data = self.get_data(&op);
                let (data, flag) = (data >> 1, (data & 0b0000_0001) == 0b0000_0001);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.register_a.set_data(data);
                self.debug(&op, format!("{:02X}", data), None);
            }
            _ => {
                let addr = self.get_addr(&op);
                let mut temp = addr;
                let data = self.mem.loadb(&mut temp);
                let (data, flag) = (data >> 1, (data & 0b0000_0001) == 0b0000_0001);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.mem.storeb(addr, data);
                self.debug(&op, format!("{:02X}", data), None);
            }
        }
    }

    fn asl(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Accumulator => {
                let data = self.get_data(&op);
                let (data, flag) = (data << 1, (data & 0b1000_0000) == 0b1000_0000);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.register_a.set_data(data);
                self.debug(&op, format!("{:02X}", data), None);
            }
            _ => {
                let addr = self.get_addr(&op);
                let mut temp = addr;
                let data = self.mem.loadb(&mut temp);
                let (data, flag) = (data << 1, (data & 0b1000_0000) == 0b1000_0000);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.mem.storeb(addr, data);
                self.debug(&op, format!("{:02X}", data), None);
            }
        }
    }

    fn ror(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Accumulator => {
                let data = self.get_data(&op);
                let (data, flag) = (data >> 1, (data & 0b0000_0001) == 0b0000_0001);
                let data = data | ((self.register_p.check_flag(Flags::C) as u8) << 7);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.register_a.set_data(data);
                self.debug(&op, format!("{:02X}", data), None);
            }
            _ => {
                let addr = self.get_addr(&op);
                let mut temp = addr;
                let data = self.mem.loadb(&mut temp);
                let (data, flag) = (data >> 1, (data & 0b0000_0001) == 0b0000_0001);
                let data = data | ((self.register_p.check_flag(Flags::C) as u8) << 7);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.mem.storeb(addr, data);
                self.debug(&op, format!("{:02X}", data), None);
            }
        }
    }

    fn rol(&mut self, op: Operation) {
        match op.addressing_mode {
            AddressingModes::Accumulator => {
                let data = self.get_data(&op);
                let (data, flag) = (data << 1, (data & 0b1000_0000) == 0b1000_0000);
                let data = data | (self.register_p.check_flag(Flags::C) as u8);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.register_a.set_data(data);
                self.debug(&op, format!("{:02X}", data), None);
            }
            _ => {
                let addr = self.get_addr(&op);
                let mut temp = addr;
                let data = self.mem.loadb(&mut temp);
                let (data, flag) = (data << 1, (data & 0b1000_0000) == 0b1000_0000);
                let data = data | (self.register_p.check_flag(Flags::C) as u8);
                self.set_zn(data);
                self.register_p.set_flag(Flags::C, flag);
                self.mem.storeb(addr, data);
                self.debug(&op, format!("{:02X}", data), None);
            }
        }
    }

    fn sty(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        self.mem.storeb(addr, self.register_y.data());
        self.debug(
            &op,
            format!("{:04X}", addr),
            Some(format!("{:02X}", self.register_y.data())),
        );
    }

    fn inc(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        let mut temp = addr;
        let data = self.mem.loadb(&mut temp).wrapping_add(1);
        self.set_zn(data);
        self.mem.storeb(addr, data);
        self.debug(&op, format!("{:04X}", addr), Some(format!("{:02X}", data)));
    }

    fn dec(&mut self, op: Operation) {
        let addr = self.get_addr(&op);
        let mut temp = addr;
        let data = self.mem.loadb(&mut temp).wrapping_sub(1);
        self.set_zn(data);
        self.mem.storeb(addr, data);
        self.debug(&op, format!("{:04X}", addr), Some(format!("{:02X}", data)));
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
        0x01 => Operation {
            instruction_type: InstructionTypes::ORA,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0x05 => Operation {
            instruction_type: InstructionTypes::ORA,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x06 => Operation {
            instruction_type: InstructionTypes::ASL,
            cycle: 5,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x08 => Operation {
            instruction_type: InstructionTypes::PHP,
            cycle: 3,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x09 => Operation {
            instruction_type: InstructionTypes::ORA,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0x0a => Operation {
            instruction_type: InstructionTypes::ASL,
            cycle: 2,
            addressing_mode: AddressingModes::Accumulator,
            opc,
        },
        0x0d => Operation {
            instruction_type: InstructionTypes::ORA,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x0e => Operation {
            instruction_type: InstructionTypes::ASL,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x10 => Operation {
            instruction_type: InstructionTypes::BPL,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0x11 => Operation {
            instruction_type: InstructionTypes::ORA,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0x18 => Operation {
            instruction_type: InstructionTypes::CLC,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x20 => Operation {
            instruction_type: InstructionTypes::JSR,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x21 => Operation {
            instruction_type: InstructionTypes::AND,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0x24 => Operation {
            instruction_type: InstructionTypes::BIT,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x25 => Operation {
            instruction_type: InstructionTypes::AND,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x26 => Operation {
            instruction_type: InstructionTypes::ROL,
            cycle: 5,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x28 => Operation {
            instruction_type: InstructionTypes::PLP,
            cycle: 4,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x29 => Operation {
            instruction_type: InstructionTypes::AND,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0x2a => Operation {
            instruction_type: InstructionTypes::ROL,
            cycle: 2,
            addressing_mode: AddressingModes::Accumulator,
            opc,
        },
        0x2c => Operation {
            instruction_type: InstructionTypes::BIT,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x2d => Operation {
            instruction_type: InstructionTypes::AND,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x2e => Operation {
            instruction_type: InstructionTypes::ROL,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x30 => Operation {
            instruction_type: InstructionTypes::BMI,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0x31 => Operation {
            instruction_type: InstructionTypes::AND,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0x38 => Operation {
            instruction_type: InstructionTypes::SEC,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x40 => Operation {
            instruction_type: InstructionTypes::RTI,
            cycle: 6,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x41 => Operation {
            instruction_type: InstructionTypes::EOR,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0x45 => Operation {
            instruction_type: InstructionTypes::EOR,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x46 => Operation {
            instruction_type: InstructionTypes::LSR,
            cycle: 5,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x48 => Operation {
            instruction_type: InstructionTypes::PHA,
            cycle: 3,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x49 => Operation {
            instruction_type: InstructionTypes::EOR,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0x4a => Operation {
            instruction_type: InstructionTypes::LSR,
            cycle: 2,
            addressing_mode: AddressingModes::Accumulator,
            opc,
        },
        0x4c => Operation {
            instruction_type: InstructionTypes::JMP,
            cycle: 3,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x4d => Operation {
            instruction_type: InstructionTypes::EOR,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x4e => Operation {
            instruction_type: InstructionTypes::LSR,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x50 => Operation {
            instruction_type: InstructionTypes::BVC,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0x51 => Operation {
            instruction_type: InstructionTypes::EOR,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0x60 => Operation {
            instruction_type: InstructionTypes::RTS,
            cycle: 6,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x61 => Operation {
            instruction_type: InstructionTypes::ADC,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0x65 => Operation {
            instruction_type: InstructionTypes::ADC,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x66 => Operation {
            instruction_type: InstructionTypes::ROR,
            cycle: 5,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x68 => Operation {
            instruction_type: InstructionTypes::PLA,
            cycle: 4,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x69 => Operation {
            instruction_type: InstructionTypes::ADC,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0x6a => Operation {
            instruction_type: InstructionTypes::ROR,
            cycle: 2,
            addressing_mode: AddressingModes::Accumulator,
            opc,
        },
        0x6c => Operation {
            instruction_type: InstructionTypes::JMP,
            cycle: 5,
            addressing_mode: AddressingModes::Indirect,
            opc,
        },
        0x6d => Operation {
            instruction_type: InstructionTypes::ADC,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x6e => Operation {
            instruction_type: InstructionTypes::ROR,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x70 => Operation {
            instruction_type: InstructionTypes::BVS,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0x71 => Operation {
            instruction_type: InstructionTypes::ADC,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0x78 => Operation {
            instruction_type: InstructionTypes::SEI,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x81 => Operation {
            instruction_type: InstructionTypes::STA,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0x84 => Operation {
            instruction_type: InstructionTypes::STY,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x85 => Operation {
            instruction_type: InstructionTypes::STA,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x86 => Operation {
            instruction_type: InstructionTypes::STX,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0x88 => Operation {
            instruction_type: InstructionTypes::DEY,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x8a => Operation {
            instruction_type: InstructionTypes::TXA,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x8c => Operation {
            instruction_type: InstructionTypes::STY,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x8d => Operation {
            instruction_type: InstructionTypes::STA,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x8e => Operation {
            instruction_type: InstructionTypes::STX,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0x90 => Operation {
            instruction_type: InstructionTypes::BCC,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0x91 => Operation {
            instruction_type: InstructionTypes::STA,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0x98 => Operation {
            instruction_type: InstructionTypes::TYA,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0x9a => Operation {
            instruction_type: InstructionTypes::TXS,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xa0 => Operation {
            instruction_type: InstructionTypes::LDY,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xa1 => Operation {
            instruction_type: InstructionTypes::LDA,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0xa2 => Operation {
            instruction_type: InstructionTypes::LDX,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xa4 => Operation {
            instruction_type: InstructionTypes::LDY,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xa5 => Operation {
            instruction_type: InstructionTypes::LDA,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xa6 => Operation {
            instruction_type: InstructionTypes::LDX,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xa8 => Operation {
            instruction_type: InstructionTypes::TAY,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xa9 => Operation {
            instruction_type: InstructionTypes::LDA,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xaa => Operation {
            instruction_type: InstructionTypes::TAX,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xac => Operation {
            instruction_type: InstructionTypes::LDY,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xad => Operation {
            instruction_type: InstructionTypes::LDA,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xae => Operation {
            instruction_type: InstructionTypes::LDX,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xb0 => Operation {
            instruction_type: InstructionTypes::BCS,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0xb1 => Operation {
            instruction_type: InstructionTypes::LDA,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0xb8 => Operation {
            instruction_type: InstructionTypes::CLV,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xb9 => Operation {
            instruction_type: InstructionTypes::LDA,
            cycle: 4,
            addressing_mode: AddressingModes::AbsoluteY,
            opc,
        },
        0xba => Operation {
            instruction_type: InstructionTypes::TSX,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xc0 => Operation {
            instruction_type: InstructionTypes::CPY,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xc1 => Operation {
            instruction_type: InstructionTypes::CMP,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0xc4 => Operation {
            instruction_type: InstructionTypes::CPY,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xc5 => Operation {
            instruction_type: InstructionTypes::CMP,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xc6 => Operation {
            instruction_type: InstructionTypes::DEC,
            cycle: 5,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xc8 => Operation {
            instruction_type: InstructionTypes::INY,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xc9 => Operation {
            instruction_type: InstructionTypes::CMP,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xca => Operation {
            instruction_type: InstructionTypes::DEX,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xcc => Operation {
            instruction_type: InstructionTypes::CPY,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xcd => Operation {
            instruction_type: InstructionTypes::CMP,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xce => Operation {
            instruction_type: InstructionTypes::DEC,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xd0 => Operation {
            instruction_type: InstructionTypes::BNE,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0xd1 => Operation {
            instruction_type: InstructionTypes::CMP,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0xd8 => Operation {
            instruction_type: InstructionTypes::CLD,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xe0 => Operation {
            instruction_type: InstructionTypes::CPX,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xe1 => Operation {
            instruction_type: InstructionTypes::SBC,
            cycle: 6,
            addressing_mode: AddressingModes::IndirectX,
            opc,
        },
        0xe4 => Operation {
            instruction_type: InstructionTypes::CPX,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xe5 => Operation {
            instruction_type: InstructionTypes::SBC,
            cycle: 3,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xe6 => Operation {
            instruction_type: InstructionTypes::INC,
            cycle: 5,
            addressing_mode: AddressingModes::ZeroPage,
            opc,
        },
        0xe8 => Operation {
            instruction_type: InstructionTypes::INX,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        0xe9 => Operation {
            instruction_type: InstructionTypes::SBC,
            cycle: 2,
            addressing_mode: AddressingModes::Immediate,
            opc,
        },
        0xea => Operation {
            instruction_type: InstructionTypes::NOP,
            cycle: 2,
            addressing_mode: AddressingModes::Empty,
            opc,
        },
        0xec => Operation {
            instruction_type: InstructionTypes::CPX,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xed => Operation {
            instruction_type: InstructionTypes::SBC,
            cycle: 4,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xee => Operation {
            instruction_type: InstructionTypes::INC,
            cycle: 6,
            addressing_mode: AddressingModes::Absolute,
            opc,
        },
        0xf0 => Operation {
            instruction_type: InstructionTypes::BEQ,
            cycle: 2,
            addressing_mode: AddressingModes::Relative,
            opc,
        },
        0xf1 => Operation {
            instruction_type: InstructionTypes::SBC,
            cycle: 5,
            addressing_mode: AddressingModes::IndirectY,
            opc,
        },
        0xf8 => Operation {
            instruction_type: InstructionTypes::SED,
            cycle: 2,
            addressing_mode: AddressingModes::Implicit,
            opc,
        },
        _ => panic!("{:x?} Operation Code not implement!", opc),
    }
}
