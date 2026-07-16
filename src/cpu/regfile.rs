// Representation of registers for the original Gameboy (DMG) model

use crate::mem::mem::Memory;

const REG_COUNT: usize = 8;
const FLAG_REG_STRIDE: usize = 3; //bits 0..3 are ignored for flags

#[derive(Clone, Copy)]
pub enum DmgSimpleRegisters{
    A = 0, F = 1,
    B = 2, C = 3,
    D = 4, E = 5,
    H = 6, L = 7,
}

#[derive(Clone, Copy)]
pub enum DmgDoubleRegisters{
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3,
}

#[derive(Clone, Copy)]
pub enum DmgFlags{
    Zero = 4,
    Subtract = 3,
    HCarry = 2,
    Carry = 1
}

pub struct Regfile{
    registers: Vec<u8>,
    sp: u16,
    pc: u16,
    ime: bool,
    //Set by EI, consumed by Cpu::step() one instruction later. See write_ime_pending.
    ime_pending: bool
}

impl Regfile {

    pub fn new() -> Regfile{
        Regfile { 
            registers: vec![0u8; REG_COUNT],
            sp: 0x0000,
            pc: 0x0000,
            ime: false,
            ime_pending: false
         }
    }

    pub fn write_register(& mut self, target: DmgSimpleRegisters, value: u8) -> () {
        self.registers[target as usize] = value;
    }

    pub fn read_register(&self, target: DmgSimpleRegisters) -> u8 {
        self.registers[target as usize]
    }

    //Note, if endianess is wrong, fix this by inverting upper and lower
    pub fn write_double_register(& mut self, target: DmgDoubleRegisters, value: u16) -> () {
        let upper: u8 = (value >> 8) as u8;
        let lower: u8 = (value & 0x00FF) as u8;
        let base_index: usize = (target as usize) * 2;
        self.registers[base_index] = upper;
        self.registers[base_index + 1] = lower;
    }

    //Note, if endianess is wrong, fix this by inverting upper and lower
    pub fn read_double_register(&self, target: DmgDoubleRegisters) -> u16 {
        let base_index: usize = (target as usize) * 2;
        let upper: u8 = self.registers[base_index];
        let lower: u8 = self.registers[base_index + 1];
        ((upper as u16) << 8) | (lower as u16)
    }

    // Reminder, the F register reads as follows: 0xZNHC0000
    pub fn read_flag(&self, target: DmgFlags) -> bool {
        (self.registers[DmgSimpleRegisters::F as usize] & (1 << (FLAG_REG_STRIDE + target as usize))) != 0
    }

    pub fn write_flag(& mut self, target: DmgFlags, value: bool) -> () {
        let mask = 1u8 << (FLAG_REG_STRIDE + target as usize);

        match value {
            true  => (self.registers[DmgSimpleRegisters::F as usize]) |= mask,
            false => (self.registers[DmgSimpleRegisters::F as usize]) &= !mask
        };
    }

    pub fn write_flags(& mut self, zero: bool, subtract: bool, hcarry: bool, carry: bool){
        self.write_flag(DmgFlags::Zero, zero);
        self.write_flag(DmgFlags::Subtract, subtract);
        self.write_flag(DmgFlags::HCarry, hcarry);
        self.write_flag(DmgFlags::Carry, carry);
    }

    pub fn read_sp(& self) -> u16 {
        self.sp
    }

    pub fn write_sp(& mut self, value: u16) -> () {
        self.sp = value;
    }
    
    pub fn read_pc(& self) -> u16 {
        self.pc
    }

    pub fn write_pc(& mut self, value: u16) -> () {
        self.pc = value;
    }

    pub fn read_ime(& self) -> bool {
        self.ime
    }

    pub fn write_ime(& mut self, value: bool) -> () {
        self.ime = value;
    }

    pub fn read_ime_pending(& self) -> bool {
        self.ime_pending
    }

