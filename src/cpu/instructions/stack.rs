use crate::{cpu::regfile::{self, *}, mem::mem::Memory};

//ADD HL, SP
pub fn op_add_hl_sp(reg_file: &mut Regfile) -> u8 {

    let sp = reg_file.read_sp();
    let hl = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let (value, carry) = hl.overflowing_add(sp);
    let hcarry = (sp & 0x0FFF) + (hl & 0x0FFF) > 0x0FFF;

    reg_file.write_double_register(DmgDoubleRegisters::HL, value);
    reg_file.write_flags(
        reg_file.read_flag(DmgFlags::Zero),
        false,
        hcarry,
        carry
    );
    8u8
}

//ADD SP, e8
pub fn op_add_sp_e8(reg_file: &mut Regfile, offset: i8) -> u8 {

    let value = generic_add_sp_e8(reg_file, offset);
    reg_file.write_sp(value);
    16u8
}

//LD HL, SP+e8
pub fn op_load_hl_spe(reg_file: &mut Regfile, offset: i8) -> u8 {

    let value = generic_add_sp_e8(reg_file, offset);
    reg_file.write_double_register(DmgDoubleRegisters::HL, value);
    12u8
}

fn generic_add_sp_e8(reg_file: &mut Regfile, offset: i8) -> u16 {
    let sp = reg_file.read_sp();
    let unsigned = offset as u8 as u16;

    let hcarry = (sp & 0x0F) + (unsigned & 0x0F) > 0x0F;
    let carry = (sp & 0xFF) + (unsigned & 0xFF) > 0xFF;

    reg_file.write_flags(false, false, hcarry, carry);
    sp.wrapping_add_signed(offset as i16)
}

//DEC SP
pub fn op_dec_sp(reg_file: &mut Regfile) -> u8 {

    let value = reg_file.read_sp().wrapping_sub(1);
    reg_file.write_sp(value);
    8u8
}

//INC SP
pub fn op_inc_sp(reg_file: &mut Regfile) -> u8 {

    let value = reg_file.read_sp().wrapping_add(1);
    reg_file.write_sp(value);
    8u8
}

//LD SP, n16
pub fn op_load_sp_n16(reg_file: &mut Regfile, source: u16) -> u8 {

    reg_file.write_sp(source);
    12u8
}

//LD [n16], SP
pub fn op_load_n16_sp(reg_file: &mut Regfile, mem: &mut Memory, destination: u16) -> u8 {

    let sp = reg_file.read_sp();
    mem.write_mem_16(destination, sp);

    20u8
}

//LD SP, HL
pub fn op_load_sp_hl(reg_file: &mut Regfile) -> u8 {

    let value = reg_file.read_double_register(DmgDoubleRegisters::HL);
    reg_file.write_sp(value);

    8u8
}

