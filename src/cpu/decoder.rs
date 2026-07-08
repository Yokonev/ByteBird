use crate::cpu::cpu::*;

const OP_TABLE_SIZE: u8 = 0xFF;

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

pub struct OpTable {
    normal: vec<OpEntry>,
    alt: vec<OpEntry>,
    is_prefixed_mode: bool
}

impl OpTable{

    pub fn new() -> OpTable {
        OpTable { 
            normal: OpTable::initialize_table(),
            alt: OpTable::initialize_alt_table(),
            is_prefixed_mode: false 
        }
    }

    pub fn execute(cpu: & mut Cpu, addr: u8) -> () {

    }

    //Initializes the normal operation table
    fn initialize_table() -> Vec<OpTable> {
        vec![]
    }

    //Initializes the alt operation table (prefixed)
    fn initialize_alt_table() -> Vec<OpTable> {
        vec![]
    }
}