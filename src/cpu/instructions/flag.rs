use crate::cpu::reg_file::{self, *};

//NOTE: u3 refers to an unsigned int on 3 bits, allowing values 0..7 (the indices of a byte)
//Since there isn't a native u3 type in Rust (in most languages for that matter), a check will be made
//in each function.

//BIT u3, r8
pub fn op_bit_u3_r8(reg_file: &RegFile, bit: u8, source: DmgSimpleRegisters) -> () {
    //TODO
}

//BIT u3, [HL]
pub fn op_bit_u3_hl(reg_file: &RegFile, bit: u8) -> () {
    //TODO
}

//RES u3, r8
pub fn op_res_u3_r8(reg_file: &RegFile, bit: u8, source: DmgSimpleRegisters) -> () {
    //TODO
}

//RES u3, [HL]
pub fn op_res_u3_hl(reg_file: &RegFile, bit: u8) -> () {
    //TODO
}

//SET u3, r8
pub fn op_set_u3_r8(reg_file: &RegFile, bit: u8, source: DmgSimpleRegisters) -> () {
    //TODO
}

//SET u3, [HL]
pub fn op_set_u3_hl(reg_file: &RegFile, bit: u8) -> () {
    //TODO
}