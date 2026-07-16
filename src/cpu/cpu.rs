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
   
        let ime_takes_effect: bool = self.regfile.read_ime_pending();

        let next_op_code: u8 = self.regfile.fetch_byte(mem); //FETCH and advance PC

        let cycles_elapsed: u8 = if next_op_code == 0xCB {
            let cb_op_code: u8 = self.regfile.fetch_byte(mem); //FETCH
            let instruction: &OpEntry = self.decoder.decode_cb(cb_op_code); //DECODE
            (instruction.get_instruction())(&mut self.regfile, mem) //EXECUTE
        } else {
            let instruction: &OpEntry = self.decoder.decode(next_op_code); //DECODE
            (instruction.get_instruction())(&mut self.regfile, mem) //EXECUTE
        };

        //Re-reading the flag lets a DI in this instruction cancel the pending enable,
        //so an EI immediately followed by DI never enables interrupts.
        if ime_takes_effect && self.regfile.read_ime_pending() {
            self.regfile.write_ime(true);
            self.regfile.write_ime_pending(false);
        }

        cycles_elapsed
    }



}

#[cfg(test)]
mod tests {

    use super::*;

    const NOP: u8 = 0x00;
    const EI: u8 = 0xFB;
    const DI: u8 = 0xF3;
    const PREFIX_CB: u8 = 0xCB;
    const SWAP_A: u8 = 0x37; //CB-prefixed

    //Loads the given bytes into work RAM at 0xC000 and points PC at them
    fn cpu_running(program: &[u8]) -> (Cpu, Memory) {
        let mut mem: Memory = Memory::new();
        for (offset, byte) in program.iter().enumerate() {
            mem.write_mem_8(0xC000 + offset as u16, *byte);
        }
        let mut cpu: Cpu = Cpu::new();
        cpu.regfile.write_pc(0xC000);
        (cpu, mem)
    }

    //EI must not raise IME on its own instruction -- only once the *following* one completes
    #[test]
    fn test_ei_enables_ime_one_instruction_late() {
        let (mut cpu, mut mem) = cpu_running(&[EI, NOP, NOP]);

        cpu.step(&mut mem); //EI
        assert_eq!(false, cpu.regfile.read_ime(), "EI must not set IME on its own instruction");

        cpu.step(&mut mem); //NOP following EI
        assert_eq!(true, cpu.regfile.read_ime(), "IME must be set once the instruction after EI completes");
    }

    //DI immediately after EI cancels the pending enable, so IME never comes up
    #[test]
    fn test_di_cancels_pending_ei() {
        let (mut cpu, mut mem) = cpu_running(&[EI, DI, NOP]);

        cpu.step(&mut mem); //EI
        cpu.step(&mut mem); //DI cancels it
        assert_eq!(false, cpu.regfile.read_ime(), "DI must cancel an EI that has not taken effect");

        cpu.step(&mut mem); //NOP
        assert_eq!(false, cpu.regfile.read_ime(), "cancelled EI must not resurface on a later instruction");
    }

    //Regression: the CB path must apply the pending enable too, not bypass it
    #[test]
    fn test_ei_takes_effect_after_cb_instruction() {
        let (mut cpu, mut mem) = cpu_running(&[EI, PREFIX_CB, SWAP_A]);

        cpu.step(&mut mem); //EI
        assert_eq!(false, cpu.regfile.read_ime());

        cpu.step(&mut mem); //CB-prefixed instruction following EI
        assert_eq!(true, cpu.regfile.read_ime(), "a CB-prefixed instruction must still apply EI's pending enable");
    }

    //A pending EI stays pending across exactly one instruction, not two
    #[test]
    fn test_ime_stays_disabled_until_ei_lands() {
        let (mut cpu, mut mem) = cpu_running(&[NOP, EI, NOP]);

        cpu.step(&mut mem); //NOP before EI
        assert_eq!(false, cpu.regfile.read_ime());

        cpu.step(&mut mem); //EI
        assert_eq!(false, cpu.regfile.read_ime());

        cpu.step(&mut mem); //NOP following EI
        assert_eq!(true, cpu.regfile.read_ime());
    }
}