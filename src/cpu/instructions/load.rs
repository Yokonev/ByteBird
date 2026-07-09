use crate::cpu::regfile::*;

//LD r8, r8
pub fn op_load_r8_r8(reg_file:&mut Regfile, destination:DmgSimpleRegisters, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//LD r8, n8
pub fn op_load_r8_n8(reg_file:&mut Regfile, destination:DmgSimpleRegisters, source:u8) -> u8 {
    8u8
}

//LD r16, n16
pub fn op_load_r16_n16(reg_file:&mut Regfile, destination:DmgDoubleRegisters, source:u16) -> u8 {
    12u8
}

//LD [HL], r8
pub fn op_load_hl_r8(reg_file:&mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LD [HL], n8
pub fn op_load_hl_n8(reg_file:&mut Regfile, source:u8) -> u8 {
    //TODO NEED MEMORY
    12u8
}

//LD r8, [HL]
pub fn op_load_r8_hl(reg_file:&mut Regfile, destination:DmgSimpleRegisters) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LD [r16], A
pub fn op_load_r16_acc(reg_file:&mut Regfile, destination:DmgDoubleRegisters) -> u8 {
    8u8
}

//LD [n16], A
pub fn op_load_n16_acc(reg_file:&mut Regfile, destination:u16) -> u8 {
    //TODO NEED MEMORY
    16u8
}

//LD A, [r16]
pub fn op_load_acc_r16(reg_file:&mut Regfile, source:DmgDoubleRegisters) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LD A, [n16]
pub fn op_load_acc_n16(reg_file:&mut Regfile, source:u16) -> u8 {
    //TODO NEED MEMORY
    16u8
}

//LD [HLI], A
pub fn op_load_hli_acc(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LD [HLD], A
pub fn op_load_hld_acc(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LD A, [HLI]
pub fn op_load_acc_hli(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LD A, [HLD]
pub fn op_load_acc_hld(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LDH [n16], A
pub fn op_load_high_n16_acc(reg_file:&mut Regfile, destination:u16) -> u8 {
    //TODO NEED MEMORY
    12u8
}

//LDH [C], A
pub fn op_load_high_c_acc(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//LDH A, [n16]
pub fn op_load_high_acc_n16(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    12u8
}

//LDH A, [C]
pub fn op_load_high_acc_c(reg_file:&mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}
