use crate::cpu::cpu::*;

pub type OpWrapper = fn(&mut Cpu); //ADD MEMORY MAP TYPE LATER

pub enum OpCycleType{
    Normal(u8), //Most instructions
    Conditional(u8, u8) //Instructions where a conditional lead to different cycles
}

pub struct OpEntry {
    mnemonic: &'static str, //Human-readable representation of the instruction
    instruction_length: u8,
    instruction_cycles: OpCycleType,
    exec: OpWrapper
}