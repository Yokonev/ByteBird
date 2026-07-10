// Address ranges
const ROM_BANK_00_START:  u16 = 0x0000;
const ROM_BANK_00_END:    u16 = 0x3FFF;

const ROM_BANK_NN_START:  u16 = 0x4000;
const ROM_BANK_NN_END:    u16 = 0x7FFF;

const VRAM_START:         u16 = 0x8000;
const VRAM_END:           u16 = 0x9FFF;

const EXTERNAL_RAM_START: u16 = 0xA000;
const EXTERNAL_RAM_END:   u16 = 0xBFFF;

const WORK_RAM_START:     u16 = 0xC000;
const WORK_RAM_END:       u16 = 0xCFFF;

const WORK_RAM_TWO_START: u16 = 0xD000;
const WORK_RAM_TWO_END:   u16 = 0xDFFF;

const ECHO_RAM_START:     u16 = 0xE000;
const ECHO_RAM_END:       u16 = 0xFDFF;

const OAM_START:          u16 = 0xFE00;
const OAM_END:            u16 = 0xFE9F;

const UNUSABLE_START:     u16 = 0xFEA0;
const UNUSABLE_END:       u16 = 0xFEFF;

const IO_REGS_START:      u16 = 0xFF00; //TODO implement I/O handling
const IO_REGS_END:        u16 = 0xFF7F;

const HIGH_RAM_START:     u16 = 0xFF80;
const HIGH_RAM_END:       u16 = 0xFFFE;

const INTERRUPT_ENABLE:   u16 = 0xFFFF;

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
        self.memory[(addr + 1) as usize] = (data >> 8) as u8;
        self.memory[addr as usize] = (data & 0x00FF) as u8;
    }

    pub fn read_mem_8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_mem_16(&self, addr: u16) -> u16 {
        ((self.memory[(addr + 1) as usize] as u16) << 8) | (self.memory[addr as usize] as u16)
    }
}