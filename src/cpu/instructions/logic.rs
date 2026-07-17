use crate::cpu::regfile::*;
use crate::mem::mem::Memory;

//AND A, r8
pub fn op_and_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {

    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let reg = reg_file.read_register(source);
    generic_and(reg_file, acc, reg);
    4u8
}

//AND A, [HL]
pub fn op_and_acc_hl(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {

    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mem_value = mem.read_mem_8(addr);
    generic_and(reg_file, acc, mem_value);
    8u8
}

//AND A, n8
pub fn op_and_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {

    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    generic_and(reg_file, acc, source);
    8u8
}

fn generic_and(reg_file: &mut Regfile, first: u8, second: u8) -> () {

    let value = first & second;
    reg_file.write_register(DmgSimpleRegisters::A, value);
    reg_file.write_flags(
        value == 0, 
        false, 
        true, 
        false
    );

}
//CPL (Bitwise not)
pub fn op_cpl(reg_file: &mut Regfile) -> u8 {

    let value = !reg_file.read_register(DmgSimpleRegisters::A);
    reg_file.write_register(DmgSimpleRegisters::A, value);
    reg_file.write_flags(
        reg_file.read_flag(DmgFlags::Zero), 
        true, 
        true, 
        reg_file.read_flag(DmgFlags::Carry));
    4u8
}

//OR A, r8
pub fn op_or_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let reg = reg_file.read_register(source);
    generic_or(reg_file, acc, reg);
    4u8
}

//OR A, [HL]
pub fn op_or_acc_hl(reg_file: &mut Regfile, mem: &mut Memory, source:DmgSimpleRegisters) -> u8 {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mem_value = mem.read_mem_8(addr);
    generic_or(reg_file, acc, mem_value);
    8u8
}

//OR A, n8
pub fn op_or_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    generic_or(reg_file, acc, source);
    8u8
}

fn generic_or(reg_file: &mut Regfile, first: u8, second: u8) -> () {

    let value = first | second;
    reg_file.write_register(DmgSimpleRegisters::A, value);
    reg_file.write_flags(
        value == 0, 
        false, 
        false, 
        false
    );
}

//XOR A, r8
pub fn op_xor_acc_r8(reg_file: &mut Regfile, source:DmgSimpleRegisters) -> u8 {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let reg = reg_file.read_register(source);
    generic_xor(reg_file, acc, reg);
    4u8
}

//XOR A, [HL]
pub fn op_xor_acc_hl(reg_file: &mut Regfile, mem: &mut Memory) -> u8 {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    let addr = reg_file.read_double_register(DmgDoubleRegisters::HL);
    let mem_value = mem.read_mem_8(addr);
    generic_xor(reg_file, acc, mem_value);
    8u8
}

//XOR A, n8
pub fn op_xor_acc_n8(reg_file: &mut Regfile, source:u8) -> u8 {
    let acc = reg_file.read_register(DmgSimpleRegisters::A);
    generic_xor(reg_file, acc, source);
    8u8
}

