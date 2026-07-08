use crate::cpu::regfile::{self, *};

//CALL n16
pub fn op_call_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//CALL cc, n16
pub fn op_call_cc_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//JP HL
pub fn op_jmp_hl(reg_file: &mut Regfile) -> () {
    //TODO
}

//JP n16
pub fn op_jmp_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//JP cc, n16
pub fn op_jmp_cc_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//JR n16
pub fn op_jr_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//JR cc, n16
pub fn op_jr_cc_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//RET cc
pub fn op_ret_cc(reg_file: &mut Regfile, cond: u8) -> () {
    //TODO
}

//RET
pub fn op_ret(reg_file: &mut Regfile) -> () {
    //TODO
}

//RETI
pub fn op_reti(reg_file: &mut Regfile) -> () {
    //TODO
}

//RST vec
pub fn op_rst_vec(reg_file: &mut Regfile, vector: u8){
    //TODO
}