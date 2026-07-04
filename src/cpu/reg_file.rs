// Representation of registers for the original Gameboy (DMG) model
// Uses a vector of unsigned integers to represent the data of the registers
// and enumerations to access them.

const REG_COUNT: usize = 10;

// Registers are already grouped to be more easily accessible together
// Ex: Register A is 0, register F is 1, so register AF is register 0 and 0+1
// Register B is 2, register C is 3, so register BC is register 2 and register 2+1...
// This is used to facilitate reads and writes operations with 16 bits register ops
#[derive(Clone, Copy)]
enum DmaSimpleRegisters{
    A = 0, F = 1,
    B = 2, C = 3,
    D = 4, E = 5,
    H = 6, L = 7,
    SP = 8, PC = 9
}

#[derive(Clone, Copy)]
enum DmaDoubleRegisters{
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3
}

struct RegFile{
    registers: Vec<u8>
}

impl RegFile {

    fn new() -> RegFile{
        RegFile { registers: vec![0u8; REG_COUNT] }
    }

    fn write_register(& mut self, target: DmaSimpleRegisters, value: u8) -> () {
        self.registers[target as usize] = value;
    }

    fn read_register(&self, target: DmaSimpleRegisters) -> u8 {
        self.registers[target as usize]
    }

    fn write_double_register(& mut self, target: DmaDoubleRegisters, value: u16) -> () {
        let upper: u8 = (value >> 8) as u8;
        let lower: u8 = (value & 0x00FF) as u8;
        let base_index: usize = (target as usize) * 2;
        self.registers[base_index] = lower;
        self.registers[base_index + 1] = upper;

    }

    fn read_double_register(&self, target: DmaDoubleRegisters) -> u16 {
        let base_index: usize = (target as usize) * 2;
        let upper: u8 = self.registers[base_index + 1];
        let lower: u8 = self.registers[base_index];
        ((upper as u16) << 8) | (lower as u16)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_regfile_correctly_initialized() {
        let reg_file: RegFile = RegFile::new();
        assert_eq!(vec![0; 10], reg_file.registers)
    }

    #[test]
    fn test_read_write_double_register() {
        let value: u16 = 0xABCD;
        let target: DmaDoubleRegisters = DmaDoubleRegisters::HL;
        let mut reg_file: RegFile = RegFile::new();
        
        reg_file.write_double_register(target, value);

        let result = reg_file.read_double_register(target);
        assert_eq!(result, value)
    }

}