use crate::cpu::regfile::{self, *};

//DI
//Takes effect immediately, and cancels an EI whose enable hasn't landed yet.
pub fn op_di(reg_file: &mut Regfile) -> u8 {
    reg_file.write_ime(false);
    reg_file.write_ime_pending(false);
    4u8
}

//EI
//Deliberately does not touch IME: the enable only takes effect after the instruction
//*following* EI, so it is recorded here and applied by Cpu::step().
pub fn op_ei(reg_file: &mut Regfile) -> u8 {
    reg_file.write_ime_pending(true);
    4u8
}

//HALT
pub fn op_halt(reg_file: &mut Regfile) -> u8 {
    //Very specific behavior, handle later
    4u8
}
