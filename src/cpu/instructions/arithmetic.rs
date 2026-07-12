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
