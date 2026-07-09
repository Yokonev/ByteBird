use crate::cpu::instructions::arithmetic::*;
use crate::cpu::instructions::load::*;
use crate::cpu::instructions::misc::*;
use crate::cpu::instructions::logic::*;
use crate::cpu::instructions::flag::*;
use crate::cpu::instructions::shift::*;
use crate::cpu::instructions::jump::*;
use crate::cpu::instructions::carry::*;
use crate::cpu::instructions::stack::*;
use crate::cpu::instructions::interrupt::*;
use crate::cpu::regfile::*;
use crate::cpu::tables;
use crate::cpu::tables::alt_optable::initialize_alt_table;
use crate::cpu::tables::normal_optable::initialize_table;
use crate::mem::mem::Memory;
use crate::cpu::tables::{
    alt_optable,
    normal_optable
};

/// Instruction decoder for the ByteBird emulator. Every OpCode used by the original Gameboy is being considered
/// and contained inside a decoding table. If the prefix instruction is detected, the decoded switches to a second table
/// with other instructions in it

pub type OpWrapper = fn(&mut Regfile, &mut Memory) -> u8; //Instruction wrapper, returns the number of cycles used in the process

#[derive(Clone, Copy)]
pub struct OpEntry {
    pub mnemonic: &'static str, //Human-readable representation of the instruction
    pub instruction_length: u8,
    pub exec: OpWrapper
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
    alt: Vec<OpEntry>
}

impl OpTable{

    pub fn new() -> Self {
        Self {
            normal: initialize_table(),
            alt: initialize_alt_table()
        }
    }

    //Decodes an opcode from the normal (unprefixed) table
    pub fn decode(&self, addr: u8) -> &OpEntry {
        &self.normal[addr as usize]
    }

    //Decodes an opcode from the alt (0xCB-prefixed) table
    pub fn decode_cb(&self, addr: u8) -> &OpEntry {
        &self.alt[addr as usize]
    }
}
