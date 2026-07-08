use crate::cpu::cpu::*;

//LD r8, r8
pub fn op_load_r8_r8(reg_file:&mut Cpu, destination:DmgSimpleRegisters, source:DmgSimpleRegisters) -> () {
    //TODO
}

//LD r8, n8
pub fn op_load_r8_n8(reg_file:&mut Cpu, destination:DmgSimpleRegisters, source:u8) -> () {
    //TODO
}

//LD r16, n16
pub fn op_load_r16_n16(reg_file:&mut Cpu, destination:DmgDoubleRegisters, source:u16) -> () {
    //TODO
}

//LD [HL], r8
pub fn op_load_hl_r8(reg_file:&mut Cpu, source:DmgSimpleRegisters) -> () {
    //TODO NEED MEMORY
}

//LD [HL], n8
pub fn op_load_hl_n8(reg_file:&mut Cpu, source:u8) -> () {
    //TODO NEED MEMORY
}

//LD r8, [HL]
pub fn op_load_r8_hl(reg_file:&mut Cpu, destination:DmgSimpleRegisters) -> () {
    //TODO NEED MEMORY
}

//LD [r16], A
pub fn op_load_r16_acc(reg_file:&mut Cpu, destination:DmgDoubleRegisters) -> () {
    //TODO NEED MEMORY
}

//LD [n16], A
pub fn op_load_n16_acc(reg_file:&mut Cpu, destination:u16) -> () {
    //TODO NEED MEMORY
}

//LD A, [r16]
pub fn op_load_acc_r16(reg_file:&mut Cpu, source:DmgDoubleRegisters) -> () {
    //TODO NEED MEMORY
}

//LD A, [n16]
pub fn op_load_acc_n16(reg_file:&mut Cpu, source:u16) -> () {
    //TODO NEED MEMORY
}

//LD [HLI], A
pub fn op_load_hli_acc(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}

//LD [HLD], A
pub fn op_load_hld_acc(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}

//LD A, [HLI]
pub fn op_load_acc_hli(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}

//LD A, [HLD]
pub fn op_load_acc_hld(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}

//LDH [n16], A
pub fn op_load_high_n16_acc(reg_file:&mut Cpu, destination:u16) -> () {
    //TODO NEED MEMORY
}

//LDH [C], A
pub fn op_load_high_c_acc(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}

//LDH A, [n16]
pub fn op_load_high_acc_n16(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}

//LDH A, [C]
pub fn op_load_high_acc_c(reg_file:&mut Cpu) -> () {
    //TODO NEED MEMORY
}