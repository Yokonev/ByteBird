use crate::{cpu::regfile::{DmgFlags::HCarry, *}, mem::mem::Memory};

// ===== ADD WITH CARRY =====

//ADC A, r8
pub fn op_adc_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {

    let reg = reg_file.read_register(source);
    generic_adc(reg_file, reg);
    4u8
}

//ADC A, [HL]
pub fn op_adc_acc_hl(reg_file: &mut Regfile, mem: &Memory) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mem_val = mem.read_mem_8(hl);
    generic_adc(reg_file, mem_val);
    8u8
}

//ADC A, n8
pub fn op_adc_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    
    generic_adc(reg_file, source);
    8u8
}

fn generic_adc(reg_file: &mut Regfile, source: u8) -> () {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let carry_in = reg_file.read_flag(DmgFlags::Carry) as u8;

    let sum = acc as u16 + source as u16 + carry_in as u16;
    let result = sum as u8;

    let hcarry =  (acc & 0x0F) + (source & 0x0F) + carry_in > 0x0F;

    reg_file.write_register(DmgSimpleRegisters::A, result);
    reg_file.write_flags(result == 0, false, hcarry, sum > 0xFF);
}

// ===== ADD WITHOUT CARRY =====

//ADD A, r8
pub fn op_add_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    
    let reg = reg_file.read_register(source);
    generic_add(reg_file, reg);
    4u8
}

//ADD A, [HL]
pub fn op_add_acc_hl(reg_file: &mut Regfile, mem: &Memory) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mem_val = mem.read_mem_8(hl);
    generic_add(reg_file, mem_val);
    8u8
}

//ADD A, n8
pub fn op_add_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    
    generic_add(reg_file, source);
    8u8
}

fn generic_add(reg_file: &mut Regfile, source: u8) -> () {

    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let (result, carry) = acc.overflowing_add(source);
    let hcarry = (acc & 0x0F) + (source & 0x0F) > 0x0F;

    reg_file.write_register(DmgSimpleRegisters::A, result);
    reg_file.write_flags(result == 0, false, hcarry, carry);
}

//ADD HL, r16
pub fn op_add_hl_r16(reg_file: &mut Regfile, source:DmgDoubleRegisters) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let reg = reg_file.read_double_register(source);

    let (result, carry) = hl.overflowing_add(reg);
    let hcarry = (hl & 0x0FFF) + (reg & 0x0FFF) > 0x0FFF;

    reg_file.write_double_register(DmgDoubleRegisters::HL, result);
    reg_file.write_flags(reg_file.read_flag(DmgFlags::Zero), false, hcarry, carry);

    8u8
}

// ===== COMPARE =====

//CP A, r8
pub fn op_cp_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    
    let reg = reg_file.read_register(source);
    generic_cp(reg_file, reg);
    4u8
}

//CP A, [HL]
pub fn op_cp_acc_hl(reg_file: &mut Regfile, mem: &Memory) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(hl);
    generic_cp(reg_file, value);
    8u8
}

//CP A, n8
pub fn op_cp_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    
    generic_cp(reg_file, source);
    8u8
}

fn generic_cp(reg_file: &mut Regfile, source: u8) -> () {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);

    let (result, borrow) = acc.overflowing_sub(source);
    let hborrow = (acc & 0x0F) < (source & 0x0F);

    reg_file.write_flags(result == 0, true, hborrow, borrow);
}

// ===== DEC =====

//DEC r8
pub fn op_dec_r8(reg_file: &mut Regfile, destination:DmgSimpleRegisters) -> u8 {

    let reg = reg_file.read_register(destination);
    let result = generic_dec(reg_file, reg);
    reg_file.write_register(destination, result);
    4u8
}

//DEC [HL]
pub fn op_dec_hl(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(hl);
    let result = generic_dec(reg_file, value);
    mem.write_mem_8(hl, result);
    12u8
}

fn generic_dec(reg_file: &mut Regfile, target: u8) -> u8 {
    let result = target.wrapping_sub(1);
    let hborrow = (target & 0x0F) < (1 & 0x0F);

    reg_file.write_flags(
        result == 0, 
        true, 
        hborrow, 
        reg_file.read_flag(DmgFlags::Carry)
    );

    result
}

