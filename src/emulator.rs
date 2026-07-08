use crate::cpu::cpu::Cpu;
use crate::cpu::regfile::Regfile;
use crate::mem::mem::Memory;

struct Emulator {
    cpu: Cpu,
    //dma: Dma,
    //ppu: Ppu,
    //timer: Timer
}

impl Emulator {
    pub fn new() -> Self {
        Self { 
            cpu: Cpu::new() }
    }

    pub fn run(&mut self) -> () {
        let stop_emu: bool = true;

        while !stop_emu {
            let cycles: u8 = self.cpu.step(); //Next instruction with total cycles count (t-cycles) returned at the end
            //self.timing.tick(cycles) //Timer handling
            //self.dma.tick(cycles) //Audio handling
            //self.ppu.tick(cycles) //Graphics handling
        };
    }
}