//POP AF
pub fn op_pop_af(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {

    //F's low nibble is not writable: the popped value keeps only the four flag bits.
    let value = generic_pop(reg_file, mem) & 0xFFF0;
    reg_file.write_double_register(DmgDoubleRegisters::AF, value);

    12u8
}

//POP r16
pub fn op_pop_r16(reg_file: &mut Regfile, mem: &mut Memory, target: DmgDoubleRegisters) -> u8 {

    let value = generic_pop(reg_file, mem);
    reg_file.write_double_register(target, value);

    12u8
}

//PUSH AF
pub fn op_push_af(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {

    let value = reg_file.read_double_register(DmgDoubleRegisters::AF) & 0xFFF0;
    generic_push(reg_file, mem, value);

    16u8
}

//PUSH r16
pub fn op_push_r16(reg_file: &mut Regfile, mem: &mut Memory, source: DmgDoubleRegisters) -> u8 {

    let value = reg_file.read_double_register(source);
    generic_push(reg_file, mem, value);

    16u8
}

fn generic_push(reg_file: &mut Regfile, mem: &mut Memory, value: u16) -> () {
    let mut sp = reg_file.read_sp();

    sp = sp.wrapping_sub(1);
    mem.write_mem_8(sp, (value >> 8) as u8);

    sp = sp.wrapping_sub(1);
    mem.write_mem_8(sp, (value & 0x00FF) as u8);

    reg_file.write_sp(sp);
}

fn generic_pop(reg_file: &mut Regfile, mem: &mut Memory) -> u16 {
    let mut sp = reg_file.read_sp();

    let low_value = mem.read_mem_8(sp);
    sp = sp.wrapping_add(1);

    let high_value = mem.read_mem_8(sp);
    sp = sp.wrapping_add(1);

    reg_file.write_sp(sp);
    u16::from_le_bytes([low_value, high_value])
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::cpu::decoder::OpTable;

    const STACK_TOP: u16 = 0xC100;

    fn regfile_with_sp(sp: u16) -> (Regfile, Memory) {
        let mut reg_file: Regfile = Regfile::new();
        reg_file.write_sp(sp);
        (reg_file, Memory::new())
    }

    //PUSH writes the high byte at the higher address and moves SP down by two
    #[test]
    fn test_push_r16_stack_layout() {
        let (mut reg_file, mut mem) = regfile_with_sp(STACK_TOP);
        reg_file.write_double_register(DmgDoubleRegisters::BC, 0x1234);

        op_push_r16(&mut reg_file, &mut mem, DmgDoubleRegisters::BC);

        assert_eq!(STACK_TOP - 2, reg_file.read_sp(), "PUSH must decrement SP by two");
        assert_eq!(0x34, mem.read_mem_8(STACK_TOP - 2), "low byte belongs at the lower address");
        assert_eq!(0x12, mem.read_mem_8(STACK_TOP - 1), "high byte belongs at the higher address");
    }

    //Regression: PUSH must write the register to memory, not read memory into the register
    #[test]
    fn test_push_r16_leaves_source_register_untouched() {
        let (mut reg_file, mut mem) = regfile_with_sp(STACK_TOP);
        reg_file.write_double_register(DmgDoubleRegisters::DE, 0xBEEF);

        op_push_r16(&mut reg_file, &mut mem, DmgDoubleRegisters::DE);

        assert_eq!(0xBEEF, reg_file.read_double_register(DmgDoubleRegisters::DE));
    }

    //POP reads the pair back little-endian and moves SP up by two
    #[test]
    fn test_pop_r16_stack_layout() {
        let (mut reg_file, mut mem) = regfile_with_sp(STACK_TOP - 2);
        mem.write_mem_8(STACK_TOP - 2, 0x34); //low
        mem.write_mem_8(STACK_TOP - 1, 0x12); //high

        op_pop_r16(&mut reg_file, &mut mem, DmgDoubleRegisters::HL);

        assert_eq!(0x1234, reg_file.read_double_register(DmgDoubleRegisters::HL));
        assert_eq!(STACK_TOP, reg_file.read_sp(), "POP must increment SP by two");
    }

    //Round trip through the opcode table, which also covers PUSH/POP's table wiring
    #[test]
    fn test_push_pop_round_trip_through_optable() {
        const PUSH_BC: u8 = 0xC5;
        const POP_BC: u8 = 0xC1;

        let (mut reg_file, mut mem) = regfile_with_sp(STACK_TOP);
        let table: OpTable = OpTable::new();
        reg_file.write_double_register(DmgDoubleRegisters::BC, 0xCAFE);

        (table.decode(PUSH_BC).get_instruction())(&mut reg_file, &mut mem);
        reg_file.write_double_register(DmgDoubleRegisters::BC, 0x0000);
        (table.decode(POP_BC).get_instruction())(&mut reg_file, &mut mem);

        assert_eq!(0xCAFE, reg_file.read_double_register(DmgDoubleRegisters::BC));
        assert_eq!(STACK_TOP, reg_file.read_sp(), "a push/pop pair must leave SP where it started");
    }

    //Regression: PUSH AF must push A too, not overwrite it with F
    #[test]
    fn test_push_af_pushes_both_bytes() {
        let (mut reg_file, mut mem) = regfile_with_sp(STACK_TOP);
        reg_file.write_register(DmgSimpleRegisters::A, 0x12);
        reg_file.write_register(DmgSimpleRegisters::F, 0xB0);

        op_push_af(&mut reg_file, &mut mem);

        assert_eq!(0xB0, mem.read_mem_8(STACK_TOP - 2), "F belongs at the lower address");
        assert_eq!(0x12, mem.read_mem_8(STACK_TOP - 1), "A must survive the push");
    }

    //Regression: POP AF must load A from the high byte, not a second copy of F
    #[test]
    fn test_pop_af_masks_flag_low_nibble() {
        let (mut reg_file, mut mem) = regfile_with_sp(STACK_TOP - 2);
        mem.write_mem_8(STACK_TOP - 2, 0xBF); //F, low nibble is not writable
        mem.write_mem_8(STACK_TOP - 1, 0x12); //A

        op_pop_af(&mut reg_file, &mut mem);

        assert_eq!(0x12, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(0xB0, reg_file.read_register(DmgSimpleRegisters::F), "F's low nibble must read back as zero");
        assert_eq!(STACK_TOP, reg_file.read_sp());
    }

    //LD [n16], SP writes exactly two bytes, little-endian
    #[test]
    fn test_load_n16_sp_writes_only_two_bytes() {
        let (mut reg_file, mut mem) = regfile_with_sp(0xABCD);
        mem.write_mem_8(0xC002, 0xFF); //canary: must not be clobbered

        op_load_n16_sp(&mut reg_file, &mut mem, 0xC000);

        assert_eq!(0xCD, mem.read_mem_8(0xC000));
        assert_eq!(0xAB, mem.read_mem_8(0xC001));
        assert_eq!(0xFF, mem.read_mem_8(0xC002), "the byte past the destination must be left alone");
    }

    //ADD SP,e8 sets H and C from the low nibble/low byte and always clears Z and N
    #[test]
    fn test_add_sp_e8_sets_half_and_full_carry() {
        let (mut reg_file, _) = regfile_with_sp(0x00FF);
        reg_file.write_flag(DmgFlags::Zero, true);

        op_add_sp_e8(&mut reg_file, 1);

        assert_eq!(0x0100, reg_file.read_sp());
        assert_eq!(false, reg_file.read_flag(DmgFlags::Zero), "ADD SP,e8 always clears Z");
        assert_eq!(false, reg_file.read_flag(DmgFlags::Subtract));
        assert_eq!(true, reg_file.read_flag(DmgFlags::HCarry));
        assert_eq!(true, reg_file.read_flag(DmgFlags::Carry));
    }

    //Regression: a negative offset must not sign-extend into the carry computation
    #[test]
    fn test_add_sp_e8_negative_offset_carries() {
        let (mut reg_file, _) = regfile_with_sp(0xC000);

        op_add_sp_e8(&mut reg_file, -1);

        assert_eq!(0xBFFF, reg_file.read_sp());
        assert_eq!(false, reg_file.read_flag(DmgFlags::HCarry), "0x0 + 0xF must not half-carry");
        assert_eq!(false, reg_file.read_flag(DmgFlags::Carry), "0x00 + 0xFF must not carry");
    }

    //LD HL, SP+e8 shares ADD SP,e8's flag behaviour and must not touch SP
    #[test]
    fn test_load_hl_spe_sets_flags_and_preserves_sp() {
        let (mut reg_file, _) = regfile_with_sp(0x00FF);
        reg_file.write_flag(DmgFlags::Zero, true);

        op_load_hl_spe(&mut reg_file, 1);

        assert_eq!(0x0100, reg_file.read_double_register(DmgDoubleRegisters::HL));
        assert_eq!(0x00FF, reg_file.read_sp(), "LD HL,SP+e8 must leave SP unchanged");
        assert_eq!(false, reg_file.read_flag(DmgFlags::Zero));
        assert_eq!(true, reg_file.read_flag(DmgFlags::HCarry));
        assert_eq!(true, reg_file.read_flag(DmgFlags::Carry));
    }
}
