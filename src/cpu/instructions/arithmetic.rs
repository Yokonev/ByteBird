use crate::cpu::regfile::*;

// ===== ADD WITH CARRY =====

//ADC A, r8
pub fn op_adc_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//ADC A, [HL]
pub fn op_adc_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//ADC A, n8
pub fn op_adc_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    //TODO
    8u8
}

// ===== ADD WITHOUT CARRY =====

//ADD A, r8
pub fn op_add_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//ADD A, [HL]
pub fn op_add_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//ADD A, n8
pub fn op_add_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    //TODO
    8u8
}

//ADD HL, r16
pub fn op_add_hl_r16(reg_file: &mut Regfile, source:DmgDoubleRegisters) -> u8 {
    //TODO
    8u8
}

// ===== COMPARE =====

//CP A, r8
pub fn op_cp_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//CP A, [HL]
pub fn op_cp_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//CP A, n8
pub fn op_cp_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    //TODO
    8u8
}

// ===== DEC =====

//DEC r8
pub fn op_dec_r8(reg_file: &mut Regfile, destination:DmgSimpleRegisters) -> u8 {
    4u8
}

//DEC [HL]
pub fn op_dec_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    12u8
}

//DEC r16
pub fn op_dec_r16(reg_file: &mut Regfile, destination:DmgDoubleRegisters) -> u8 {
    //TODO
    8u8
}

// ===== INC =====

//INC r8
pub fn op_inc_r8(reg_file: &mut Regfile, destination:DmgSimpleRegisters) -> u8 {
    4u8
}

//INC [HL]
pub fn op_inc_hl(reg_file: &mut Regfile) -> u8 {
    12u8
}

//INC r16
pub fn op_inc_r16(reg_file: &mut Regfile, destination:DmgDoubleRegisters) -> u8 {
    8u8
}

// ===== SUBTRACT WITH CARRY =====

//SBC A, r8
pub fn op_sbc_acc_r8(reg_file: &mut Regfile, destination:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//SBC A, [HL]
pub fn op_sbc_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//SBC A, n8
pub fn op_sbc_acc_n8(reg_file: &mut Regfile, destination:u8) -> u8 {
    //TODO
    8u8
}

// ===== SUBTRACT WITHOUT CARRY =====

//SUB A, r8
pub fn op_sub_acc_r8(reg_file: &mut Regfile, destination:DmgSimpleRegisters) -> u8 {
    //TODO
    4u8
}

//SUB A, [HL]
pub fn op_sub_acc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    8u8
}

//SUB A, n8
pub fn op_sub_acc_n8(reg_file: &mut Regfile, destination:u8) -> u8 {
    //TODO
    8u8
}
