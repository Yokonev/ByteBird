use crate::cpu::regfile::{self, *};
use crate::mem::mem::Memory;

//LD r8, r8
pub fn op_load_r8_r8(reg_file:&mut Regfile, destination:DmgSimpleRegisters, source:DmgSimpleRegisters) -> u8 {
    let value = reg_file.read_register(source);
    reg_file.write_register(destination, value);
    //Note: Handle breakpoints (LD B, B) here later.
    4u8
}

//LD r8, n8
pub fn op_load_r8_n8(reg_file:&mut Regfile, destination:DmgSimpleRegisters, source:u8) -> u8 {
    reg_file.write_register(destination, source);
    8u8
}

//LD r16, n16
pub fn op_load_r16_n16(reg_file:&mut Regfile, destination:DmgDoubleRegisters, source:u16) -> u8 {
    reg_file.write_double_register(destination, source);
    12u8
}

//LD [HL], r8
pub fn op_load_hl_r8(reg_file:&mut Regfile, mem: &mut Memory, source:DmgSimpleRegisters) -> u8 {
    let value = reg_file.read_register(source);
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    mem.write_mem_8(addr, value);
    8u8
}

//LD [HL], n8
pub fn op_load_hl_n8(reg_file:&mut Regfile, mem: &mut Memory, source:u8) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    mem.write_mem_8(addr, source);
    12u8
}

//LD r8, [HL]
pub fn op_load_r8_hl(reg_file:&mut Regfile, mem: &mut Memory, destination:DmgSimpleRegisters) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(addr);
    reg_file.write_register(destination, value);
    8u8
}

//LD [r16], A
pub fn op_load_r16_acc(reg_file:&mut Regfile, mem: &mut Memory, destination:DmgDoubleRegisters) -> u8 {
    let addr = reg_file.read_double_register(destination);
    let value = reg_file.read_register(DmgSimpleRegisters::A);
    mem.write_mem_8(addr, value);
    8u8
}

//LD [n16], A
pub fn op_load_n16_acc(reg_file:&mut Regfile, mem: &mut Memory, destination:u16) -> u8 {
    let value = reg_file.read_register(DmgSimpleRegisters::A);
    mem.write_mem_8(destination, value);
    16u8
}

//LD A, [r16]
pub fn op_load_acc_r16(reg_file:&mut Regfile, mem: &mut Memory, source:DmgDoubleRegisters) -> u8 {
    let addr = reg_file.read_double_register(source);
    let value = mem.read_mem_8(addr);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    8u8
}

//LD A, [n16]
pub fn op_load_acc_n16(reg_file:&mut Regfile, mem: &mut Memory, source:u16) -> u8 {
    let value = mem.read_mem_8(source);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    16u8
}

//LD [HLI], A
pub fn op_load_hli_acc(reg_file:&mut Regfile, mem: &mut Memory) -> u8 {
    let value = reg_file.read_register(DmgSimpleRegisters::A);
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    mem.write_mem_8(addr, value);
    reg_file.write_double_register(DmgDoubleRegisters::HL, addr.wrapping_add(1));
    8u8
}

//LD [HLD], A
pub fn op_load_hld_acc(reg_file:&mut Regfile, mem: &mut Memory) -> u8 {
    let value = reg_file.read_register(DmgSimpleRegisters::A);
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    mem.write_mem_8(addr, value);
    reg_file.write_double_register(DmgDoubleRegisters::HL, addr.wrapping_sub(1));
    8u8
}

//LD A, [HLI]
pub fn op_load_acc_hli(reg_file:&mut Regfile, mem: &mut Memory) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(addr);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    reg_file.write_double_register(DmgDoubleRegisters::HL, addr.wrapping_add(1));
    8u8
}

//LD A, [HLD]
pub fn op_load_acc_hld(reg_file:&mut Regfile, mem: &mut Memory) -> u8 {
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(addr);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    reg_file.write_double_register(DmgDoubleRegisters::HL, addr.wrapping_sub(1));
    8u8
}

//LDH [n16], A
pub fn op_load_high_n16_acc(reg_file:&mut Regfile, mem: &mut Memory, destination:u16) -> u8 {
    let value = reg_file.read_register(DmgSimpleRegisters::A);
    mem.write_mem_8(destination, value);
    12u8
}

//LDH [C], A
pub fn op_load_high_c_acc(reg_file:&mut Regfile, mem: &mut Memory) -> u8 {
    let value = reg_file.read_register(DmgSimpleRegisters::A);
    let addr = (0xFF00 | reg_file.read_register(DmgSimpleRegisters::C) as u16);
    mem.write_mem_8(addr, value);
    8u8
}

//LDH A, [n16]
pub fn op_load_high_acc_n16(reg_file:&mut Regfile, mem: &mut Memory, source: u16) -> u8 {
    let value = mem.read_mem_8(source);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    12u8
}

//LDH A, [C]
pub fn op_load_high_acc_c(reg_file:&mut Regfile, mem: &mut Memory) -> u8 {
    let addr = (0xFF00 | reg_file.read_register(DmgSimpleRegisters::C) as u16);
    let value = mem.read_mem_8(addr);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    8u8
}

