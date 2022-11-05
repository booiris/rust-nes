use std::ops::{Add, AddAssign, SubAssign};

#[allow(dead_code)]
#[repr(u8)]
enum Flags {
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

// impl Add<u8> for Register<u8> {
//     type Output = Register<u8>;

//     fn add(self, num: u8) -> Register<u8> {
//         <Register<u8> as RegisterWork<u8>>::new_with_data(self.data + num)
//     }
// }

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

// impl Add<u16> for Register<u16> {
//     type Output = Register<u16>;

//     fn add(self, num: u16) -> Register<u16> {
//         <Register<u16> as RegisterWork<u16>>::new_with_data(self.data + num)
//     }
// }

impl AddAssign<u16> for Register<u16> {
    fn add_assign(&mut self, num: u16) {
        self.data += num;
    }
}

////////////////////////////////////////////////////////////////
/// trait for Register
////////////////////////////////////////////////////////////////
pub trait RegisterWork<T: Add> {
    fn new() -> Register<T>;

    fn data(&self) -> T;
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

    fn set_data(&mut self, data: u8) {
        self.data = data;
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
    fn set_data(&mut self, data: u16) {
        self.data = data;
    }
}
