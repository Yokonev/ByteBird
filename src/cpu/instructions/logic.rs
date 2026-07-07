use crate::cpu::reg_file::*;

//AND A, r8
pub fn op_and_acc_r8(reg_file:&RegFile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//AND A, [HL]
pub fn op_and_acc_hl(reg_file:&RegFile) -> () {
    //TODO NEED MEMORY
}

//AND A, n8
pub fn op_and_acc_n8(reg_file:&RegFile, source:u8) -> () {
    //TODO
}

//CPL (Bitwise not)
pub fn op_cpl(reg_file:&RegFile) -> () {
    //TODO
}

//OR A, r8
pub fn op_or_acc_r8(reg_file:&RegFile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//OR A, [HL]
pub fn op_or_acc_hl(reg_file:&RegFile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//OR A, n8
pub fn op_or_acc_n8(reg_file:&RegFile, source:u8) -> () {
    //TODO
}

//XOR A, r8
pub fn op_xor_acc_r8(reg_file:&RegFile, source:DmgSimpleRegisters) -> () {
    //TODO
}

//XOR A, [HL]
pub fn op_xor_acc_hl(reg_file:&RegFile) -> () {
    //TODO
}

//XOR A, n8
pub fn op_xor_acc_n8(reg_file:&RegFile, source:u8) -> () {
    //TODO
}