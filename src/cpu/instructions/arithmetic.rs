use crate::cpu::cpu::*;

// ===== ADD WITH CARRY =====

//ADC A, r8
pub fn op_adc_acc_r8(reg_file: &mut Cpu, source:DmgSimpleRegisters) -> () {
    //TODO
}

//ADC A, [HL]
pub fn op_adc_acc_hl(reg_file: &mut Cpu) -> () {
    //TODO NEED MEMORY
}

//ADC A, n8
pub fn op_adc_acc_n8(reg_file: &mut Cpu, source:u8) -> () {
    //TODO
}

// ===== ADD WITHOUT CARRY =====

//ADD A, r8
pub fn op_add_acc_r8(reg_file: &mut Cpu, source:DmgSimpleRegisters) -> () {
    //TODO
}

//ADD A, [HL]
pub fn op_add_acc_hl(reg_file: &mut Cpu) -> () {
    //TODO NEED MEMORY
}

//ADD A, n8
pub fn op_add_acc_n8(reg_file: &mut Cpu, source:u8) -> () {
    //TODO
}

//ADD HL, r16
pub fn op_add_hl_r16(reg_file: &mut Cpu, source:DmgDoubleRegisters) -> () {
    //TODO
}

// ===== COMPARE =====

//CP A, r8
pub fn op_cp_acc_r8(reg_file: &mut Cpu, source:DmgSimpleRegisters) -> () {
    //TODO
}

//CP A, [HL]
pub fn op_cp_acc_hl(reg_file: &mut Cpu) -> () {
    //TODO NEED MEMORY
}

//CP A, n8
pub fn op_cp_acc_u8(reg_file: &mut Cpu, source:u8) -> () {
    //TODO
}

// ===== DEC =====

//DEC r8
pub fn op_dec_r8(reg_file: &mut Cpu, destination:DmgSimpleRegisters) -> () {
    //TODO
}

//DEC [HL]
pub fn op_dec_hl(reg_file: &mut Cpu) -> () {
    //TODO NEED MEMORY
}

//DEC r16
pub fn op_dec_r16(reg_file: &mut Cpu, destination:DmgDoubleRegisters) -> () {
    //TODO
}

// ===== INC =====

//INC r8
pub fn op_inc_r8(reg_file: &mut Cpu, destination:DmgSimpleRegisters) -> () {
    //TODO
}

//INC [HL]
pub fn op_inc_hl(reg_file: &mut Cpu) -> () {

}

//INC r16
pub fn op_inc_r16(reg_file: &mut Cpu, destination:DmgDoubleRegisters) -> () {
    
}

// ===== SUBTRACT WITH CARRY =====

//SBC A, r8
pub fn op_sbc_acc_r8(reg_file: &mut Cpu, destination:DmgSimpleRegisters) -> () {
    //TODO
}

//SBC A, [HL]
pub fn op_sbc_acc_hl(reg_file: &mut Cpu) -> () {
    //TODO NEED MEMORY
}

//SBC A, n8
pub fn op_sbc_acc_n8(reg_file: &mut Cpu, destination:u8) -> () {
    //TODO
}

// ===== SUBTRACT WITHOUT CARRY =====

//SUB A, r8
pub fn op_sub_acc_r8(reg_file: &mut Cpu, destination:DmgSimpleRegisters) -> () {
    //TODO
}

//SUB A, [HL]
pub fn op_sub_acc_hl(reg_file: &mut Cpu) -> () {
    //TODO NEED MEMORY
}

//SUB A, n8
pub fn op_sub_acc_n8(reg_file: &mut Cpu, destination:u8) -> () {
    //TODO
}