#[cfg(test)]
mod tests {

    use super::*;

    const HL_ADDR: u16 = 0xC000;

    fn regfile_with_hl(addr: u16) -> (Regfile, Memory) {
        let mut reg_file: Regfile = Regfile::new();
        reg_file.write_double_register(DmgDoubleRegisters::HL, addr);
        (reg_file, Memory::new())
    }

    // ===== REGISTER / IMMEDIATE LOADS =====

    //LD r8,r8 copies the source into the destination and leaves the source alone
    #[test]
    fn test_load_r8_r8_copies_without_disturbing_source() {
        let (mut reg_file, _) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::B, 0x42);

        let cycles = op_load_r8_r8(&mut reg_file, DmgSimpleRegisters::D, DmgSimpleRegisters::B);

        assert_eq!(0x42, reg_file.read_register(DmgSimpleRegisters::D));
        assert_eq!(0x42, reg_file.read_register(DmgSimpleRegisters::B), "the source register must survive the copy");
        assert_eq!(4, cycles);
    }

    //LD B,B is the DMG's breakpoint idiom: it must still be a well-behaved no-op
    #[test]
    fn test_load_r8_r8_same_register_is_a_no_op() {
        let (mut reg_file, _) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::B, 0x42);

        op_load_r8_r8(&mut reg_file, DmgSimpleRegisters::B, DmgSimpleRegisters::B);

        assert_eq!(0x42, reg_file.read_register(DmgSimpleRegisters::B));
    }

    #[test]
    fn test_load_r8_n8_writes_immediate() {
        let (mut reg_file, _) = regfile_with_hl(HL_ADDR);

        let cycles = op_load_r8_n8(&mut reg_file, DmgSimpleRegisters::E, 0x7F);

        assert_eq!(0x7F, reg_file.read_register(DmgSimpleRegisters::E));
        assert_eq!(8, cycles);
    }

    //LD r16,n16 must land big-endian across the pair's two backing bytes
    #[test]
    fn test_load_r16_n16_splits_across_the_pair() {
        let (mut reg_file, _) = regfile_with_hl(HL_ADDR);

        let cycles = op_load_r16_n16(&mut reg_file, DmgDoubleRegisters::BC, 0xABCD);

        assert_eq!(0xABCD, reg_file.read_double_register(DmgDoubleRegisters::BC));
        assert_eq!(0xAB, reg_file.read_register(DmgSimpleRegisters::B), "B holds the high byte");
        assert_eq!(0xCD, reg_file.read_register(DmgSimpleRegisters::C), "C holds the low byte");
        assert_eq!(12, cycles);
    }

    // ===== [HL] LOADS =====

    #[test]
    fn test_load_hl_r8_writes_register_to_hl_address() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::D, 0x99);

        let cycles = op_load_hl_r8(&mut reg_file, &mut mem, DmgSimpleRegisters::D);

        assert_eq!(0x99, mem.read_mem_8(HL_ADDR));
        assert_eq!(8, cycles);
    }

    #[test]
    fn test_load_hl_n8_writes_immediate_to_hl_address() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);

        let cycles = op_load_hl_n8(&mut reg_file, &mut mem, 0x5A);

        assert_eq!(0x5A, mem.read_mem_8(HL_ADDR));
        assert_eq!(12, cycles);
    }

    #[test]
    fn test_load_r8_hl_reads_from_hl_address() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        mem.write_mem_8(HL_ADDR, 0x5A);

        let cycles = op_load_r8_hl(&mut reg_file, &mut mem, DmgSimpleRegisters::C);

        assert_eq!(0x5A, reg_file.read_register(DmgSimpleRegisters::C));
        assert_eq!(8, cycles);
    }

    //LD H,[HL] overwrites the very pointer it loaded through: the read must happen first
    #[test]
    fn test_load_r8_hl_into_pointer_register() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        mem.write_mem_8(HL_ADDR, 0x5A);

        op_load_r8_hl(&mut reg_file, &mut mem, DmgSimpleRegisters::H);

        assert_eq!(0x5A, reg_file.read_register(DmgSimpleRegisters::H));
    }

    // ===== ACCUMULATOR / ADDRESS LOADS =====

    #[test]
    fn test_load_r16_acc_writes_accumulator_through_pair() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_double_register(DmgDoubleRegisters::DE, 0xC050);
        reg_file.write_register(DmgSimpleRegisters::A, 0x77);

        let cycles = op_load_r16_acc(&mut reg_file, &mut mem, DmgDoubleRegisters::DE);

        assert_eq!(0x77, mem.read_mem_8(0xC050));
        assert_eq!(8, cycles);
    }

    #[test]
    fn test_load_acc_r16_reads_through_pair() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_double_register(DmgDoubleRegisters::BC, 0xC050);
        mem.write_mem_8(0xC050, 0x77);

        let cycles = op_load_acc_r16(&mut reg_file, &mut mem, DmgDoubleRegisters::BC);

        assert_eq!(0x77, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
    }

    //A round trip through an absolute address must come back byte-identical
    #[test]
    fn test_load_n16_acc_and_back() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::A, 0xC3);

        assert_eq!(16, op_load_n16_acc(&mut reg_file, &mut mem, 0xC123));
        assert_eq!(0xC3, mem.read_mem_8(0xC123));

        reg_file.write_register(DmgSimpleRegisters::A, 0x00);

        assert_eq!(16, op_load_acc_n16(&mut reg_file, &mut mem, 0xC123));
        assert_eq!(0xC3, reg_file.read_register(DmgSimpleRegisters::A));
    }

    // ===== HL INCREMENT / DECREMENT =====

    //LD [HLI],A stores at the *old* HL, then advances the pointer
    #[test]
    fn test_load_hli_acc_stores_then_increments() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::A, 0x11);

        let cycles = op_load_hli_acc(&mut reg_file, &mut mem);

        assert_eq!(0x11, mem.read_mem_8(HL_ADDR), "the store must use HL before the increment");
        assert_eq!(HL_ADDR + 1, reg_file.read_double_register(DmgDoubleRegisters::HL));
        assert_eq!(8, cycles);
    }

    //LD [HLD],A stores at the old HL, then walks the pointer back
    #[test]
    fn test_load_hld_acc_stores_then_decrements() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::A, 0x22);

        op_load_hld_acc(&mut reg_file, &mut mem);

        assert_eq!(0x22, mem.read_mem_8(HL_ADDR), "the store must use HL before the decrement");
        assert_eq!(HL_ADDR - 1, reg_file.read_double_register(DmgDoubleRegisters::HL));
    }

    #[test]
    fn test_load_acc_hli_reads_then_increments() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        mem.write_mem_8(HL_ADDR, 0x33);

        op_load_acc_hli(&mut reg_file, &mut mem);

        assert_eq!(0x33, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(HL_ADDR + 1, reg_file.read_double_register(DmgDoubleRegisters::HL));
    }

    #[test]
    fn test_load_acc_hld_reads_then_decrements() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        mem.write_mem_8(HL_ADDR, 0x44);

        op_load_acc_hld(&mut reg_file, &mut mem);

        assert_eq!(0x44, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(HL_ADDR - 1, reg_file.read_double_register(DmgDoubleRegisters::HL));
    }

    //HL must wrap rather than panic when the pointer walks off either end
    #[test]
    fn test_load_hli_acc_wraps_at_top_of_address_space() {
        let (mut reg_file, mut mem) = regfile_with_hl(0xFFFF);
        reg_file.write_register(DmgSimpleRegisters::A, 0x55);

        op_load_hli_acc(&mut reg_file, &mut mem);

        assert_eq!(0x0000, reg_file.read_double_register(DmgDoubleRegisters::HL));
    }

    #[test]
    fn test_load_hld_acc_wraps_at_bottom_of_address_space() {
        let (mut reg_file, mut mem) = regfile_with_hl(0x0000);
        reg_file.write_register(DmgSimpleRegisters::A, 0x55);

        op_load_hld_acc(&mut reg_file, &mut mem);

        assert_eq!(0xFFFF, reg_file.read_double_register(DmgDoubleRegisters::HL));
    }

    // ===== HIGH RAM / IO LOADS =====

    //LDH takes an already-resolved address: the table entry does the 0xFF00 + n8 arithmetic
    #[test]
    fn test_load_high_n16_acc_writes_resolved_address() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::A, 0x66);

        let cycles = op_load_high_n16_acc(&mut reg_file, &mut mem, 0xFF80);

        assert_eq!(0x66, mem.read_mem_8(0xFF80));
        assert_eq!(12, cycles);
    }

    #[test]
    fn test_load_high_acc_n16_reads_resolved_address() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        mem.write_mem_8(0xFF80, 0x66);

        let cycles = op_load_high_acc_n16(&mut reg_file, &mut mem, 0xFF80);

        assert_eq!(0x66, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(12, cycles);
    }

    //LDH [C],A resolves its own address as 0xFF00 | C
    #[test]
    fn test_load_high_c_acc_offsets_into_page_ff() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::C, 0x80);
        reg_file.write_register(DmgSimpleRegisters::A, 0x88);

        let cycles = op_load_high_c_acc(&mut reg_file, &mut mem);

        assert_eq!(0x88, mem.read_mem_8(0xFF80));
        assert_eq!(8, cycles);
    }

    #[test]
    fn test_load_high_acc_c_offsets_into_page_ff() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::C, 0x80);
        mem.write_mem_8(0xFF80, 0x88);

        let cycles = op_load_high_acc_c(&mut reg_file, &mut mem);

        assert_eq!(0x88, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
    }

    //C == 0x00 must address 0xFF00 itself, not wrap to 0x0000
    #[test]
    fn test_load_high_c_acc_with_zero_offset() {
        let (mut reg_file, mut mem) = regfile_with_hl(HL_ADDR);
        reg_file.write_register(DmgSimpleRegisters::C, 0x00);
        reg_file.write_register(DmgSimpleRegisters::A, 0x99);

        op_load_high_c_acc(&mut reg_file, &mut mem);

        assert_eq!(0x99, mem.read_mem_8(0xFF00));
    }
}
