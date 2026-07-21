use crate::cpu::regfile::{self, *};
use crate::mem::mem::Memory;

//DAA
pub fn op_daa(reg_file: &mut Regfile) -> u8 {
    
    let mut acc = reg_file.read_register(DmgSimpleRegisters::A);
    let mut adjustment: u8 = 0;
    let mut carry = reg_file.read_flag(DmgFlags::Carry);

    if reg_file.read_flag(DmgFlags::Subtract) {
        
        if reg_file.read_flag(DmgFlags::HCarry) {
            adjustment = adjustment.wrapping_add(0x6);
        }

        if carry {
            adjustment = adjustment.wrapping_add(0x60);
        }

        acc = acc.wrapping_sub(adjustment);
        reg_file.write_register(DmgSimpleRegisters::A, acc);

    }
    else {

        if (reg_file.read_flag(DmgFlags::HCarry)) || ((acc & 0xF)  > 0x9) {
            adjustment = adjustment.wrapping_add(0x6);
        }
        if (carry) || (acc > 0x99) {
            adjustment = adjustment.wrapping_add(0x60);
            carry = true;
        }

        acc = acc.wrapping_add(adjustment);
        reg_file.write_register(DmgSimpleRegisters::A, acc);

    }

    reg_file.write_flags(
        acc == 0, 
        reg_file.read_flag(DmgFlags::Subtract), 
        false, 
        carry
    );
    4u8
}

//NOP
pub fn op_nop() -> u8 {
    4u8
}

//STOP  — 0x10 0x00
pub fn op_stop(reg_file: &mut Regfile, memory: &mut Memory) -> u8 {
    let _pad = reg_file.fetch_byte(memory);
    reg_file.write_stopped(true);

    // TODO: reset DIV (0xFF04) to 0 once the timer/divider exists.
    // TODO: real DMG wakeup is a joypad line transition (P1/JOYP, 0xFF00); the main loop
    //       must clear `stopped` when that subsystem lands.
    4u8
}

#[cfg(test)]
mod tests {

    use super::*;

    fn read_flags(reg_file: &Regfile) -> (bool, bool, bool, bool) {
        (
            reg_file.read_flag(DmgFlags::Zero),
            reg_file.read_flag(DmgFlags::Subtract),
            reg_file.read_flag(DmgFlags::HCarry),
            reg_file.read_flag(DmgFlags::Carry),
        )
    }

    //A regfile with A preloaded and the four flags set to the given values
    fn daa_setup(acc: u8, n: bool, h: bool, c: bool) -> Regfile {
        let mut reg_file: Regfile = Regfile::new();
        reg_file.write_register(DmgSimpleRegisters::A, acc);
        reg_file.write_flags(false, n, h, c);
        reg_file
    }

    // ===== DAA (after addition, N clear) =====

    //A half-carry (or low nibble > 9) triggers the +0x06 correction; here C must stay clear.
    //Regression guard: an earlier bug set C unconditionally after every add-DAA.
    #[test]
    fn test_daa_after_add_low_nibble_correction_keeps_carry_clear() {
        let mut reg_file = daa_setup(0x11, false, true, false); // e.g. 0x08 + 0x09
        let cycles = op_daa(&mut reg_file);

        assert_eq!(0x17, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, false, false), read_flags(&reg_file));
        assert_eq!(4, cycles);
    }

    //A carry-in propagates the +0x60 correction and DAA re-sets the carry flag.
    #[test]
    fn test_daa_after_add_carry_in_applies_high_correction() {
        let mut reg_file = daa_setup(0x10, false, false, true); // e.g. 0x90 + 0x80
        op_daa(&mut reg_file);

        assert_eq!(0x70, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, false, false, true), read_flags(&reg_file));
    }

    //acc > 0x99 sets the carry even with C clear; the double correction here wraps to zero.
    #[test]
    fn test_daa_after_add_high_nibble_overflow_sets_carry_and_zero() {
        let mut reg_file = daa_setup(0x9A, false, false, false);
        op_daa(&mut reg_file);

        assert_eq!(0x00, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((true, false, false, true), read_flags(&reg_file));
    }

    // ===== DAA (after subtraction, N set) =====

    //After a subtraction DAA only corrects on the stored H/C flags; H alone subtracts 0x06
    //and leaves the carry untouched.
    #[test]
    fn test_daa_after_sub_half_borrow_subtracts_six() {
        let mut reg_file = daa_setup(0x0D, true, true, false); // e.g. 0x45 - 0x38
        op_daa(&mut reg_file);

        assert_eq!(0x07, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, false, false), read_flags(&reg_file));
    }

    //A borrow (C set) subtracts 0x60 and the carry is preserved across the correction.
    #[test]
    fn test_daa_after_sub_full_borrow_preserves_carry() {
        let mut reg_file = daa_setup(0xF5, true, false, true); // e.g. 0x05 - 0x10
        op_daa(&mut reg_file);

        assert_eq!(0x95, reg_file.read_register(DmgSimpleRegisters::A));
        assert_eq!((false, true, false, true), read_flags(&reg_file));
    }

    // ===== STOP =====

    //STOP is a 2-byte opcode: it must consume the padding byte (advancing PC past it,
    //whatever its value) and latch the stopped state.
    #[test]
    fn test_stop_consumes_pad_byte_and_sets_stopped() {
        let mut reg_file: Regfile = Regfile::new();
        let mut mem: Memory = Memory::new();
        reg_file.write_pc(0xC000);
        mem.write_mem_8(0xC000, 0xFF); // non-zero pad must still be consumed

        let cycles = op_stop(&mut reg_file, &mut mem);

        assert!(reg_file.read_stopped());
        assert_eq!(0xC001, reg_file.read_pc(), "PC must advance past the pad byte");
        assert_eq!(4, cycles);
    }
}
