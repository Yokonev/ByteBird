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