    //EI sets this instead of IME directly: IME must only come up *after* the instruction
    //following EI, and an instruction closure can't see across instruction boundaries.
    //Cpu::step() latches this before EXECUTE and applies it after. DI clears it, so an
    //EI immediately followed by DI never enables interrupts.
    pub fn write_ime_pending(& mut self, value: bool) -> () {
        self.ime_pending = value;
    }

    //Reads the byte at PC and advances PC by one (fetch-and-advance). After the
    //opcode + all operands are fetched, PC points at the next instruction.
    pub fn fetch_byte(& mut self, mem: &Memory) -> u8 {
        let byte: u8 = mem.read_mem_8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    //Reads a little-endian u16 operand at PC (low byte first) and advances PC by two.
    pub fn fetch_word(& mut self, mem: &Memory) -> u16 {
        let lower: u16 = self.fetch_byte(mem) as u16;
        let upper: u16 = self.fetch_byte(mem) as u16;
        (upper << 8) | lower
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    //Verifies that a new RegFile contains a vector of 10 zeroes
    #[test]
    fn test_registers_correctly_initialized() {
        let reg_file: Regfile = Regfile::new();
        assert_eq!(vec![0; REG_COUNT], reg_file.registers)
    }

    //Verifies that the double reads and writes work as intended considering the little-endian representation
    #[test]
    fn test_read_write_double_register() {
        let value: u16 = 0xABCD;
        let target: DmgDoubleRegisters = DmgDoubleRegisters::HL;
        let mut reg_file: Regfile = Regfile::new();
        
        reg_file.write_double_register(target, value);

        let result = reg_file.read_double_register(target);
        assert_eq!(result, value)
    }

    //Verifies fetch_byte returns the byte at PC and advances PC by one each call
    #[test]
    fn test_fetch_byte_advances_pc() {
        let mut mem: Memory = Memory::new();
        mem.write_mem_8(0xC000, 0x12);
        mem.write_mem_8(0xC001, 0x34);
        let mut reg_file: Regfile = Regfile::new();
        reg_file.write_pc(0xC000);

        assert_eq!(0x12, reg_file.fetch_byte(&mem));
        assert_eq!(0xC001, reg_file.read_pc());
        assert_eq!(0x34, reg_file.fetch_byte(&mem));
        assert_eq!(0xC002, reg_file.read_pc());
    }

    //Verifies fetch_word reads a little-endian u16 (low byte first) and advances PC by two
    #[test]
    fn test_fetch_word_little_endian() {
        let mut mem: Memory = Memory::new();
        mem.write_mem_8(0xC000, 0xCD); //low byte
        mem.write_mem_8(0xC001, 0xAB); //high byte
        let mut reg_file: Regfile = Regfile::new();
        reg_file.write_pc(0xC000);

        assert_eq!(0xABCD, reg_file.fetch_word(&mem));
        assert_eq!(0xC002, reg_file.read_pc());
    }

    //Verifies PC wraps rather than panicking when fetching at the top of the address space
    #[test]
    fn test_fetch_byte_wraps_at_boundary() {
        let mut mem: Memory = Memory::new();
        mem.write_mem_8(0xFFFF, 0x99);
        let mut reg_file: Regfile = Regfile::new();
        reg_file.write_pc(0xFFFF);

        assert_eq!(0x99, reg_file.fetch_byte(&mem));
        assert_eq!(0x0000, reg_file.read_pc());
    }

    #[test]
    fn test_modifying_flags_work() {
        let mut reg_file: Regfile = Regfile::new();
        let expected_one: u8 = 0b1111_0000;
        let expected_two: u8 = 0b1001_0000;

        reg_file.write_flag(DmgFlags::Zero, true);
        reg_file.write_flag(DmgFlags::Subtract, true);
        reg_file.write_flag(DmgFlags::HCarry, true);
        reg_file.write_flag(DmgFlags::Carry, true);
        
        assert_eq!(expected_one, reg_file.read_register(DmgSimpleRegisters::F));

        reg_file.write_flag(DmgFlags::Subtract, false);
        reg_file.write_flag(DmgFlags::HCarry, false);

        assert_eq!(expected_two, reg_file.read_register(DmgSimpleRegisters::F));
    }
}