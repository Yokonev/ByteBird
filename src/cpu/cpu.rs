use crate::{
    cpu::{
        decoder::{OpTable, OpEntry}, regfile::Regfile
    }, mem::mem::Memory
};

pub struct Cpu {
    regfile: Regfile,
    decoder: OpTable,
    memory: Memory //Cpu has ownership on the ram
}

impl Cpu {

    pub fn new() -> Self {
        Cpu { 
            regfile: Regfile::new(), 
            decoder: OpTable::new(), 
            memory: Memory::new() 
        }
    }

    //Loads the provided cartride into memory, handles the memory banks if needed
    pub fn load_cartridge(path: &'static str) -> () {

    }

    //Loads the provided bootrom into memory to replicate the GB's usual work flow
    pub fn load_bootrom(path: &'static str) -> () {

    }

    //Fetches, decodes and executes the next instruction, then return the cycles used in said instruction (always T-cycles)
    pub fn step(&mut self) -> u8 {
        let current_pc: u16 = self.regfile.read_pc();

        let next_op_code: u8 = self.memory.read_mem(current_pc); //FETCH
        let instruction: OpEntry = self.decoder.decode(next_op_code); //DECODE
        let cycles_elapsed = (instruction.get_instruction())(&mut self.regfile, &mut self.memory); //EXECUTE
        
        cycles_elapsed
    }



}