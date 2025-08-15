use crate::cpu::Cpu;
use crate::mem::mirror_memory::MirrorMemory;
use crate::mem::ppu_registers::PpuRegisters;
use crate::ppu::PpuStub;
use crate::rom::{RomFile, RomFileConvertible};
use crate::savestate;
use crate::savestate::{CpuState, PpuState, SaveState};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

pub const CPU_CYCLES_PER_FRAME: u16 = 29780;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
pub const MASTER_CYCLES_PER_FRAME: i32 = 357366;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<PpuStub>>,
    pub cycles: u64,
    pub rom_file: Option<RomFile>,
}

impl Nes {
    pub fn new(cpu: Cpu, ppu_stub: Rc<RefCell<PpuStub>>) -> Self {
        Self {
            cpu,
            ppu: ppu_stub,
            cycles: 0,
            rom_file: None,
        }
    }

    pub fn init(&mut self) {
        self.cpu.memory.add_memory(
            0x2000..=0x3FFF,
            Box::new(MirrorMemory::new(
                Box::new(PpuRegisters::new(self.ppu.clone())),
                0x0007,
            )),
        );

        self.cpu.ppu = Some(self.ppu.clone());

        self.cpu.reset()
    }

    pub fn run(&mut self) {
        let mut leftover_cpu_cycles = 0;
        loop {
            self.cycles += 1;

            if self.cycles.is_multiple_of(12) {
                if leftover_cpu_cycles == 0 {
                    leftover_cpu_cycles = self.cpu.step();
                }

                if leftover_cpu_cycles == 0xFF {
                    return;
                }

                leftover_cpu_cycles -= 1;
            }

            if self.cycles.is_multiple_of(4) {
                self.ppu.borrow_mut().step();
            }
        }
    }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        self.cpu.load_rom(&rom_file);
        self.rom_file = Some(rom_file);
    }

    pub fn save_state(&self, path: &str) {
        let state = SaveState {
            cpu: CpuState::from(&self.cpu),
            ppu: PpuState::from(self.ppu.borrow().deref()),
            cycles: self.cycles,
            memory: self
                .cpu
                .memory
                .get_memory_debug(0x00..=0xFFFF)
                .as_slice()
                .try_into()
                .expect("Wrong memory length"),
            rom_file: self.rom_file.as_ref().unwrap().clone(),
        };

        savestate::save_state(state, path);
    }

    pub fn load_state(&mut self, path: &str) {
        let state = savestate::load_state(path);

        self.rom_file = Some(state.rom_file);
        self.ppu = Rc::new(RefCell::new(PpuStub::from(state.ppu)));
        self.cpu = Cpu::from(
            state.cpu,
            self.ppu.clone(),
            self.rom_file.as_ref().unwrap().clone(),
        );
        self.cycles = state.cycles;

        self.cpu.memory.load(&state.memory)
    }
}
