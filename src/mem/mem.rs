// Address ranges
pub const ROM_BANK_00_START:  u16 = 0x0000;
pub const ROM_BANK_00_END:    u16 = 0x3FFF;

pub const ROM_BANK_NN_START:  u16 = 0x4000;
pub const ROM_BANK_NN_END:    u16 = 0x7FFF;

pub const VRAM_START:         u16 = 0x8000;
pub const VRAM_END:           u16 = 0x9FFF;

pub const EXTERNAL_RAM_START: u16 = 0xA000;
pub const EXTERNAL_RAM_END:   u16 = 0xBFFF;

pub const WORK_RAM_START:     u16 = 0xC000;
pub const WORK_RAM_END:       u16 = 0xCFFF;

pub const WORK_RAM_TWO_START: u16 = 0xD000;
pub const WORK_RAM_TWO_END:   u16 = 0xDFFF;

pub const ECHO_RAM_START:     u16 = 0xE000;
pub const ECHO_RAM_END:       u16 = 0xFDFF;

pub const OAM_START:          u16 = 0xFE00;
pub const OAM_END:            u16 = 0xFE9F;

pub const UNUSABLE_START:     u16 = 0xFEA0;
pub const UNUSABLE_END:       u16 = 0xFEFF;

pub const IO_REGS_START:      u16 = 0xFF00; //TODO implement I/O handling
pub const IO_REGS_END:        u16 = 0xFF7F;

pub const HIGH_RAM_START:     u16 = 0xFF80;
pub const HIGH_RAM_END:       u16 = 0xFFFE;

pub const INTERRUPT_ENABLE:   u16 = 0xFFFF;

pub struct Memory {
    memory: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: vec![0u8; 0x10000] }
    }

    pub fn write_mem_8(& mut self, addr: u16, data: u8) -> () {
        self.memory[addr as usize] = data;
    }

    pub fn write_mem_16(& mut self, addr: u16, data: u16) -> () {
        self.memory[(addr.wrapping_add(1)) as usize] = (data >> 8) as u8;
        self.memory[addr as usize] = (data & 0x00FF) as u8;
    }

    pub fn read_mem_8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_mem_16(&self, addr: u16) -> u16 {
        ((self.memory[(addr.wrapping_add(1)) as usize] as u16) << 8) | (self.memory[addr as usize] as u16)
    }
}