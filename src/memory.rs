use crate::ROM::ROM;

pub struct CpuMemory {
    ram: [u8; 2048],
    rom: ROM,
}

impl CpuMemory {
    pub fn new(rom: ROM) -> Self {
        CpuMemory {
            ram: [0; 2048],
            rom,
        }
    }
}

impl CpuMemory {
    pub fn storeb(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram[(address & 0x07FF) as usize] = data;
        } else if address < 0x6000 {
            todo!()
        } else {
            self.rom.write(address, data);
        }
    }

    pub fn storew(&mut self, address: u16, data: u16) {
        self.storeb(address, (data & 0xFF) as u8);
        self.storeb(address + 1, ((data >> 8) & 0xFF) as u8)
    }

    pub fn loadb(&self, address: &mut u16) -> u8 {
        let res;
        if *address < 0x2000 {
            res = self.ram[(*address & 0x07FF) as usize];
        } else if *address < 0x6000 {
            todo!()
        } else {
            res = self.rom.read(address);
        }
        *address += 1;
        res
    }

    pub fn loadw(&self, address: &mut u16) -> u16 {
        let low = self.loadb(address) as u16;
        let high = (self.loadb(address) as u16) << 8;
        high | low
    }
}
