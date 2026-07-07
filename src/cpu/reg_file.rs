// Representation of registers for the original Gameboy (DMG) model

const REG_COUNT: usize = 10;
const FLAG_REG_STRIDE: usize = 3; //bits 0..3 are ignored for flags

#[derive(Clone, Copy)]
pub enum DmgSimpleRegisters{
    A = 0, F = 1,
    B = 2, C = 3,
    D = 4, E = 5,
    H = 6, L = 7,
    SP = 8, PC = 9
}

#[derive(Clone, Copy)]
pub enum DmgDoubleRegisters{
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3
}

#[derive(Clone, Copy)]
pub enum DmgFlags{
    ZERO = 4,
    SUBTRACT = 3,
    HCARRY = 2,
    CARRY = 1
}

pub struct RegFile{
    registers: Vec<u8>
}

impl RegFile {

    pub fn new() -> RegFile{
        RegFile { 
            registers: vec![0u8; REG_COUNT]
         }
    }

    pub fn write_register(& mut self, target: DmgSimpleRegisters, value: u8) -> () {
        self.registers[target as usize] = value;
    }

    pub fn read_register(&self, target: DmgSimpleRegisters) -> u8 {
        self.registers[target as usize]
    }

    pub fn write_double_register(& mut self, target: DmgDoubleRegisters, value: u16) -> () {
        let upper: u8 = (value >> 8) as u8;
        let lower: u8 = (value & 0x00FF) as u8;
        let base_index: usize = (target as usize) * 2;
        self.registers[base_index] = lower;
        self.registers[base_index + 1] = upper;
    }

    pub fn read_double_register(&self, target: DmgDoubleRegisters) -> u16 {
        let base_index: usize = (target as usize) * 2;
        let upper: u8 = self.registers[base_index + 1];
        let lower: u8 = self.registers[base_index];
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
}

#[cfg(test)]
mod tests {

    use super::*;

    //Verifies that a new RegFile contains a vector of 10 zeroes
    #[test]
    fn test_registers_correctly_initialized() {
        let reg_file: RegFile = RegFile::new();
        assert_eq!(vec![0; REG_COUNT], reg_file.registers)
    }

    //Verifies that the double reads and writes work as intended considering the little-endian representation
    #[test]
    fn test_read_write_double_register() {
        let value: u16 = 0xABCD;
        let target: DmgDoubleRegisters = DmgDoubleRegisters::HL;
        let mut reg_file: RegFile = RegFile::new();
        
        reg_file.write_double_register(target, value);

        let result = reg_file.read_double_register(target);
        assert_eq!(result, value)
    }

    #[test]
    fn test_modifying_flags_work() {
        let mut reg_file: RegFile = RegFile::new();
        let expected_one: u8 = 0b1111_0000;
        let expected_two: u8 = 0b1001_0000;

        reg_file.write_flag(DmgFlags::ZERO, true);
        reg_file.write_flag(DmgFlags::SUBTRACT, true);
        reg_file.write_flag(DmgFlags::HCARRY, true);
        reg_file.write_flag(DmgFlags::CARRY, true);
        
        assert_eq!(expected_one, reg_file.read_register(DmgSimpleRegisters::F));

        reg_file.write_flag(DmgFlags::SUBTRACT, false);
        reg_file.write_flag(DmgFlags::HCARRY, false);

        assert_eq!(expected_two, reg_file.read_register(DmgSimpleRegisters::F));
    }
}