use crate::cpu::regfile::{self, *};

//ADD HL, SP
pub fn op_add_hl_sp(reg_file: &mut Regfile) -> () {
    //TODO
}

//ADD SP, e8
pub fn op_add_sp_e8(reg_file: &mut Regfile, offset: i8) -> () {
    //TODO
}

//DEC SP
pub fn op_dec_sp(reg_file: &mut Regfile) -> () {
    //TODO
}

//INC SP
pub fn op_inc_sp(reg_file: &mut Regfile) -> () {
    //TODO
}

//LD SP, n16
pub fn op_load_sp_n16(reg_file: &mut Regfile, source: u16) -> () {
    //TODO
}

//LD [n16], SP
pub fn op_load_n16_sp(reg_file: &mut Regfile, destination: u16) -> () {
    //TODO
}

//LD HL, SP+e8
pub fn op_load_hl_spe(reg_file: &mut Regfile, offset: i8) -> () {
    //TODO
}

//LD SP, HL
pub fn op_load_sp_hl(reg_file: &mut Regfile) -> () {
    //TODO
}

//POP AF
pub fn op_pop_af(reg_file: &mut Regfile) -> () {
    //TODO
}

//POP r16
pub fn pop_r16(reg_file: &mut Regfile, source: DmgDoubleRegisters) -> () {
    //TODO
}

//PUSH AF
pub fn push_af(reg_file: &mut Regfile) -> () {
    //TODO
}

//PUSH r16
pub fn push_r16(reg_file: &mut Regfile, source: DmgDoubleRegisters) -> () {
    //TODO
}