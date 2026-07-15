use crate::cpu::regfile::Regfile;
use crate::cpu::decoder::OpTable;
use crate::cpu::decoder::OpEntry;
use crate::mem::mem::Memory;

pub struct Cpu {
    regfile: Regfile,
    decoder: OpTable,
}

impl Cpu {

    pub fn new() -> Self {
        Cpu { 
            regfile: Regfile::new(), 
            decoder: OpTable::new(), 
        }
    }

    //Loads the provided cartride into memory, handles the memory banks if needed
    pub fn load_cartridge(path: &'static str) -> () {

    }

    //Loads the provided bootrom into memory to replicate the GB's usual work flow
    pub fn load_bootrom(path: &'static str) -> () {

    }

    //Fetches, decodes and executes the next instruction, then return the cycles used in said instruction (always T-cycles)
    pub fn step(&mut self, mem: &mut Memory) -> u8 {
        //FETCH: read the opcode at PC and advance PC past it. Each instruction's
        //closure then fetches its own operands (also advancing PC), so once EXECUTE
        //returns PC already points at the next instruction. Jumps overwrite PC outright.
        let next_op_code: u8 = self.regfile.fetch_byte(mem); //FETCH

        //0xCB switches to the CB-prefixed opcode table for the following byte instead of being a regular opcode
        if next_op_code == 0xCB {
            let cb_op_code: u8 = self.regfile.fetch_byte(mem); //FETCH
            let instruction: &OpEntry = self.decoder.decode_cb(cb_op_code); //DECODE
            return (instruction.get_instruction())(&mut self.regfile, mem); //EXECUTE
        }

        let instruction: &OpEntry = self.decoder.decode(next_op_code); //DECODE
        let cycles_elapsed = (instruction.get_instruction())(&mut self.regfile, mem); //EXECUTE

        cycles_elapsed
    }



}