//DEC r16
pub fn op_dec_r16(reg_file: &mut Regfile, destination:DmgDoubleRegisters) -> u8 {
    let reg = reg_file.read_double_register(destination);
    reg_file.write_double_register(destination, reg.wrapping_sub(1));
    8u8
}

// ===== INC =====

//INC r8
pub fn op_inc_r8(reg_file: &mut Regfile, destination:DmgSimpleRegisters) -> u8 {
    let reg = reg_file.read_register(destination);
    let result = generic_inc(reg_file, reg);
    reg_file.write_register(destination, result);
    4u8
}

//INC [HL]
pub fn op_inc_hl(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(hl);
    let result = generic_inc(reg_file, value);
    mem.write_mem_8(hl, result);
    12u8
}

//INC r16
pub fn op_inc_r16(reg_file: &mut Regfile, destination:DmgDoubleRegisters) -> u8 {
    let reg = reg_file.read_double_register(destination);
    reg_file.write_double_register(destination, reg.wrapping_add(1));
    8u8
}

fn generic_inc(reg_file: &mut Regfile, target: u8) -> u8 {
    let result = target.wrapping_add(1);
    let hcarry = (target & 0x0F) + (1 & 0x0F) > 0x0F;
    
    reg_file.write_flags(
        result == 0, 
        false, 
        hcarry, 
        reg_file.read_flag(DmgFlags::Carry)
    );
    
    result
}

// ===== SUBTRACT WITH CARRY =====

//SBC A, r8
pub fn op_sbc_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {

    let reg = reg_file.read_register(source);
    generic_sbc(reg_file, reg);
    4u8
}

//SBC A, [HL]
pub fn op_sbc_acc_hl(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(hl);
    generic_sbc(reg_file, value);
    8u8
}

//SBC A, n8
pub fn op_sbc_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    generic_sbc(reg_file, source);
    8u8
}

fn generic_sbc(reg_file: &mut Regfile, source: u8) -> () {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let carry_in = reg_file.read_flag(DmgFlags::Carry) as u8;

    let diff = acc as i16 - source as i16 - carry_in as i16;
    let result = diff as u8;

    let hborrow =  ((acc & 0x0F) as i16 - (source & 0x0F) as i16 - carry_in as i16) < 0;

    reg_file.write_register(DmgSimpleRegisters::A, result);
    reg_file.write_flags(result == 0, true, hborrow, diff < 0);
}

// ===== SUBTRACT WITHOUT CARRY =====

//SUB A, r8
pub fn op_sub_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    
    let reg = reg_file.read_register(source);
    generic_sub(reg_file, reg);
    4u8
}

//SUB A, [HL]
pub fn op_sub_acc_hl(reg_file: &mut Regfile, mem: &Memory) -> u8 {
    
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let value = mem.read_mem_8(hl);
    generic_sub(reg_file, value);
    8u8
}

//SUB A, n8
pub fn op_sub_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    generic_sub(reg_file, source);
    8u8
}

fn generic_sub(reg_file: &mut Regfile, source: u8) -> () {

    let acc = reg_file.read_register(DmgSimpleRegisters::A);

    let (result, borrow) = acc.overflowing_sub(source);
    let hborrow = (acc & 0x0F) < (source & 0x0F);

    reg_file.write_register(DmgSimpleRegisters::A, result);
    reg_file.write_flags(result == 0, true, hborrow, borrow);
}

#[cfg(test)]
mod tests {

    use super::*;

    const HL_ADDR: u16 = 0xC000;

    //A regfile with A preloaded and HL pointing at a work RAM byte holding `at_hl`
    fn regfile_with(acc: u8, at_hl: u8) -> (Regfile, Memory) {
        let mut reg_file: Regfile = Regfile::new();
        let mut mem: Memory = Memory::new();

        reg_file.write_register(DmgSimpleRegisters::A, acc);
        reg_file.write_double_register(DmgDoubleRegisters::HL, HL_ADDR);
        mem.write_mem_8(HL_ADDR, at_hl);

        (reg_file, mem)
    }

    fn read_flags(reg_file: &Regfile) -> (bool, bool, bool, bool) {
        (
            reg_file.read_flag(DmgFlags::Zero),
            reg_file.read_flag(DmgFlags::Subtract),
            reg_file.read_flag(DmgFlags::HCarry),
            reg_file.read_flag(DmgFlags::Carry),
        )
    }

