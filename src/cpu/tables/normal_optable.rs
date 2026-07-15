use crate::cpu::decoder::OpEntry;
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
use crate::mem::mem::Memory;

const OP_TABLE_SIZE: usize = 0x100;

//Initializes the normal operation table
//NOTE: 11 opcodes (0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD) are unused/illegal
//on the DMG and are intentionally left as OpEntry::default(), which panics if ever executed.
pub fn initialize_table() -> Vec<OpEntry> {
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

    table[0x07] = OpEntry { 
        mnemonic: "RLCA",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlca(regfile)
        }
    };

    table[0x08] = OpEntry { 
        mnemonic: "LD (n16), SP",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let destination: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_load_n16_sp(regfile, destination)
        }
    };

    table[0x09] = OpEntry { 
        mnemonic: "ADD HL, BC",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_hl_r16(regfile, DmgDoubleRegisters::BC)
        }
    };

    table[0x0A] = OpEntry { 
        mnemonic: "LD A, (BC)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_acc_r16(regfile, DmgDoubleRegisters::BC)
        }
    };

    table[0x0B] = OpEntry { 
        mnemonic: "DEC BC",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r16(regfile, DmgDoubleRegisters::BC)
        }
    };

    table[0x0C] = OpEntry { 
        mnemonic: "INC C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x0D] = OpEntry { 
        mnemonic: "DEC C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x0E] = OpEntry { 
        mnemonic: "LD C, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_r8_n8(regfile, DmgSimpleRegisters::C, value)
        }
    };

    table[0x0F] = OpEntry { 
        mnemonic: "RRCA",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrca(regfile)
        }
    };

    table[0x10] = OpEntry { 
        mnemonic: "STOP",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_stop(regfile)
        }
    };

    table[0x11] = OpEntry { 
        mnemonic: "LD DE, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let source: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_load_r16_n16(regfile, DmgDoubleRegisters::DE, source)
        }
    };

    table[0x12] = OpEntry { 
        mnemonic: "LD (DE), A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r16_acc(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0x13] = OpEntry { 
        mnemonic: "INC DE",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r16(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0x14] = OpEntry { 
        mnemonic: "INC D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x15] = OpEntry { 
        mnemonic: "DEC D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x16] = OpEntry { 
        mnemonic: "LD D, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_r8_n8(regfile, DmgSimpleRegisters::D, value)
        }
    };

    table[0x17] = OpEntry { 
        mnemonic: "RLA",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rla(regfile)
        }
    };

    table[0x18] = OpEntry { 
        mnemonic: "JR e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            let target: u16 = (regfile.read_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            op_jr_n16(regfile, target)
        }
    };

    table[0x19] = OpEntry { 
        mnemonic: "ADD HL, DE",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_hl_r16(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0x1A] = OpEntry { 
        mnemonic: "LD A, (DE)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_acc_r16(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0x1B] = OpEntry { 
        mnemonic: "DEC DE",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r16(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0x1C] = OpEntry { 
        mnemonic: "INC E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x1D] = OpEntry { 
        mnemonic: "DEC E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x1E] = OpEntry { 
        mnemonic: "LD E, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_r8_n8(regfile, DmgSimpleRegisters::E, value)
        }
    };

    table[0x1F] = OpEntry { 
        mnemonic: "RRA",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rra(regfile)
        }
    };

    table[0x20] = OpEntry { 
        mnemonic: "JR NZ, e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            let target: u16 = (regfile.read_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            op_jr_cc_n16(regfile, target);
            8u8 //placeholder: real cycles are 8t (not taken) / 12t (taken) once op_jr_cc_n16 implements the flag check
        }
    };

    table[0x21] = OpEntry { 
        mnemonic: "LD HL, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let source: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_load_r16_n16(regfile, DmgDoubleRegisters::HL, source)
        }
    };

    table[0x22] = OpEntry { 
        mnemonic: "LD (HL+), A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hli_acc(regfile)
        }
    };

    table[0x23] = OpEntry { 
        mnemonic: "INC HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r16(regfile, DmgDoubleRegisters::HL)
        }
    };

    table[0x24] = OpEntry { 
        mnemonic: "INC H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x25] = OpEntry { 
        mnemonic: "DEC H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x26] = OpEntry { 
        mnemonic: "LD H, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_r8_n8(regfile, DmgSimpleRegisters::H, value)
        }
    };

    table[0x27] = OpEntry { 
        mnemonic: "DAA",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_daa(regfile)
        }
    };

    table[0x28] = OpEntry { 
        mnemonic: "JR Z, e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            let target: u16 = (regfile.read_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            op_jr_cc_n16(regfile, target);
            8u8 //placeholder: real cycles are 8t (not taken) / 12t (taken) once op_jr_cc_n16 implements the flag check
        }
    };

    table[0x29] = OpEntry { 
        mnemonic: "ADD HL, HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_hl_r16(regfile, DmgDoubleRegisters::HL)
        }
    };

    table[0x2A] = OpEntry { 
        mnemonic: "LD A, (HL+)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_acc_hli(regfile)
        }
    };

    table[0x2B] = OpEntry { 
        mnemonic: "DEC HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r16(regfile, DmgDoubleRegisters::HL)
        }
    };

    table[0x2C] = OpEntry { 
        mnemonic: "INC L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x2D] = OpEntry { 
        mnemonic: "DEC L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x2E] = OpEntry { 
        mnemonic: "LD L, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_r8_n8(regfile, DmgSimpleRegisters::L, value)
        }
    };

    table[0x2F] = OpEntry { 
        mnemonic: "CPL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cpl(regfile)
        }
    };

    table[0x30] = OpEntry { 
        mnemonic: "JR NC, e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            let target: u16 = (regfile.read_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            op_jr_cc_n16(regfile, target);
            8u8 //placeholder: real cycles are 8t (not taken) / 12t (taken) once op_jr_cc_n16 implements the flag check
        }
    };

    table[0x31] = OpEntry { 
        mnemonic: "LD SP, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let source: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_load_sp_n16(regfile, source)
        }
    };

    table[0x32] = OpEntry { 
        mnemonic: "LD (HL-), A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hld_acc(regfile)
        }
    };

    table[0x33] = OpEntry { 
        mnemonic: "INC SP",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_sp(regfile)
        }
    };

    table[0x34] = OpEntry { 
        mnemonic: "INC (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_hl(regfile, memory)
        }
    };

    table[0x35] = OpEntry { 
        mnemonic: "DEC (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_hl(regfile, memory)
        }
    };

    table[0x36] = OpEntry { 
        mnemonic: "LD (HL), u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_hl_n8(regfile, value)
        }
    };

    table[0x37] = OpEntry { 
        mnemonic: "SCF",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_scf(regfile)
        }
    };

    table[0x38] = OpEntry { 
        mnemonic: "JR C, e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            let target: u16 = (regfile.read_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            op_jr_cc_n16(regfile, target);
            8u8 //placeholder: real cycles are 8t (not taken) / 12t (taken) once op_jr_cc_n16 implements the flag check
        }
    };

    table[0x39] = OpEntry { 
        mnemonic: "ADD HL, SP",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_hl_sp(regfile)
        }
    };

    table[0x3A] = OpEntry { 
        mnemonic: "LD A, (HL-)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_acc_hld(regfile)
        }
    };

    table[0x3B] = OpEntry { 
        mnemonic: "DEC SP",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_sp(regfile)
        }
    };

    table[0x3C] = OpEntry { 
        mnemonic: "INC A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_inc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x3D] = OpEntry { 
        mnemonic: "DEC A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_dec_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x3E] = OpEntry { 
        mnemonic: "LD A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_load_r8_n8(regfile, DmgSimpleRegisters::A, value)
        }
    };

    table[0x3F] = OpEntry { 
        mnemonic: "CCF",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ccf(regfile)
        }
    };

    table[0x40] = OpEntry { 
        mnemonic: "LD B, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::B)
        }
    };

    table[0x41] = OpEntry { 
        mnemonic: "LD B, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::C)
        }
    };

    table[0x42] = OpEntry { 
        mnemonic: "LD B, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::D)
        }
    };

    table[0x43] = OpEntry { 
        mnemonic: "LD B, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::E)
        }
    };

    table[0x44] = OpEntry { 
        mnemonic: "LD B, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::H)
        }
    };

    table[0x45] = OpEntry { 
        mnemonic: "LD B, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::L)
        }
    };

    table[0x46] = OpEntry { 
        mnemonic: "LD B, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x47] = OpEntry { 
        mnemonic: "LD B, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::B, DmgSimpleRegisters::A)
        }
    };

    table[0x48] = OpEntry { 
        mnemonic: "LD C, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::B)
        }
    };

    table[0x49] = OpEntry { 
        mnemonic: "LD C, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::C)
        }
    };

    table[0x4A] = OpEntry { 
        mnemonic: "LD C, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::D)
        }
    };

    table[0x4B] = OpEntry { 
        mnemonic: "LD C, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::E)
        }
    };

    table[0x4C] = OpEntry { 
        mnemonic: "LD C, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::H)
        }
    };

    table[0x4D] = OpEntry { 
        mnemonic: "LD C, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::L)
        }
    };

    table[0x4E] = OpEntry { 
        mnemonic: "LD C, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x4F] = OpEntry { 
        mnemonic: "LD C, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::C, DmgSimpleRegisters::A)
        }
    };

    table[0x50] = OpEntry { 
        mnemonic: "LD D, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::B)
        }
    };

    table[0x51] = OpEntry { 
        mnemonic: "LD D, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::C)
        }
    };

    table[0x52] = OpEntry { 
        mnemonic: "LD D, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::D)
        }
    };

    table[0x53] = OpEntry { 
        mnemonic: "LD D, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::E)
        }
    };

    table[0x54] = OpEntry { 
        mnemonic: "LD D, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::H)
        }
    };

    table[0x55] = OpEntry { 
        mnemonic: "LD D, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::L)
        }
    };

    table[0x56] = OpEntry { 
        mnemonic: "LD D, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x57] = OpEntry { 
        mnemonic: "LD D, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::D, DmgSimpleRegisters::A)
        }
    };

    table[0x58] = OpEntry { 
        mnemonic: "LD E, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::B)
        }
    };

    table[0x59] = OpEntry { 
        mnemonic: "LD E, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::C)
        }
    };

    table[0x5A] = OpEntry { 
        mnemonic: "LD E, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::D)
        }
    };

    table[0x5B] = OpEntry { 
        mnemonic: "LD E, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::E)
        }
    };

    table[0x5C] = OpEntry { 
        mnemonic: "LD E, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::H)
        }
    };

    table[0x5D] = OpEntry { 
        mnemonic: "LD E, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::L)
        }
    };

    table[0x5E] = OpEntry { 
        mnemonic: "LD E, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x5F] = OpEntry { 
        mnemonic: "LD E, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::E, DmgSimpleRegisters::A)
        }
    };

    table[0x60] = OpEntry { 
        mnemonic: "LD H, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::B)
        }
    };

    table[0x61] = OpEntry { 
        mnemonic: "LD H, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::C)
        }
    };

    table[0x62] = OpEntry { 
        mnemonic: "LD H, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::D)
        }
    };

    table[0x63] = OpEntry { 
        mnemonic: "LD H, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::E)
        }
    };

    table[0x64] = OpEntry { 
        mnemonic: "LD H, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::H)
        }
    };

    table[0x65] = OpEntry { 
        mnemonic: "LD H, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::L)
        }
    };

    table[0x66] = OpEntry { 
        mnemonic: "LD H, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x67] = OpEntry { 
        mnemonic: "LD H, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::H, DmgSimpleRegisters::A)
        }
    };

    table[0x68] = OpEntry { 
        mnemonic: "LD L, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::B)
        }
    };

    table[0x69] = OpEntry { 
        mnemonic: "LD L, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::C)
        }
    };

    table[0x6A] = OpEntry { 
        mnemonic: "LD L, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::D)
        }
    };

    table[0x6B] = OpEntry { 
        mnemonic: "LD L, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::E)
        }
    };

    table[0x6C] = OpEntry { 
        mnemonic: "LD L, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::H)
        }
    };

    table[0x6D] = OpEntry { 
        mnemonic: "LD L, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::L)
        }
    };

    table[0x6E] = OpEntry { 
        mnemonic: "LD L, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x6F] = OpEntry { 
        mnemonic: "LD L, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::L, DmgSimpleRegisters::A)
        }
    };

    table[0x70] = OpEntry { 
        mnemonic: "LD (HL), B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x71] = OpEntry { 
        mnemonic: "LD (HL), C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x72] = OpEntry { 
        mnemonic: "LD (HL), D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x73] = OpEntry { 
        mnemonic: "LD (HL), E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x74] = OpEntry { 
        mnemonic: "LD (HL), H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x75] = OpEntry { 
        mnemonic: "LD (HL), L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x76] = OpEntry { 
        mnemonic: "HALT",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_halt(regfile)
        }
    };

    table[0x77] = OpEntry { 
        mnemonic: "LD (HL), A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_hl_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x78] = OpEntry { 
        mnemonic: "LD A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::B)
        }
    };

    table[0x79] = OpEntry { 
        mnemonic: "LD A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::C)
        }
    };

    table[0x7A] = OpEntry { 
        mnemonic: "LD A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::D)
        }
    };

    table[0x7B] = OpEntry { 
        mnemonic: "LD A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::E)
        }
    };

    table[0x7C] = OpEntry { 
        mnemonic: "LD A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::H)
        }
    };

    table[0x7D] = OpEntry { 
        mnemonic: "LD A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::L)
        }
    };

    table[0x7E] = OpEntry { 
        mnemonic: "LD A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_hl(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x7F] = OpEntry { 
        mnemonic: "LD A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_r8_r8(regfile, DmgSimpleRegisters::A, DmgSimpleRegisters::A)
        }
    };

    table[0x80] = OpEntry { 
        mnemonic: "ADD A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x81] = OpEntry { 
        mnemonic: "ADD A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x82] = OpEntry { 
        mnemonic: "ADD A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x83] = OpEntry { 
        mnemonic: "ADD A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x84] = OpEntry { 
        mnemonic: "ADD A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x85] = OpEntry { 
        mnemonic: "ADD A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x86] = OpEntry { 
        mnemonic: "ADD A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_hl(regfile, memory)
        }
    };

    table[0x87] = OpEntry { 
        mnemonic: "ADD A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_add_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x88] = OpEntry { 
        mnemonic: "ADC A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x89] = OpEntry { 
        mnemonic: "ADC A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x8A] = OpEntry { 
        mnemonic: "ADC A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x8B] = OpEntry { 
        mnemonic: "ADC A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x8C] = OpEntry { 
        mnemonic: "ADC A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x8D] = OpEntry { 
        mnemonic: "ADC A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x8E] = OpEntry { 
        mnemonic: "ADC A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_hl(regfile, memory)
        }
    };

    table[0x8F] = OpEntry { 
        mnemonic: "ADC A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_adc_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x90] = OpEntry { 
        mnemonic: "SUB A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x91] = OpEntry { 
        mnemonic: "SUB A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x92] = OpEntry { 
        mnemonic: "SUB A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x93] = OpEntry { 
        mnemonic: "SUB A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x94] = OpEntry { 
        mnemonic: "SUB A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x95] = OpEntry { 
        mnemonic: "SUB A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x96] = OpEntry { 
        mnemonic: "SUB A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_hl(regfile, memory)
        }
    };

    table[0x97] = OpEntry { 
        mnemonic: "SUB A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sub_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x98] = OpEntry { 
        mnemonic: "SBC A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x99] = OpEntry { 
        mnemonic: "SBC A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x9A] = OpEntry { 
        mnemonic: "SBC A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x9B] = OpEntry { 
        mnemonic: "SBC A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x9C] = OpEntry { 
        mnemonic: "SBC A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x9D] = OpEntry { 
        mnemonic: "SBC A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x9E] = OpEntry { 
        mnemonic: "SBC A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_hl(regfile, memory)
        }
    };

    table[0x9F] = OpEntry { 
        mnemonic: "SBC A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sbc_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0xA0] = OpEntry { 
        mnemonic: "AND A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0xA1] = OpEntry { 
        mnemonic: "AND A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0xA2] = OpEntry { 
        mnemonic: "AND A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0xA3] = OpEntry { 
        mnemonic: "AND A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0xA4] = OpEntry { 
        mnemonic: "AND A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0xA5] = OpEntry { 
        mnemonic: "AND A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0xA6] = OpEntry { 
        mnemonic: "AND A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_hl(regfile)
        }
    };

    table[0xA7] = OpEntry { 
        mnemonic: "AND A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_and_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0xA8] = OpEntry { 
        mnemonic: "XOR A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0xA9] = OpEntry { 
        mnemonic: "XOR A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0xAA] = OpEntry { 
        mnemonic: "XOR A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0xAB] = OpEntry { 
        mnemonic: "XOR A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0xAC] = OpEntry { 
        mnemonic: "XOR A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0xAD] = OpEntry { 
        mnemonic: "XOR A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0xAE] = OpEntry { 
        mnemonic: "XOR A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_hl(regfile)
        }
    };

    table[0xAF] = OpEntry { 
        mnemonic: "XOR A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_xor_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0xB0] = OpEntry { 
        mnemonic: "OR A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0xB1] = OpEntry { 
        mnemonic: "OR A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0xB2] = OpEntry { 
        mnemonic: "OR A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0xB3] = OpEntry { 
        mnemonic: "OR A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0xB4] = OpEntry { 
        mnemonic: "OR A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0xB5] = OpEntry { 
        mnemonic: "OR A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0xB6] = OpEntry { 
        mnemonic: "OR A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_hl(regfile, DmgSimpleRegisters::A) //NOTE: op_or_acc_hl takes an unused source param that looks like a copy-paste leftover; passing a dummy value
        }
    };

    table[0xB7] = OpEntry { 
        mnemonic: "OR A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_or_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0xB8] = OpEntry { 
        mnemonic: "CP A, B",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0xB9] = OpEntry { 
        mnemonic: "CP A, C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0xBA] = OpEntry { 
        mnemonic: "CP A, D",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0xBB] = OpEntry { 
        mnemonic: "CP A, E",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0xBC] = OpEntry { 
        mnemonic: "CP A, H",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0xBD] = OpEntry { 
        mnemonic: "CP A, L",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0xBE] = OpEntry { 
        mnemonic: "CP A, (HL)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_hl(regfile, memory)
        }
    };

    table[0xBF] = OpEntry { 
        mnemonic: "CP A, A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_cp_acc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0xC0] = OpEntry { 
        mnemonic: "RET NZ",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ret_cc(regfile, 0u8);
            8u8 //placeholder: real cycles are 8t (not taken) / 20t (taken) once op_ret_cc implements the flag check
        }
    };

    table[0xC1] = OpEntry { 
        mnemonic: "POP BC",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_pop_r16(regfile, DmgDoubleRegisters::BC)
        }
    };

    table[0xC2] = OpEntry { 
        mnemonic: "JP NZ, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_jmp_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 16t (taken) once op_jmp_cc_n16 implements the flag check
        }
    };

    table[0xC3] = OpEntry { 
        mnemonic: "JP u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_jmp_n16(regfile, addr)
        }
    };

    table[0xC4] = OpEntry { 
        mnemonic: "CALL NZ, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_call_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 24t (taken) once op_call_cc_n16 implements the flag check
        }
    };

    table[0xC5] = OpEntry { 
        mnemonic: "PUSH BC",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_push_r16(regfile, DmgDoubleRegisters::BC)
        }
    };

    table[0xC6] = OpEntry { 
        mnemonic: "ADD A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_add_acc_n8(regfile, value)
        }
    };

    table[0xC7] = OpEntry { 
        mnemonic: "RST 00h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x00u8)
        }
    };

    table[0xC8] = OpEntry { 
        mnemonic: "RET Z",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ret_cc(regfile, 1u8);
            8u8 //placeholder: real cycles are 8t (not taken) / 20t (taken) once op_ret_cc implements the flag check
        }
    };

    table[0xC9] = OpEntry { 
        mnemonic: "RET",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ret(regfile)
        }
    };

    table[0xCA] = OpEntry { 
        mnemonic: "JP Z, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_jmp_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 16t (taken) once op_jmp_cc_n16 implements the flag check
        }
    };

    table[0xCB] = OpEntry { 
        mnemonic: "PREFIX CB",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            4u8 //unreachable: Cpu::step() special-cases 0xCB before reaching this table
        }
    };

    table[0xCC] = OpEntry { 
        mnemonic: "CALL Z, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_call_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 24t (taken) once op_call_cc_n16 implements the flag check
        }
    };

    table[0xCD] = OpEntry { 
        mnemonic: "CALL u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_call_n16(regfile, addr)
        }
    };

    table[0xCE] = OpEntry { 
        mnemonic: "ADC A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_adc_acc_n8(regfile, value)
        }
    };

    table[0xCF] = OpEntry { 
        mnemonic: "RST 08h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x08u8)
        }
    };

    table[0xD0] = OpEntry { 
        mnemonic: "RET NC",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ret_cc(regfile, 2u8);
            8u8 //placeholder: real cycles are 8t (not taken) / 20t (taken) once op_ret_cc implements the flag check
        }
    };

    table[0xD1] = OpEntry { 
        mnemonic: "POP DE",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_pop_r16(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0xD2] = OpEntry { 
        mnemonic: "JP NC, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_jmp_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 16t (taken) once op_jmp_cc_n16 implements the flag check
        }
    };

    table[0xD4] = OpEntry { 
        mnemonic: "CALL NC, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_call_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 24t (taken) once op_call_cc_n16 implements the flag check
        }
    };

    table[0xD5] = OpEntry { 
        mnemonic: "PUSH DE",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_push_r16(regfile, DmgDoubleRegisters::DE)
        }
    };

    table[0xD6] = OpEntry { 
        mnemonic: "SUB A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_sub_acc_n8(regfile, value)
        }
    };

    table[0xD7] = OpEntry { 
        mnemonic: "RST 10h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x10u8)
        }
    };

    table[0xD8] = OpEntry { 
        mnemonic: "RET C",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ret_cc(regfile, 3u8);
            8u8 //placeholder: real cycles are 8t (not taken) / 20t (taken) once op_ret_cc implements the flag check
        }
    };

    table[0xD9] = OpEntry { 
        mnemonic: "RETI",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_reti(regfile)
        }
    };

    table[0xDA] = OpEntry { 
        mnemonic: "JP C, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_jmp_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 16t (taken) once op_jmp_cc_n16 implements the flag check
        }
    };

    table[0xDC] = OpEntry { 
        mnemonic: "CALL C, u16",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let addr: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_call_cc_n16(regfile, addr);
            12u8 //placeholder: real cycles are 12t (not taken) / 24t (taken) once op_call_cc_n16 implements the flag check
        }
    };

    table[0xDE] = OpEntry { 
        mnemonic: "SBC A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_sbc_acc_n8(regfile, value)
        }
    };

    table[0xDF] = OpEntry { 
        mnemonic: "RST 18h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x18u8)
        }
    };

    table[0xE0] = OpEntry { 
        mnemonic: "LDH (n16), A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            let destination: u16 = 0xFF00 + offset as u16;
            op_load_high_n16_acc(regfile, destination)
        }
    };

    table[0xE1] = OpEntry { 
        mnemonic: "POP HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_pop_r16(regfile, DmgDoubleRegisters::HL)
        }
    };

    table[0xE2] = OpEntry { 
        mnemonic: "LDH (C), A",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_high_c_acc(regfile)
        }
    };

    table[0xE5] = OpEntry { 
        mnemonic: "PUSH HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_push_r16(regfile, DmgDoubleRegisters::HL)
        }
    };

    table[0xE6] = OpEntry { 
        mnemonic: "AND A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_and_acc_n8(regfile, value)
        }
    };

    table[0xE7] = OpEntry { 
        mnemonic: "RST 20h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x20u8)
        }
    };

    table[0xE8] = OpEntry { 
        mnemonic: "ADD SP, e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            op_add_sp_e8(regfile, offset)
        }
    };

    table[0xE9] = OpEntry { 
        mnemonic: "JP HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_jmp_hl(regfile)
        }
    };

    table[0xEA] = OpEntry { 
        mnemonic: "LD (n16), A",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let destination: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_load_n16_acc(regfile, destination)
        }
    };

    table[0xEE] = OpEntry { 
        mnemonic: "XOR A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_xor_acc_n8(regfile, value)
        }
    };

    table[0xEF] = OpEntry { 
        mnemonic: "RST 28h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x28u8)
        }
    };

    table[0xF0] = OpEntry { 
        mnemonic: "LDH A, (n16)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_high_acc_n16(regfile) //NOTE: op_load_high_acc_n16 has no address parameter to receive the u8 operand read from memory
        }
    };

    table[0xF1] = OpEntry { 
        mnemonic: "POP AF",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_pop_af(regfile)
        }
    };

    table[0xF2] = OpEntry { 
        mnemonic: "LDH A, (C)",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_high_acc_c(regfile)
        }
    };

    table[0xF3] = OpEntry { 
        mnemonic: "DI",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_di(regfile)
        }
    };

    table[0xF5] = OpEntry { 
        mnemonic: "PUSH AF",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_push_af(regfile)
        }
    };

    table[0xF6] = OpEntry { 
        mnemonic: "OR A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_or_acc_n8(regfile, value)
        }
    };

    table[0xF7] = OpEntry { 
        mnemonic: "RST 30h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x30u8)
        }
    };

    table[0xF8] = OpEntry { 
        mnemonic: "LD HL, SP+e8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let offset: i8 = memory.read_mem_8(regfile.read_pc() + 1) as i8;
            op_load_hl_spe(regfile, offset)
        }
    };

    table[0xF9] = OpEntry { 
        mnemonic: "LD SP, HL",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_load_sp_hl(regfile)
        }
    };

    table[0xFA] = OpEntry { 
        mnemonic: "LD A, (n16)",
        instruction_length: 3,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let source: u16 = memory.read_mem_16(regfile.read_pc() + 1);
            op_load_acc_n16(regfile, source)
        }
    };

    table[0xFB] = OpEntry { 
        mnemonic: "EI",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_ei(regfile)
        }
    };

    table[0xFE] = OpEntry { 
        mnemonic: "CP A, u8",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            let value: u8 = memory.read_mem_8(regfile.read_pc() + 1);
            op_cp_acc_n8(regfile, value)
        }
    };

    table[0xFF] = OpEntry { 
        mnemonic: "RST 38h",
        instruction_length: 1,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rst_vec(regfile, 0x38u8)
        }
    };


    table
}