use crate::cpu::regfile::{self, *};

//RL r8
pub fn op_rl_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//RL [HL]
pub fn op_rl_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    16u8
}

//RLA
pub fn op_rla(reg_file: &mut Regfile) -> u8 {
    //TODO
    4u8
}

//RLC r8
pub fn op_rlc_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//RLC [HL]
pub fn op_rlc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    16u8
}

//RLCA
pub fn op_rlca(reg_file: &mut Regfile) -> u8 {
    //TODO
    4u8
}

//RR r8
pub fn op_rr_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//RR [HL]
pub fn op_rr_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    16u8
}

//RRA
pub fn op_rra(reg_file: &mut Regfile) -> u8 {
    //TODO
    4u8
}

//RRC r8
pub fn op_rrc_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//RRC [HL]
pub fn op_rrc_hl(reg_file: &mut Regfile) -> u8 {
    //TODO NEED MEMORY
    16u8
}

//RRCA
pub fn op_rrca(reg_file: &mut Regfile) -> u8 {
    //TODO
    4u8
}

//SLA r8
pub fn op_sla_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//SLA [HL]
pub fn op_sla_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}

//SRA r8
pub fn op_sra_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//SRA [HL]
pub fn op_sra_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}

//SRL r8
pub fn op_srl_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//SRL [HL]
pub fn op_srl_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}

//SWAP r8
pub fn op_swap_r8(reg_file: &mut Regfile, source: DmgSimpleRegisters) -> u8 {
    //TODO
    8u8
}

//SWAP [HL]
pub fn op_swap_hl(reg_file: &mut Regfile) -> u8 {
    //TODO
    16u8
}
