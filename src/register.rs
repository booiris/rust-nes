use std::ops::{Add, AddAssign, SubAssign};

use crate::{memory::CpuMemory, CONST::STACK_BASE};

#[allow(dead_code)]
#[repr(u8)]
pub enum Flags {
    C = 1 << 0, // Carry
    Z = 1 << 1, // Zero
    I = 1 << 2, // Disable interrupt
    D = 1 << 3, // Decimal Mode ( unused in nes )
    B = 1 << 4, // Break
    U = 1 << 5, // Unused ( always 1 )
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

#[derive(Debug)]
pub struct Register<T: Add> {
    data: T,
}

impl AddAssign<u8> for Register<u8> {
    fn add_assign(&mut self, num: u8) {
        self.data += num;
    }
}

impl SubAssign<u8> for Register<u8> {
    fn sub_assign(&mut self, rhs: u8) {
        self.data -= rhs;
    }
}

impl AddAssign<u16> for Register<u16> {
    fn add_assign(&mut self, num: u16) {
        self.data += num;
    }
}

impl SubAssign<u16> for Register<u16> {
    fn sub_assign(&mut self, rhs: u16) {
        self.data -= rhs;
    }
}

////////////////////////////////////////////////////////////////
/// trait for Register
////////////////////////////////////////////////////////////////
pub trait RegisterWork<T: Add> {
    fn new() -> Register<T>;

    fn data(&self) -> T;
    fn mut_data(&mut self) -> &mut T;
    fn set_data(&mut self, data: T);

    fn new_with_data(data: T) -> Register<T> {
        Register::<T> { data }
    }
}

////////////////////////////////////////////////////////////////
/// impl for Register u8
////////////////////////////////////////////////////////////////
impl RegisterWork<u8> for Register<u8> {
    fn new() -> Register<u8> {
        Register { data: 0 }
    }

    fn data(&self) -> u8 {
        self.data
    }

    fn mut_data(&mut self) -> &mut u8 {
        &mut self.data
    }

    fn set_data(&mut self, data: u8) {
        self.data = data;
    }
}

impl Register<u8> {
    pub fn get_stack_addr(&self) -> u16 {
        STACK_BASE + self.data as u16
    }
    pub fn stack_push_byte(&mut self, mem: &mut CpuMemory, data: u8) {
        mem.storeb(self.get_stack_addr(), data);
        self.data -= 1;
    }
    pub fn stack_push_word(&mut self, mem: &mut CpuMemory, data: u16) {
        self.data -= 1;
        mem.storew(self.get_stack_addr(), data);
        self.data -= 1;
    }
}

impl Register<u8> {
    pub fn set_flag(&mut self, flag: Flags, on: bool) {
        if on {
            self.data |= flag as u8;
        } else {
            self.data &= !(flag as u8);
        }
    }

    pub fn check_flag(&mut self, flag: Flags) -> bool {
        self.data & flag as u8 != 0
    }
}

////////////////////////////////////////////////////////////////
/// impl for Register u16
////////////////////////////////////////////////////////////////
impl RegisterWork<u16> for Register<u16> {
    fn new() -> Register<u16> {
        Register { data: 0 }
    }

    fn data(&self) -> u16 {
        self.data
    }

    fn mut_data(&mut self) -> &mut u16 {
        &mut self.data
    }

    fn set_data(&mut self, data: u16) {
        self.data = data;
    }
}
