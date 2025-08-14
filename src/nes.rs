use crate::cpu::Cpu;
use crate::mem::mirror_memory::MirrorMemory;
use crate::mem::ppu_registers::PpuRegisters;
use crate::ppu::PpuStub;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub const CPU_CYCLES_PER_FRAME: u16 = 29780;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
pub const MASTER_CYCLES_PER_FRAME: i32 = 357366;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu_stub: Rc<RefCell<PpuStub>>,
}

impl Nes {
    pub fn new(cpu: Cpu, ppu_stub: Rc<RefCell<PpuStub>>) -> Self {
        Self { cpu, ppu_stub }
    }

    pub fn init(&mut self) {
        self.cpu.memory.add_memory(
            0x2000..=0x3FFF,
            Box::new(MirrorMemory::new(
                Box::new(PpuRegisters::new(self.ppu_stub.clone())),
                0x0007,
            )),
        );

        self.cpu.ppu = Some(self.ppu_stub.clone());

        self.cpu.reset()
    }

    pub fn run(&mut self) {
        let mut cycles: u64 = 0;
        let mut leftover_cpu_cycles = 0;
        loop {
            cycles += 1;

            if cycles % 12 == 0 {
                if leftover_cpu_cycles == 0 {
                    leftover_cpu_cycles = self.cpu.step();
                }

                leftover_cpu_cycles -= 1;
            }

            if cycles % 4 == 0 {
                self.ppu_stub.borrow_mut().step();
            }
        }
    }
}
