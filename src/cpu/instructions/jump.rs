use crate::cpu::regfile::{self, *};

//CALL n16
pub fn op_call_n16(reg_file: &mut Regfile, addr: u16) -> u8 {
    //TODO
    24u8
}

//CALL cc, n16
//NOTE: cycles are 12t (not taken) / 24t (taken); left returning () until the flag check is implemented
pub fn op_call_cc_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//JP HL
pub fn op_jmp_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    4u8
}

//JP n16
pub fn op_jmp_n16(reg_file: &mut Regfile, addr: u16) -> u8 {
    //TODO
    16u8
}

//JP cc, n16
//NOTE: cycles are 12t (not taken) / 16t (taken); left returning () until the flag check is implemented
pub fn op_jmp_cc_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//JR n16
pub fn op_jr_n16(reg_file: &mut Regfile, addr: u16) -> u8 {
    //TODO
    12u8
}

//JR cc, n16
//NOTE: cycles are 8t (not taken) / 12t (taken); left returning () until the flag check is implemented
pub fn op_jr_cc_n16(reg_file: &mut Regfile, addr: u16) -> () {
    //TODO
}

//RET cc
//NOTE: cycles are 8t (not taken) / 20t (taken); left returning () until the flag check is implemented
pub fn op_ret_cc(reg_file: &mut Regfile, cond: u8) -> () {
    //TODO
}

//RET
pub fn op_ret(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}

//RETI
pub fn op_reti(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}

//RST vec
pub fn op_rst_vec(reg_file: &mut Regfile, vector: u8) -> u8 {
    //TODO
    16u8
}
