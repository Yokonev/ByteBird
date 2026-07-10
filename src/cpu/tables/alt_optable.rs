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

//Initializes the alt operation table (prefixed, reached via the 0xCB byte)
pub fn initialize_alt_table() -> Vec<OpEntry> {
    let mut table = vec![OpEntry::default(); OP_TABLE_SIZE];

    table[0x00] = OpEntry { 
        mnemonic: "RLC B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x01] = OpEntry { 
        mnemonic: "RLC C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x02] = OpEntry { 
        mnemonic: "RLC D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x03] = OpEntry { 
        mnemonic: "RLC E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x04] = OpEntry { 
        mnemonic: "RLC H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x05] = OpEntry { 
        mnemonic: "RLC L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x06] = OpEntry { 
        mnemonic: "RLC (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_hl(regfile)
        }
    };

    table[0x07] = OpEntry { 
        mnemonic: "RLC A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rlc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x08] = OpEntry { 
        mnemonic: "RRC B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x09] = OpEntry { 
        mnemonic: "RRC C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x0A] = OpEntry { 
        mnemonic: "RRC D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x0B] = OpEntry { 
        mnemonic: "RRC E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x0C] = OpEntry { 
        mnemonic: "RRC H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x0D] = OpEntry { 
        mnemonic: "RRC L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x0E] = OpEntry { 
        mnemonic: "RRC (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_hl(regfile)
        }
    };

    table[0x0F] = OpEntry { 
        mnemonic: "RRC A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rrc_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x10] = OpEntry { 
        mnemonic: "RL B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x11] = OpEntry { 
        mnemonic: "RL C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x12] = OpEntry { 
        mnemonic: "RL D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x13] = OpEntry { 
        mnemonic: "RL E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x14] = OpEntry { 
        mnemonic: "RL H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x15] = OpEntry { 
        mnemonic: "RL L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x16] = OpEntry { 
        mnemonic: "RL (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_hl(regfile)
        }
    };

    table[0x17] = OpEntry { 
        mnemonic: "RL A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rl_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x18] = OpEntry { 
        mnemonic: "RR B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x19] = OpEntry { 
        mnemonic: "RR C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x1A] = OpEntry { 
        mnemonic: "RR D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x1B] = OpEntry { 
        mnemonic: "RR E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x1C] = OpEntry { 
        mnemonic: "RR H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x1D] = OpEntry { 
        mnemonic: "RR L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x1E] = OpEntry { 
        mnemonic: "RR (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_hl(regfile)
        }
    };

    table[0x1F] = OpEntry { 
        mnemonic: "RR A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_rr_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x20] = OpEntry { 
        mnemonic: "SLA B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x21] = OpEntry { 
        mnemonic: "SLA C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x22] = OpEntry { 
        mnemonic: "SLA D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x23] = OpEntry { 
        mnemonic: "SLA E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x24] = OpEntry { 
        mnemonic: "SLA H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x25] = OpEntry { 
        mnemonic: "SLA L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x26] = OpEntry { 
        mnemonic: "SLA (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_hl(regfile)
        }
    };

    table[0x27] = OpEntry { 
        mnemonic: "SLA A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sla_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x28] = OpEntry { 
        mnemonic: "SRA B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x29] = OpEntry { 
        mnemonic: "SRA C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x2A] = OpEntry { 
        mnemonic: "SRA D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x2B] = OpEntry { 
        mnemonic: "SRA E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x2C] = OpEntry { 
        mnemonic: "SRA H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x2D] = OpEntry { 
        mnemonic: "SRA L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x2E] = OpEntry { 
        mnemonic: "SRA (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_hl(regfile)
        }
    };

    table[0x2F] = OpEntry { 
        mnemonic: "SRA A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_sra_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x30] = OpEntry { 
        mnemonic: "SWAP B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x31] = OpEntry { 
        mnemonic: "SWAP C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x32] = OpEntry { 
        mnemonic: "SWAP D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x33] = OpEntry { 
        mnemonic: "SWAP E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x34] = OpEntry { 
        mnemonic: "SWAP H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x35] = OpEntry { 
        mnemonic: "SWAP L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x36] = OpEntry { 
        mnemonic: "SWAP (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_hl(regfile)
        }
    };

    table[0x37] = OpEntry { 
        mnemonic: "SWAP A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_swap_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x38] = OpEntry { 
        mnemonic: "SRL B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::B)
        }
    };

    table[0x39] = OpEntry { 
        mnemonic: "SRL C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::C)
        }
    };

    table[0x3A] = OpEntry { 
        mnemonic: "SRL D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::D)
        }
    };

    table[0x3B] = OpEntry { 
        mnemonic: "SRL E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::E)
        }
    };

    table[0x3C] = OpEntry { 
        mnemonic: "SRL H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::H)
        }
    };

    table[0x3D] = OpEntry { 
        mnemonic: "SRL L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::L)
        }
    };

    table[0x3E] = OpEntry { 
        mnemonic: "SRL (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_hl(regfile)
        }
    };

    table[0x3F] = OpEntry { 
        mnemonic: "SRL A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_srl_r8(regfile, DmgSimpleRegisters::A)
        }
    };

    table[0x40] = OpEntry { 
        mnemonic: "BIT 0, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::B)
        }
    };

    table[0x41] = OpEntry { 
        mnemonic: "BIT 0, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::C)
        }
    };

    table[0x42] = OpEntry { 
        mnemonic: "BIT 0, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::D)
        }
    };

    table[0x43] = OpEntry { 
        mnemonic: "BIT 0, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::E)
        }
    };

    table[0x44] = OpEntry { 
        mnemonic: "BIT 0, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::H)
        }
    };

    table[0x45] = OpEntry { 
        mnemonic: "BIT 0, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::L)
        }
    };

    table[0x46] = OpEntry { 
        mnemonic: "BIT 0, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 0u8)
        }
    };

    table[0x47] = OpEntry { 
        mnemonic: "BIT 0, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 0u8, DmgSimpleRegisters::A)
        }
    };

    table[0x48] = OpEntry { 
        mnemonic: "BIT 1, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::B)
        }
    };

    table[0x49] = OpEntry { 
        mnemonic: "BIT 1, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::C)
        }
    };

    table[0x4A] = OpEntry { 
        mnemonic: "BIT 1, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::D)
        }
    };

    table[0x4B] = OpEntry { 
        mnemonic: "BIT 1, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::E)
        }
    };

    table[0x4C] = OpEntry { 
        mnemonic: "BIT 1, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::H)
        }
    };

    table[0x4D] = OpEntry { 
        mnemonic: "BIT 1, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::L)
        }
    };

    table[0x4E] = OpEntry { 
        mnemonic: "BIT 1, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 1u8)
        }
    };

    table[0x4F] = OpEntry { 
        mnemonic: "BIT 1, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 1u8, DmgSimpleRegisters::A)
        }
    };

    table[0x50] = OpEntry { 
        mnemonic: "BIT 2, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::B)
        }
    };

    table[0x51] = OpEntry { 
        mnemonic: "BIT 2, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::C)
        }
    };

    table[0x52] = OpEntry { 
        mnemonic: "BIT 2, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::D)
        }
    };

    table[0x53] = OpEntry { 
        mnemonic: "BIT 2, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::E)
        }
    };

    table[0x54] = OpEntry { 
        mnemonic: "BIT 2, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::H)
        }
    };

    table[0x55] = OpEntry { 
        mnemonic: "BIT 2, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::L)
        }
    };

    table[0x56] = OpEntry { 
        mnemonic: "BIT 2, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 2u8)
        }
    };

    table[0x57] = OpEntry { 
        mnemonic: "BIT 2, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 2u8, DmgSimpleRegisters::A)
        }
    };

    table[0x58] = OpEntry { 
        mnemonic: "BIT 3, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::B)
        }
    };

    table[0x59] = OpEntry { 
        mnemonic: "BIT 3, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::C)
        }
    };

    table[0x5A] = OpEntry { 
        mnemonic: "BIT 3, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::D)
        }
    };

    table[0x5B] = OpEntry { 
        mnemonic: "BIT 3, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::E)
        }
    };

    table[0x5C] = OpEntry { 
        mnemonic: "BIT 3, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::H)
        }
    };

    table[0x5D] = OpEntry { 
        mnemonic: "BIT 3, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::L)
        }
    };

    table[0x5E] = OpEntry { 
        mnemonic: "BIT 3, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 3u8)
        }
    };

    table[0x5F] = OpEntry { 
        mnemonic: "BIT 3, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 3u8, DmgSimpleRegisters::A)
        }
    };

    table[0x60] = OpEntry { 
        mnemonic: "BIT 4, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::B)
        }
    };

    table[0x61] = OpEntry { 
        mnemonic: "BIT 4, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::C)
        }
    };

    table[0x62] = OpEntry { 
        mnemonic: "BIT 4, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::D)
        }
    };

    table[0x63] = OpEntry { 
        mnemonic: "BIT 4, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::E)
        }
    };

    table[0x64] = OpEntry { 
        mnemonic: "BIT 4, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::H)
        }
    };

    table[0x65] = OpEntry { 
        mnemonic: "BIT 4, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::L)
        }
    };

    table[0x66] = OpEntry { 
        mnemonic: "BIT 4, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 4u8)
        }
    };

    table[0x67] = OpEntry { 
        mnemonic: "BIT 4, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 4u8, DmgSimpleRegisters::A)
        }
    };

    table[0x68] = OpEntry { 
        mnemonic: "BIT 5, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::B)
        }
    };

    table[0x69] = OpEntry { 
        mnemonic: "BIT 5, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::C)
        }
    };

    table[0x6A] = OpEntry { 
        mnemonic: "BIT 5, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::D)
        }
    };

    table[0x6B] = OpEntry { 
        mnemonic: "BIT 5, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::E)
        }
    };

    table[0x6C] = OpEntry { 
        mnemonic: "BIT 5, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::H)
        }
    };

    table[0x6D] = OpEntry { 
        mnemonic: "BIT 5, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::L)
        }
    };

    table[0x6E] = OpEntry { 
        mnemonic: "BIT 5, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 5u8)
        }
    };

    table[0x6F] = OpEntry { 
        mnemonic: "BIT 5, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 5u8, DmgSimpleRegisters::A)
        }
    };

    table[0x70] = OpEntry { 
        mnemonic: "BIT 6, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::B)
        }
    };

    table[0x71] = OpEntry { 
        mnemonic: "BIT 6, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::C)
        }
    };

    table[0x72] = OpEntry { 
        mnemonic: "BIT 6, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::D)
        }
    };

    table[0x73] = OpEntry { 
        mnemonic: "BIT 6, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::E)
        }
    };

    table[0x74] = OpEntry { 
        mnemonic: "BIT 6, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::H)
        }
    };

    table[0x75] = OpEntry { 
        mnemonic: "BIT 6, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::L)
        }
    };

    table[0x76] = OpEntry { 
        mnemonic: "BIT 6, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 6u8)
        }
    };

    table[0x77] = OpEntry { 
        mnemonic: "BIT 6, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 6u8, DmgSimpleRegisters::A)
        }
    };

    table[0x78] = OpEntry { 
        mnemonic: "BIT 7, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::B)
        }
    };

    table[0x79] = OpEntry { 
        mnemonic: "BIT 7, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::C)
        }
    };

    table[0x7A] = OpEntry { 
        mnemonic: "BIT 7, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::D)
        }
    };

    table[0x7B] = OpEntry { 
        mnemonic: "BIT 7, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::E)
        }
    };

    table[0x7C] = OpEntry { 
        mnemonic: "BIT 7, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::H)
        }
    };

    table[0x7D] = OpEntry { 
        mnemonic: "BIT 7, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::L)
        }
    };

    table[0x7E] = OpEntry { 
        mnemonic: "BIT 7, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_hl(regfile, 7u8)
        }
    };

    table[0x7F] = OpEntry { 
        mnemonic: "BIT 7, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_bit_u3_r8(regfile, 7u8, DmgSimpleRegisters::A)
        }
    };

    table[0x80] = OpEntry { 
        mnemonic: "RES 0, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::B)
        }
    };

    table[0x81] = OpEntry { 
        mnemonic: "RES 0, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::C)
        }
    };

    table[0x82] = OpEntry { 
        mnemonic: "RES 0, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::D)
        }
    };

    table[0x83] = OpEntry { 
        mnemonic: "RES 0, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::E)
        }
    };

    table[0x84] = OpEntry { 
        mnemonic: "RES 0, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::H)
        }
    };

    table[0x85] = OpEntry { 
        mnemonic: "RES 0, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::L)
        }
    };

    table[0x86] = OpEntry { 
        mnemonic: "RES 0, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 0u8)
        }
    };

    table[0x87] = OpEntry { 
        mnemonic: "RES 0, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 0u8, DmgSimpleRegisters::A)
        }
    };

    table[0x88] = OpEntry { 
        mnemonic: "RES 1, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::B)
        }
    };

    table[0x89] = OpEntry { 
        mnemonic: "RES 1, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::C)
        }
    };

    table[0x8A] = OpEntry { 
        mnemonic: "RES 1, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::D)
        }
    };

    table[0x8B] = OpEntry { 
        mnemonic: "RES 1, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::E)
        }
    };

    table[0x8C] = OpEntry { 
        mnemonic: "RES 1, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::H)
        }
    };

    table[0x8D] = OpEntry { 
        mnemonic: "RES 1, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::L)
        }
    };

    table[0x8E] = OpEntry { 
        mnemonic: "RES 1, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 1u8)
        }
    };

    table[0x8F] = OpEntry { 
        mnemonic: "RES 1, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 1u8, DmgSimpleRegisters::A)
        }
    };

    table[0x90] = OpEntry { 
        mnemonic: "RES 2, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::B)
        }
    };

    table[0x91] = OpEntry { 
        mnemonic: "RES 2, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::C)
        }
    };

    table[0x92] = OpEntry { 
        mnemonic: "RES 2, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::D)
        }
    };

    table[0x93] = OpEntry { 
        mnemonic: "RES 2, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::E)
        }
    };

    table[0x94] = OpEntry { 
        mnemonic: "RES 2, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::H)
        }
    };

    table[0x95] = OpEntry { 
        mnemonic: "RES 2, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::L)
        }
    };

    table[0x96] = OpEntry { 
        mnemonic: "RES 2, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 2u8)
        }
    };

    table[0x97] = OpEntry { 
        mnemonic: "RES 2, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 2u8, DmgSimpleRegisters::A)
        }
    };

    table[0x98] = OpEntry { 
        mnemonic: "RES 3, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::B)
        }
    };

    table[0x99] = OpEntry { 
        mnemonic: "RES 3, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::C)
        }
    };

    table[0x9A] = OpEntry { 
        mnemonic: "RES 3, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::D)
        }
    };

    table[0x9B] = OpEntry { 
        mnemonic: "RES 3, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::E)
        }
    };

    table[0x9C] = OpEntry { 
        mnemonic: "RES 3, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::H)
        }
    };

    table[0x9D] = OpEntry { 
        mnemonic: "RES 3, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::L)
        }
    };

    table[0x9E] = OpEntry { 
        mnemonic: "RES 3, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 3u8)
        }
    };

    table[0x9F] = OpEntry { 
        mnemonic: "RES 3, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 3u8, DmgSimpleRegisters::A)
        }
    };

    table[0xA0] = OpEntry { 
        mnemonic: "RES 4, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::B)
        }
    };

    table[0xA1] = OpEntry { 
        mnemonic: "RES 4, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::C)
        }
    };

    table[0xA2] = OpEntry { 
        mnemonic: "RES 4, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::D)
        }
    };

    table[0xA3] = OpEntry { 
        mnemonic: "RES 4, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::E)
        }
    };

    table[0xA4] = OpEntry { 
        mnemonic: "RES 4, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::H)
        }
    };

    table[0xA5] = OpEntry { 
        mnemonic: "RES 4, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::L)
        }
    };

    table[0xA6] = OpEntry { 
        mnemonic: "RES 4, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 4u8)
        }
    };

    table[0xA7] = OpEntry { 
        mnemonic: "RES 4, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 4u8, DmgSimpleRegisters::A)
        }
    };

    table[0xA8] = OpEntry { 
        mnemonic: "RES 5, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::B)
        }
    };

    table[0xA9] = OpEntry { 
        mnemonic: "RES 5, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::C)
        }
    };

    table[0xAA] = OpEntry { 
        mnemonic: "RES 5, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::D)
        }
    };

    table[0xAB] = OpEntry { 
        mnemonic: "RES 5, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::E)
        }
    };

    table[0xAC] = OpEntry { 
        mnemonic: "RES 5, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::H)
        }
    };

    table[0xAD] = OpEntry { 
        mnemonic: "RES 5, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::L)
        }
    };

    table[0xAE] = OpEntry { 
        mnemonic: "RES 5, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 5u8)
        }
    };

    table[0xAF] = OpEntry { 
        mnemonic: "RES 5, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 5u8, DmgSimpleRegisters::A)
        }
    };

    table[0xB0] = OpEntry { 
        mnemonic: "RES 6, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::B)
        }
    };

    table[0xB1] = OpEntry { 
        mnemonic: "RES 6, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::C)
        }
    };

    table[0xB2] = OpEntry { 
        mnemonic: "RES 6, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::D)
        }
    };

    table[0xB3] = OpEntry { 
        mnemonic: "RES 6, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::E)
        }
    };

    table[0xB4] = OpEntry { 
        mnemonic: "RES 6, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::H)
        }
    };

    table[0xB5] = OpEntry { 
        mnemonic: "RES 6, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::L)
        }
    };

    table[0xB6] = OpEntry { 
        mnemonic: "RES 6, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 6u8)
        }
    };

    table[0xB7] = OpEntry { 
        mnemonic: "RES 6, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 6u8, DmgSimpleRegisters::A)
        }
    };

    table[0xB8] = OpEntry { 
        mnemonic: "RES 7, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::B)
        }
    };

    table[0xB9] = OpEntry { 
        mnemonic: "RES 7, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::C)
        }
    };

    table[0xBA] = OpEntry { 
        mnemonic: "RES 7, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::D)
        }
    };

    table[0xBB] = OpEntry { 
        mnemonic: "RES 7, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::E)
        }
    };

    table[0xBC] = OpEntry { 
        mnemonic: "RES 7, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::H)
        }
    };

    table[0xBD] = OpEntry { 
        mnemonic: "RES 7, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::L)
        }
    };

    table[0xBE] = OpEntry { 
        mnemonic: "RES 7, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_hl(regfile, 7u8)
        }
    };

    table[0xBF] = OpEntry { 
        mnemonic: "RES 7, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_res_u3_r8(regfile, 7u8, DmgSimpleRegisters::A)
        }
    };

    table[0xC0] = OpEntry { 
        mnemonic: "SET 0, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::B)
        }
    };

    table[0xC1] = OpEntry { 
        mnemonic: "SET 0, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::C)
        }
    };

    table[0xC2] = OpEntry { 
        mnemonic: "SET 0, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::D)
        }
    };

    table[0xC3] = OpEntry { 
        mnemonic: "SET 0, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::E)
        }
    };

    table[0xC4] = OpEntry { 
        mnemonic: "SET 0, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::H)
        }
    };

    table[0xC5] = OpEntry { 
        mnemonic: "SET 0, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::L)
        }
    };

    table[0xC6] = OpEntry { 
        mnemonic: "SET 0, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 0u8)
        }
    };

    table[0xC7] = OpEntry { 
        mnemonic: "SET 0, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 0u8, DmgSimpleRegisters::A)
        }
    };

    table[0xC8] = OpEntry { 
        mnemonic: "SET 1, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::B)
        }
    };

    table[0xC9] = OpEntry { 
        mnemonic: "SET 1, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::C)
        }
    };

    table[0xCA] = OpEntry { 
        mnemonic: "SET 1, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::D)
        }
    };

    table[0xCB] = OpEntry { 
        mnemonic: "SET 1, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::E)
        }
    };

    table[0xCC] = OpEntry { 
        mnemonic: "SET 1, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::H)
        }
    };

    table[0xCD] = OpEntry { 
        mnemonic: "SET 1, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::L)
        }
    };

    table[0xCE] = OpEntry { 
        mnemonic: "SET 1, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 1u8)
        }
    };

    table[0xCF] = OpEntry { 
        mnemonic: "SET 1, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 1u8, DmgSimpleRegisters::A)
        }
    };

    table[0xD0] = OpEntry { 
        mnemonic: "SET 2, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::B)
        }
    };

    table[0xD1] = OpEntry { 
        mnemonic: "SET 2, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::C)
        }
    };

    table[0xD2] = OpEntry { 
        mnemonic: "SET 2, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::D)
        }
    };

    table[0xD3] = OpEntry { 
        mnemonic: "SET 2, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::E)
        }
    };

    table[0xD4] = OpEntry { 
        mnemonic: "SET 2, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::H)
        }
    };

    table[0xD5] = OpEntry { 
        mnemonic: "SET 2, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::L)
        }
    };

    table[0xD6] = OpEntry { 
        mnemonic: "SET 2, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 2u8)
        }
    };

    table[0xD7] = OpEntry { 
        mnemonic: "SET 2, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 2u8, DmgSimpleRegisters::A)
        }
    };

    table[0xD8] = OpEntry { 
        mnemonic: "SET 3, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::B)
        }
    };

    table[0xD9] = OpEntry { 
        mnemonic: "SET 3, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::C)
        }
    };

    table[0xDA] = OpEntry { 
        mnemonic: "SET 3, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::D)
        }
    };

    table[0xDB] = OpEntry { 
        mnemonic: "SET 3, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::E)
        }
    };

    table[0xDC] = OpEntry { 
        mnemonic: "SET 3, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::H)
        }
    };

    table[0xDD] = OpEntry { 
        mnemonic: "SET 3, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::L)
        }
    };

    table[0xDE] = OpEntry { 
        mnemonic: "SET 3, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 3u8)
        }
    };

    table[0xDF] = OpEntry { 
        mnemonic: "SET 3, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 3u8, DmgSimpleRegisters::A)
        }
    };

    table[0xE0] = OpEntry { 
        mnemonic: "SET 4, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::B)
        }
    };

    table[0xE1] = OpEntry { 
        mnemonic: "SET 4, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::C)
        }
    };

    table[0xE2] = OpEntry { 
        mnemonic: "SET 4, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::D)
        }
    };

    table[0xE3] = OpEntry { 
        mnemonic: "SET 4, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::E)
        }
    };

    table[0xE4] = OpEntry { 
        mnemonic: "SET 4, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::H)
        }
    };

    table[0xE5] = OpEntry { 
        mnemonic: "SET 4, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::L)
        }
    };

    table[0xE6] = OpEntry { 
        mnemonic: "SET 4, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 4u8)
        }
    };

    table[0xE7] = OpEntry { 
        mnemonic: "SET 4, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 4u8, DmgSimpleRegisters::A)
        }
    };

    table[0xE8] = OpEntry { 
        mnemonic: "SET 5, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::B)
        }
    };

    table[0xE9] = OpEntry { 
        mnemonic: "SET 5, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::C)
        }
    };

    table[0xEA] = OpEntry { 
        mnemonic: "SET 5, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::D)
        }
    };

    table[0xEB] = OpEntry { 
        mnemonic: "SET 5, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::E)
        }
    };

    table[0xEC] = OpEntry { 
        mnemonic: "SET 5, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::H)
        }
    };

    table[0xED] = OpEntry { 
        mnemonic: "SET 5, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::L)
        }
    };

    table[0xEE] = OpEntry { 
        mnemonic: "SET 5, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 5u8)
        }
    };

    table[0xEF] = OpEntry { 
        mnemonic: "SET 5, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 5u8, DmgSimpleRegisters::A)
        }
    };

    table[0xF0] = OpEntry { 
        mnemonic: "SET 6, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::B)
        }
    };

    table[0xF1] = OpEntry { 
        mnemonic: "SET 6, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::C)
        }
    };

    table[0xF2] = OpEntry { 
        mnemonic: "SET 6, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::D)
        }
    };

    table[0xF3] = OpEntry { 
        mnemonic: "SET 6, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::E)
        }
    };

    table[0xF4] = OpEntry { 
        mnemonic: "SET 6, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::H)
        }
    };

    table[0xF5] = OpEntry { 
        mnemonic: "SET 6, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::L)
        }
    };

    table[0xF6] = OpEntry { 
        mnemonic: "SET 6, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 6u8)
        }
    };

    table[0xF7] = OpEntry { 
        mnemonic: "SET 6, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 6u8, DmgSimpleRegisters::A)
        }
    };

    table[0xF8] = OpEntry { 
        mnemonic: "SET 7, B",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::B)
        }
    };

    table[0xF9] = OpEntry { 
        mnemonic: "SET 7, C",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::C)
        }
    };

    table[0xFA] = OpEntry { 
        mnemonic: "SET 7, D",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::D)
        }
    };

    table[0xFB] = OpEntry { 
        mnemonic: "SET 7, E",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::E)
        }
    };

    table[0xFC] = OpEntry { 
        mnemonic: "SET 7, H",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::H)
        }
    };

    table[0xFD] = OpEntry { 
        mnemonic: "SET 7, L",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::L)
        }
    };

    table[0xFE] = OpEntry { 
        mnemonic: "SET 7, (HL)",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_hl(regfile, 7u8)
        }
    };

    table[0xFF] = OpEntry { 
        mnemonic: "SET 7, A",
        instruction_length: 2,
        exec: |regfile: &mut Regfile, memory: &mut Memory| -> u8 {
            op_set_u3_r8(regfile, 7u8, DmgSimpleRegisters::A)
        }
    };


    table
}