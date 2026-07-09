use crate::cpu::regfile::*;

//AND A, r8
pub fn op_and_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//AND A, [HL]
pub fn op_and_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//AND A, n8
pub fn op_and_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    //TODO
    8u8
}

//CPL (Bitwise not)
pub fn op_cpl(reg_file: &mut Regfile) -> u8 {
    //TODO
    4u8
}

//OR A, r8
pub fn op_or_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//OR A, [HL]
pub fn op_or_acc_hl(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//OR A, n8
pub fn op_or_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    //TODO
    8u8
}

//XOR A, r8
pub fn op_xor_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//XOR A, [HL]
pub fn op_xor_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    8u8
}

//XOR A, n8
pub fn op_xor_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    //TODO
    8u8
}
