use crate::cpu::instructions::arithmetic::*;
use crate::cpu::instructions::load::*;
use crate::cpu::instructions::misc::*;
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

    fn prefix_op(&mut self, _regfile: &mut Regfile, _memory: &mut Memory) {
        self.is_prefixed_mode = true;
    }

    //Initializes the normal operation table
    fn initialize_table() -> Vec<OpEntry> {
        let mut table = vec![OpEntry::default(); OP_TABLE_SIZE];

        table[0x00] = OpEntry { 
            mnemonic: "NOP",
            instruction_length: 1, 
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 { op_nop() } 
        };

        table[0x01] = OpEntry { 
            mnemonic: "LD BC, u16", 
            instruction_length: 3, 
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 { 
                let source: u16 = memory.read_mem_16(regfile.read_pc() + 1);
                let destination = DmgDoubleRegisters::BC;
                op_load_r16_n16(regfile, destination, source)
            } 
        };

        table[0x02] = OpEntry { 
            mnemonic: "LD (BC), A", 
            instruction_length: 1, 
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
                op_load_r16_acc(regfile, DmgDoubleRegisters::BC)
            }
        };

        table[0x03] = OpEntry { 
            mnemonic: "INC BC",
            instruction_length: 1,
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
                op_inc_r16(regfile, DmgDoubleRegisters::BC)
            }
        };

        table[0x04] = OpEntry { 
            mnemonic: "INC B",
            instruction_length: 1,
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
                op_inc_r8(regfile, DmgSimpleRegisters::B)
            }
        };

        table[0x05] = OpEntry { 
            mnemonic: "DEC B",
            instruction_length: 1,
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
                op_dec_r8(regfile, DmgSimpleRegisters::B)
            }
        };

        table[0x06] = OpEntry { 
            mnemonic: "LD B, u8",
            instruction_length: 2,
            exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
                let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
                op_load_r8_n8(regfile, DmgSimpleRegisters::B, value)
            }
        };

        table
    }

    //Initializes the alt operation table (prefixed)
    fn initialize_alt_table() -> Vec<OpEntry> {
        vec![OpEntry::default(); OP_TABLE_SIZE]
    }
}