use crate::cpu::regfile::{self, *};

//ADD HL, SP
pub fn op_add_hl_sp(reg_file: &mut Regfile) -> u8 {
    //TODO
    8u8
}

//ADD SP, e8
pub fn op_add_sp_e8(reg_file: &mut Regfile, offset: i8) -> u8 {
    //TODO
    16u8
}

//DEC SP
pub fn op_dec_sp(reg_file: &mut Regfile) -> u8 {
    //TODO
    8u8
}

//INC SP
pub fn op_inc_sp(reg_file: &mut Regfile) -> u8 {
    //TODO
    8u8
}

//LD SP, n16
pub fn op_load_sp_n16(reg_file: &mut Regfile, source: u16) -> u8 {
    //TODO
    12u8
}

//LD [n16], SP
pub fn op_load_n16_sp(reg_file: &mut Regfile, destination: u16) -> u8 {
    //TODO
    20u8
}

//LD HL, SP+e8
pub fn op_load_hl_spe(reg_file: &mut Regfile, offset: i8) -> u8 {
    //TODO
    12u8
}

//LD SP, HL
pub fn op_load_sp_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    8u8
}

//POP AF
pub fn op_pop_af(reg_file: &mut Regfile) -> u8 {
    //TODO
    12u8
}

//POP r16
pub fn pop_r16(reg_file: &mut Regfile, source: DmgDoubleRegisters) -> u8 {
    //TODO
    12u8
}

//PUSH AF
pub fn push_af(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}

//PUSH r16
pub fn push_r16(reg_file: &mut Regfile, source: DmgDoubleRegisters) -> u8 {
    //TODO
    16u8
}