    // ===== ADC =====

    //ADC must fold the incoming carry into both the result and the half-carry
    #[test]
    fn test_adc_acc_r8_adds_carry_in() {
        let (mut reg_file, _) = regfile_with(0x0E, 0x00);
        reg_file.write_register(DmgSimpleRegisters::B, 0x01);
        reg_file.write_flag(DmgFlags::Carry, true);

        op_adc_acc_r8(&mut reg_file, DmgSimpleRegisters::B);

        assert_eq!(0x10, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, true, false), read_flags(&reg_file), "0x0E + 0x01 + carry must half-carry");
    }

    //The carry-in alone can push the result over 0xFF, and 0x00 must report zero
    #[test]
    fn test_adc_acc_n8_carry_in_causes_overflow() {
        let (mut reg_file, _) = regfile_with(0xFF, 0x00);
        reg_file.write_flag(DmgFlags::Carry, true);

        op_adc_acc_n8(&mut reg_file, 0x00);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((true, false, true, true), read_flags(&reg_file));
    }

    //ADC A,[HL] takes its operand from memory and costs 8 cycles
    #[test]
    fn test_adc_acc_hl_reads_memory() {
        let (mut reg_file, mem) = regfile_with(0x10, 0x05);
        reg_file.write_flag(DmgFlags::Carry, true);

        let cycles = op_adc_acc_hl(&mut reg_file, &mem);

        assert_eq!(0x16, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
    }

    // ===== ADD =====

    //A plain add that neither carries nor half-carries must clear every flag but keep N clear
    #[test]
    fn test_add_acc_r8_no_carry() {
        let (mut reg_file, _) = regfile_with(0x01, 0x00);
        reg_file.write_register(DmgSimpleRegisters::C, 0x02);

        op_add_acc_r8(&mut reg_file, DmgSimpleRegisters::C);

        assert_eq!(0x03, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, false, false), read_flags(&reg_file));
    }

    //Wrapping past 0xFF must set both C and Z rather than panicking
    #[test]
    fn test_add_acc_n8_wraps_and_sets_zero() {
        let (mut reg_file, _) = regfile_with(0xFF, 0x00);

        op_add_acc_n8(&mut reg_file, 0x01);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((true, false, true, true), read_flags(&reg_file));
    }

    //ADD A,A doubles the accumulator: the source and destination are the same register
    #[test]
    fn test_add_acc_r8_source_is_accumulator() {
        let (mut reg_file, _) = regfile_with(0x08, 0x00);

        op_add_acc_r8(&mut reg_file, DmgSimpleRegisters::A);

        assert_eq!(0x10, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, true, false), read_flags(&reg_file));
    }

    #[test]
    fn test_add_acc_hl_reads_memory() {
        let (mut reg_file, mem) = regfile_with(0x20, 0x22);

        let cycles = op_add_acc_hl(&mut reg_file, &mem);

        assert_eq!(0x42, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
    }

    //ADD HL,r16 half-carries out of bit 11, not bit 3, and leaves Z alone
    #[test]
    fn test_add_hl_r16_half_carry_from_bit_eleven() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_double_register(DmgDoubleRegisters::HL, 0x0FFF);
        reg_file.write_double_register(DmgDoubleRegisters::BC, 0x0001);
        reg_file.write_flag(DmgFlags::Zero, true);

        op_add_hl_r16(&mut reg_file, DmgDoubleRegisters::BC);

        assert_eq!(0x1000, reg_file.read_double_register(DmgDoubleRegisters::HL));
        assert_eq!((true, false, true, false), read_flags(&reg_file), "ADD HL,r16 must preserve Z");
    }

    //A 16-bit overflow sets C; the result wraps and Z still must not be touched
    #[test]
    fn test_add_hl_r16_overflow_sets_carry_and_preserves_zero() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_double_register(DmgDoubleRegisters::HL, 0xFFFF);
        reg_file.write_double_register(DmgDoubleRegisters::DE, 0x0001);

        op_add_hl_r16(&mut reg_file, DmgDoubleRegisters::DE);

        assert_eq!(0x0000, reg_file.read_double_register(DmgDoubleRegisters::HL));
        assert_eq!((false, false, true, true), read_flags(&reg_file), "a zero result must not set Z");
    }

    // ===== CP =====

    //CP is SUB without the write-back: flags move, A does not
    #[test]
    fn test_cp_acc_r8_leaves_accumulator_untouched() {
        let (mut reg_file, _) = regfile_with(0x3C, 0x00);
        reg_file.write_register(DmgSimpleRegisters::B, 0x2F);

        op_cp_acc_r8(&mut reg_file, DmgSimpleRegisters::B);

        assert_eq!(0x3C, reg_file.read_register(DmgSimpleRegisters::A), "CP must not write A");
        assert_eq!((false, true, true, false), read_flags(&reg_file));
    }

    //Comparing equal values sets Z and clears the borrows
    #[test]
    fn test_cp_acc_n8_equal_sets_zero() {
        let (mut reg_file, _) = regfile_with(0x3C, 0x00);

        op_cp_acc_n8(&mut reg_file, 0x3C);

        assert_eq!((true, true, false, false), read_flags(&reg_file));
    }

    //A larger operand borrows out of bit 7
    #[test]
    fn test_cp_acc_hl_greater_operand_sets_carry() {
        let (mut reg_file, mem) = regfile_with(0x10, 0x20);

        op_cp_acc_hl(&mut reg_file, &mem);

        assert_eq!(0x10, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, false, true), read_flags(&reg_file));
    }

    // ===== DEC =====

    //DEC r8 half-borrows when the low nibble is empty, and must leave C alone
    #[test]
    fn test_dec_r8_half_borrow_preserves_carry() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_register(DmgSimpleRegisters::D, 0x10);
        reg_file.write_flag(DmgFlags::Carry, true);

        op_dec_r8(&mut reg_file, DmgSimpleRegisters::D);

        assert_eq!(0x0F, reg_file.read_register(DmgSimpleRegisters::D));
        assert_eq!((false, true, true, true), read_flags(&reg_file), "DEC r8 must not touch C");
    }

    //0x00 - 1 wraps to 0xFF rather than panicking
    #[test]
    fn test_dec_r8_wraps_from_zero() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_register(DmgSimpleRegisters::E, 0x00);

        op_dec_r8(&mut reg_file, DmgSimpleRegisters::E);

        assert_eq!(0xFF, reg_file.read_register(DmgSimpleRegisters::E));
        assert_eq!((false, true, true, false), read_flags(&reg_file));
    }

    //Reaching zero sets Z
    #[test]
    fn test_dec_r8_to_zero_sets_zero_flag() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_register(DmgSimpleRegisters::B, 0x01);

        op_dec_r8(&mut reg_file, DmgSimpleRegisters::B);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::B));
        assert_eq!((true, true, false, false), read_flags(&reg_file));
    }

    //DEC [HL] writes the decremented byte back to memory it read it from
    #[test]
    fn test_dec_hl_writes_back_to_memory() {
        let (mut reg_file, mut mem) = regfile_with(0x00, 0x01);

        let cycles = op_dec_hl(&mut reg_file, &mut mem);

        assert_eq!(0x00, mem.read_mem_8(HL_ADDR));
        assert_eq!(12, cycles);
        assert_eq!((true, true, false, false), read_flags(&reg_file));
    }

    //DEC r16 is pure arithmetic: it wraps and touches no flags at all
    #[test]
    fn test_dec_r16_wraps_without_touching_flags() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_double_register(DmgDoubleRegisters::BC, 0x0000);
        reg_file.write_flags(true, true, true, true);

        op_dec_r16(&mut reg_file, DmgDoubleRegisters::BC);

        assert_eq!(0xFFFF, reg_file.read_double_register(DmgDoubleRegisters::BC));
        assert_eq!((true, true, true, true), read_flags(&reg_file), "DEC r16 must not write flags");
    }

    // ===== INC =====

    //INC r8 half-carries out of a full low nibble, and must leave C alone
    #[test]
    fn test_inc_r8_half_carry_preserves_carry() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_register(DmgSimpleRegisters::H, 0x0F);
        reg_file.write_flag(DmgFlags::Carry, true);

        op_inc_r8(&mut reg_file, DmgSimpleRegisters::H);

        assert_eq!(0x10, reg_file.read_register(DmgSimpleRegisters::H));
        assert_eq!((false, false, true, true), read_flags(&reg_file), "INC r8 must not touch C");
    }

    //0xFF + 1 wraps to zero: Z and H set, C untouched (here: left clear)
    #[test]
    fn test_inc_r8_wraps_to_zero() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_register(DmgSimpleRegisters::L, 0xFF);

        op_inc_r8(&mut reg_file, DmgSimpleRegisters::L);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::L));
        assert_eq!((true, false, true, false), read_flags(&reg_file));
    }

    //INC [HL] writes the incremented byte back to the address it read
    #[test]
    fn test_inc_hl_writes_back_to_memory() {
        let (mut reg_file, mut mem) = regfile_with(0x00, 0x0F);

        let cycles = op_inc_hl(&mut reg_file, &mut mem);

        assert_eq!(0x10, mem.read_mem_8(HL_ADDR));
        assert_eq!(12, cycles);
        assert_eq!((false, false, true, false), read_flags(&reg_file));
    }

    //INC r16 wraps and touches no flags at all
    #[test]
    fn test_inc_r16_wraps_without_touching_flags() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_double_register(DmgDoubleRegisters::DE, 0xFFFF);
        reg_file.write_flags(false, false, false, false);

        op_inc_r16(&mut reg_file, DmgDoubleRegisters::DE);

        assert_eq!(0x0000, reg_file.read_double_register(DmgDoubleRegisters::DE));
        assert_eq!((false, false, false, false), read_flags(&reg_file), "INC r16 must not write flags");
    }

    // ===== SBC =====

    //SBC must subtract the incoming carry on top of the operand
    #[test]
    fn test_sbc_acc_r8_subtracts_carry_in() {
        let (mut reg_file, _) = regfile_with(0x10, 0x00);
        reg_file.write_register(DmgSimpleRegisters::B, 0x01);
        reg_file.write_flag(DmgFlags::Carry, true);

        op_sbc_acc_r8(&mut reg_file, DmgSimpleRegisters::B);

        assert_eq!(0x0E, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, true, false), read_flags(&reg_file));
    }

    //The carry-in alone can borrow past zero
    #[test]
    fn test_sbc_acc_n8_carry_in_causes_borrow() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);
        reg_file.write_flag(DmgFlags::Carry, true);

        op_sbc_acc_n8(&mut reg_file, 0x00);

        assert_eq!(0xFF, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, true, true), read_flags(&reg_file));
    }

    //With carry clear SBC behaves exactly like SUB
    #[test]
    fn test_sbc_acc_hl_without_carry_matches_sub() {
        let (mut reg_file, mut mem) = regfile_with(0x3B, 0x2A);

        let cycles = op_sbc_acc_hl(&mut reg_file, &mut mem);

        assert_eq!(0x11, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((false, true, false, false), read_flags(&reg_file));
    }

    // ===== SUB =====

    //Subtracting a value with a larger low nibble half-borrows
    #[test]
    fn test_sub_acc_r8_half_borrow() {
        let (mut reg_file, _) = regfile_with(0x3E, 0x00);
        reg_file.write_register(DmgSimpleRegisters::C, 0x0F);

        op_sub_acc_r8(&mut reg_file, DmgSimpleRegisters::C);

        assert_eq!(0x2F, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, true, false), read_flags(&reg_file));
    }

    //Subtracting a register from itself always yields zero
    #[test]
    fn test_sub_acc_r8_source_is_accumulator() {
        let (mut reg_file, _) = regfile_with(0x3E, 0x00);

        op_sub_acc_r8(&mut reg_file, DmgSimpleRegisters::A);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((true, true, false, false), read_flags(&reg_file));
    }

    //Underflowing wraps and sets C rather than panicking
    #[test]
    fn test_sub_acc_n8_underflow_wraps() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);

        op_sub_acc_n8(&mut reg_file, 0x01);

        assert_eq!(0xFF, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, true, true), read_flags(&reg_file));
    }

    #[test]
    fn test_sub_acc_hl_reads_memory() {
        let (mut reg_file, mem) = regfile_with(0x42, 0x02);

        let cycles = op_sub_acc_hl(&mut reg_file, &mem);

        assert_eq!(0x40, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
    }
}
