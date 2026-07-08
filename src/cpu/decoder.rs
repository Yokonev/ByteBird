use crate::cpu::regfile::*;
use crate::mem::mem::Memory;

/// Instruction decoder for the ByteBird emulator. Every OpCode used by the original Gameboy is being considered
/// and contained inside a decoding table. If the prefix instruction is detected, the decoded switches to a second table
/// with other instructions in it

const OP_TABLE_SIZE: usize = 0xFF;

pub type OpWrapper = fn(&mut Regfile, &mut Memory) -> u8; //Instruction wrapper, returns the number of cycles used in the process

#[derive(Clone, Copy)]
pub struct OpEntry {
    mnemonic: &'static str, //Human-readable representation of the instruction
    instruction_length: u8,
    exec: OpWrapper
}

impl OpEntry {
    pub fn default() -> Self {
        Self { 
            mnemonic: "unknown_op", 
            instruction_length: 0, 
            exec: |_regfile: &mut Regfile, _memory: &mut Memory| -> u8 {
                unimplemented!("This instruction is currently unimplemented")
            } 
        }
    }

    pub fn new(
        mnemonic: &'static str, 
        ins_len: u8, 
        ins: OpWrapper) -> Self
    {
        Self { 
            mnemonic, 
            instruction_length: ins_len, 
            exec: ins 
        }    
    }

    pub fn get_mnemonic(&self) -> &'static str {
        self.mnemonic
    }

    pub fn get_instruction_length(&self) -> u8 {
        self.instruction_length
    }

    pub fn get_instruction(&self) -> OpWrapper {
        self.exec
    }
}

pub struct OpTable {
    normal: Vec<OpEntry>,
    alt: Vec<OpEntry>,
    is_prefixed_mode: bool
}

impl OpTable{

    pub fn new() -> Self {
        Self { 
            normal: OpTable::initialize_table(),
            alt: OpTable::initialize_alt_table(),
            is_prefixed_mode: false 
        }
    }

    pub fn decode(& mut self, addr: u8) -> OpEntry {
        match self.is_prefixed_mode {
            true => {
                self.is_prefixed_mode = false;
                self.alt[addr as usize]
            },
            false => self.normal[addr as usize]
        }
    }

    //Initializes the normal operation table
    fn initialize_table() -> Vec<OpEntry> {
        vec![OpEntry::default(); OP_TABLE_SIZE]
    }

    //Initializes the alt operation table (prefixed)
    fn initialize_alt_table() -> Vec<OpEntry> {
        vec![OpEntry::default(); OP_TABLE_SIZE]
    }

    fn prefix_op(&mut self, _regfile: &mut Regfile, _memory: &mut Memory) {
        self.is_prefixed_mode = true;
    }
}