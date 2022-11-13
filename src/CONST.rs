pub const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const PRG_ROM_PAGE_SIZE: usize = 16384;
pub const CHR_ROM_PAGE_SIZE: usize = 8192;
pub const STACK_BASE: u16 = 0x0100;
pub const RESET_ADDR: u16 = 0xFFFC;
pub const NMI_ADDR: u16 = 0xFFFA;
pub const IRQ_ADDR: u16 = 0xFFFE;
