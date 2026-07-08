use crate::cpu::regfile::*;

//AND A, r8
pub fn op_and_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//AND A, [HL]
pub fn op_and_acc_hl(reg_file: &mut Regfile) -> () {
    //TODO NEED MEMORY
}

//AND A, n8
pub fn op_and_acc_n8(reg_file: &mut Regfile, source:u8) -> () {
    //TODO
}

//CPL (Bitwise not)
pub fn op_cpl(reg_file: &mut Regfile) -> () {
    //TODO
}

//OR A, r8
pub fn op_or_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//OR A, [HL]
pub fn op_or_acc_hl(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//OR A, n8
pub fn op_or_acc_n8(reg_file: &mut Regfile, source:u8) -> () {
    //TODO
}

//XOR A, r8
pub fn op_xor_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//XOR A, [HL]
pub fn op_xor_acc_hl(reg_file: &mut Regfile) -> () {
    //TODO
}

//XOR A, n8
pub fn op_xor_acc_n8(reg_file: &mut Regfile, source:u8) -> () {
    //TODO
}