fn generic_xor(reg_file: &mut Regfile, first: u8, second: u8) -> () {
    let value = first ^ second;
    reg_file.write_register(DmgSimpleRegisters::A, value);
    reg_file.write_flags(
        value == 0,
        false,
        false,
        false
    );
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

    // ===== AND =====

    //AND is the one logic op that sets H: N and C clear, H always set
    #[test]
    fn test_and_acc_r8_sets_half_carry() {
        let (mut reg_file, _) = regfile_with(0b1010_1010, 0x00);
        reg_file.write_register(DmgSimpleRegisters::B, 0b1100_1100);

        let cycles = op_and_acc_r8(&mut reg_file, DmgSimpleRegisters::B);

        assert_eq!(0b1000_1000, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(4, cycles);
        assert_eq!((false, false, true, false), read_flags(&reg_file), "AND must always set H");
    }

    //AND must clear an incoming C rather than preserve it
    #[test]
    fn test_and_acc_r8_clears_incoming_carry() {
        let (mut reg_file, _) = regfile_with(0xFF, 0x00);
        reg_file.write_register(DmgSimpleRegisters::C, 0x0F);
        reg_file.write_flags(false, true, false, true);

        op_and_acc_r8(&mut reg_file, DmgSimpleRegisters::C);

        assert_eq!(0x0F, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, true, false), read_flags(&reg_file), "AND must clear N and C");
    }

    //Disjoint operands yield zero, which sets Z (H stays set even then)
    #[test]
    fn test_and_acc_n8_zero_result_sets_zero_flag() {
        let (mut reg_file, _) = regfile_with(0b0101_0101, 0x00);

        let cycles = op_and_acc_n8(&mut reg_file, 0b1010_1010);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((true, false, true, false), read_flags(&reg_file));
    }

    //AND A,A is the idiomatic "test A": A is unchanged, only the flags move
    #[test]
    fn test_and_acc_r8_source_is_accumulator() {
        let (mut reg_file, _) = regfile_with(0x3C, 0x00);

        op_and_acc_r8(&mut reg_file, DmgSimpleRegisters::A);

        assert_eq!(0x3C, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, true, false), read_flags(&reg_file));
    }

    //AND A,[HL] takes its operand from the byte HL points at and costs 8 cycles
    #[test]
    fn test_and_acc_hl_reads_memory() {
        let (mut reg_file, mut mem) = regfile_with(0xF0, 0x3C);

        let cycles = op_and_acc_hl(&mut reg_file, &mut mem);

        assert_eq!(0x30, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((false, false, true, false), read_flags(&reg_file));
    }

    // ===== CPL =====

    //CPL flips every bit of A and sets N and H, leaving Z and C exactly as they were
    #[test]
    fn test_cpl_inverts_accumulator_and_preserves_zero_and_carry() {
        let (mut reg_file, _) = regfile_with(0b0011_0101, 0x00);
        reg_file.write_flags(true, false, false, true);

        let cycles = op_cpl(&mut reg_file);

        assert_eq!(0b1100_1010, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(4, cycles);
        assert_eq!((true, true, true, true), read_flags(&reg_file), "CPL must not touch Z or C");
    }

    //Z and C stay clear across a CPL too: CPL never computes them, it copies them
    #[test]
    fn test_cpl_does_not_set_zero_on_zero_result() {
        let (mut reg_file, _) = regfile_with(0xFF, 0x00);
        reg_file.write_flags(false, false, false, false);

        op_cpl(&mut reg_file);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, true, false), read_flags(&reg_file), "a zero result must not set Z");
    }

    //Two CPLs in a row are an identity on A
    #[test]
    fn test_cpl_twice_restores_accumulator() {
        let (mut reg_file, _) = regfile_with(0x5A, 0x00);

        op_cpl(&mut reg_file);
        op_cpl(&mut reg_file);

        assert_eq!(0x5A, reg_file.read_register(DmgSimpleRegisters::A));
    }

    // ===== OR =====

    //OR clears every flag but Z, including an H that AND may have left behind
    #[test]
    fn test_or_acc_r8_clears_all_flags_but_zero() {
        let (mut reg_file, _) = regfile_with(0b1010_0000, 0x00);
        reg_file.write_register(DmgSimpleRegisters::D, 0b0000_0101);
        reg_file.write_flags(true, true, true, true);

        let cycles = op_or_acc_r8(&mut reg_file, DmgSimpleRegisters::D);

        assert_eq!(0b1010_0101, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(4, cycles);
        assert_eq!((false, false, false, false), read_flags(&reg_file), "OR must clear N, H and C");
    }

    //OR only reports zero when both operands are zero
    #[test]
    fn test_or_acc_n8_zero_result_sets_zero_flag() {
        let (mut reg_file, _) = regfile_with(0x00, 0x00);

        let cycles = op_or_acc_n8(&mut reg_file, 0x00);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((true, false, false, false), read_flags(&reg_file));
    }

    //OR A,A leaves A alone and is used purely to refresh Z
    #[test]
    fn test_or_acc_r8_source_is_accumulator() {
        let (mut reg_file, _) = regfile_with(0x3C, 0x00);

        op_or_acc_r8(&mut reg_file, DmgSimpleRegisters::A);

        assert_eq!(0x3C, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, false, false), read_flags(&reg_file));
    }

    //OR A,[HL] reads [HL] only: the trailing `source` param is a dead copy-paste artifact,
    //so passing a register whose value would change the result must not change it
    #[test]
    fn test_or_acc_hl_reads_memory_and_ignores_source_param() {
        let (mut reg_file, mut mem) = regfile_with(0x0A, 0x50);
        reg_file.write_register(DmgSimpleRegisters::B, 0xFF);

        let cycles = op_or_acc_hl(&mut reg_file, &mut mem, DmgSimpleRegisters::B);

        assert_eq!(0x5A, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((false, false, false, false), read_flags(&reg_file));
    }

    // ===== XOR =====

    //XOR clears every flag but Z
    #[test]
    fn test_xor_acc_r8_clears_all_flags_but_zero() {
        let (mut reg_file, _) = regfile_with(0b1111_0000, 0x00);
        reg_file.write_register(DmgSimpleRegisters::E, 0b1010_1010);
        reg_file.write_flags(true, true, true, true);

        let cycles = op_xor_acc_r8(&mut reg_file, DmgSimpleRegisters::E);

        assert_eq!(0b0101_1010, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(4, cycles);
        assert_eq!((false, false, false, false), read_flags(&reg_file), "XOR must clear N, H and C");
    }

    //XOR A,A is the canonical "clear A": zero result, Z set
    #[test]
    fn test_xor_acc_r8_source_is_accumulator_clears_accumulator() {
        let (mut reg_file, _) = regfile_with(0xFF, 0x00);

        op_xor_acc_r8(&mut reg_file, DmgSimpleRegisters::A);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((true, false, false, false), read_flags(&reg_file));
    }

    //XOR against 0xFF is a bitwise not of A, matching what CPL computes
    #[test]
    fn test_xor_acc_n8_with_ones_inverts_accumulator() {
        let (mut reg_file, _) = regfile_with(0b0011_0101, 0x00);

        let cycles = op_xor_acc_n8(&mut reg_file, 0xFF);

        assert_eq!(0b1100_1010, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((false, false, false, false), read_flags(&reg_file));
    }

    //XOR A,[HL] takes its operand from the byte HL points at and costs 8 cycles
    #[test]
    fn test_xor_acc_hl_reads_memory() {
        let (mut reg_file, mut mem) = regfile_with(0xF0, 0x3C);

        let cycles = op_xor_acc_hl(&mut reg_file, &mut mem);

        assert_eq!(0xCC, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!(8, cycles);
        assert_eq!((false, false, false, false), read_flags(&reg_file));
    }

    //Equal operands cancel to zero, which sets Z
    #[test]
    fn test_xor_acc_n8_equal_operands_set_zero_flag() {
        let (mut reg_file, _) = regfile_with(0x3C, 0x00);

        op_xor_acc_n8(&mut reg_file, 0x3C);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((true, false, false, false), read_flags(&reg_file));
    }
}
