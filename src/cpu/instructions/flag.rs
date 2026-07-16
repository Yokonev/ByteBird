use crate::cpu::regfile::{self, *};
use crate::mem::mem::Memory;

//NOTE: u3 refers to an unsigned int on 3 bits, allowing values 0..7 (the indices of a byte)
//Since there isn't a native u3 type in Rust (in most languages for that matter), a check will be made
//in each function.

//BIT u3, r8
pub fn op_bit_u3_r8(reg_file: &mut Regfile, bit: u8, source: DmgSimpleRegisters) -> u8 {
    let value = reg_file.read_register(source);
    reg_file.write_flags(
        (value & (1u8 << bit) == 0), 
        false, 
        true, 
        reg_file.read_flag(DmgFlags::Carry));
    8u8
}

//BIT u3, [HL]
pub fn op_bit_u3_hl(reg_file: &mut Regfile, mem: &mut Memory, bit: u8) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(addr);
    reg_file.write_flags(
        (value & (1u8 << bit) == 0), 
        false, 
        true, 
        reg_file.read_flag(DmgFlags::Carry));
    12u8
}

//RES u3, r8
pub fn op_res_u3_r8(reg_file: &mut Regfile, bit: u8, source: DmgSimpleRegisters) -> u8 {
    let mut value = reg_file.read_register(source);
    value &= !(1u8 << bit);
    reg_file.write_register(source, value);
    8u8
}

//RES u3, [HL]
pub fn op_res_u3_hl(reg_file: &mut Regfile, mem: &mut Memory, bit: u8) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mut value = mem.read_mem_8(addr);
    value &= !(1u8 << bit);
    mem.write_mem_8(addr, value);
    16u8
}

//SET u3, r8
pub fn op_set_u3_r8(reg_file: &mut Regfile, bit: u8, source: DmgSimpleRegisters) -> u8 {
    let mut value = reg_file.read_register(source);
    value |= 1u8 << bit;
    reg_file.write_register(source, value);
    8u8
}

//SET u3, [HL]
pub fn op_set_u3_hl(reg_file: &mut Regfile, mem: &mut Memory, bit: u8) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mut value = mem.read_mem_8(addr);
    value |= 1u8 << bit;
    mem.write_mem_8(addr, value);
    16u8
}
