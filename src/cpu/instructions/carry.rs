use crate::cpu::regfile::{self, *};

//CCF
pub fn op_ccf(reg_file: &mut Regfile) -> u8 {
    let value = !reg_file.read_flag(DmgFlags::Carry);
    reg_file.write_flags(
        reg_file.read_flag(DmgFlags::Zero),
        false,
        false, 
        value);
    4u8
}

//SCF
pub fn op_scf(reg_file: &mut Regfile) -> u8 {
    reg_file.write_flags(
        reg_file.read_flag(DmgFlags::Zero),
        false,
        false, 
        true);
    4u8
}
