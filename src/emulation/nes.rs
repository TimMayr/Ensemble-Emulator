use crate::emulation::cpu::Cpu;
use crate::emulation::emu::{Console, HEIGHT, WIDTH};
use crate::emulation::mem::Memory;
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::ppu_registers::PpuRegisters;
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::{RomFile, RomFileConvertible};
use crate::emulation::savestate;
use crate::emulation::savestate::{CpuState, PpuState, SaveState};
use crate::frontend::{Frontend, Frontends};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

pub const CPU_CYCLES_PER_FRAME: u16 = 29780;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
pub const MASTER_CYCLES_PER_FRAME: u32 = 357366;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<Ppu>>,
    pub cycles: u128,
    pub rom_file: Option<RomFile>,
}

impl Console for Nes {
    fn get_pixel_buffer(&self) -> [u32; (WIDTH * HEIGHT) as usize] {
        self.ppu.borrow().pixel_buffer
    }

    fn load_rom(&mut self, path: &String) {
        self.load_rom(path);
    }

    fn reset(&mut self) {
        self.reset()
    }

    fn run(&mut self, possible_frontend: &mut Option<Frontends>) -> Result<(), String> {
        let mut leftover_cpu_cycles = 0;
        loop {
            self.cycles += 1;

            if self.cycles.is_multiple_of(12) {
                if leftover_cpu_cycles == 0 {
                    leftover_cpu_cycles = self.cpu.step();
                }

                leftover_cpu_cycles -= 1;
            }

            if self.cycles.is_multiple_of(4) {
                self.ppu.borrow_mut().step();
            }

            if self.cycles.is_multiple_of(MASTER_CYCLES_PER_FRAME as u128) {
                if let Some(frontend) = possible_frontend.as_mut() {
                    if let Err(res) = frontend.show_frame(&self.get_pixel_buffer()) {
                        return Err(res);
                    }
                }
            }
        }
    }
}

impl Nes {
    pub fn new(cpu: Cpu, ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            cpu,
            ppu,
            cycles: 0,
            rom_file: None,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.memory.add_memory(
            0x2000..=0x3FFF,
            Memory::MirrorMemory(MirrorMemory::new(
                Box::new(Memory::PpuRegisters(PpuRegisters::new(self.ppu.clone()))),
                0x0007,
            )),
        );

        self.cpu.ppu = Some(self.ppu.clone());

        self.cpu.reset();
        self.ppu.borrow_mut().reset();
    }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        self.cpu.load_rom(&rom_file);
        self.ppu.borrow_mut().load_rom(&rom_file);
        self.rom_file = Some(rom_file);
    }

    pub fn save_state(&self, path: &str) {
        let ppu_state = {
            let ppu_ref = self.ppu.borrow();
            PpuState::from(ppu_ref.deref())
        };

        let state = SaveState {
            cpu: CpuState::from(&self.cpu),
            ppu: ppu_state,
            cycles: self.cycles,
            rom_file: self.rom_file.as_ref().unwrap().clone(),
            version: 1,
        };

        savestate::save_state(state, path);
    }

    pub fn load_state(&mut self, path: &str) {
        let state = savestate::load_state(path);

        self.rom_file = Some(state.rom_file);
        self.ppu = Rc::new(RefCell::new(Ppu::from(
            &state.ppu,
            self.rom_file.as_ref().unwrap(),
        )));
        self.cpu = Cpu::from(
            &state.cpu,
            self.ppu.clone(),
            self.rom_file.as_ref().unwrap(),
        );
        self.cycles = state.cycles;

        self.cpu.memory.load(&state.cpu.memory);
        self.ppu.borrow_mut().memory.load(&state.ppu.memory);
    }
}

impl Default for Nes {
    fn default() -> Self {
        let cpu = Cpu::new();
        let ppu = Rc::new(RefCell::new(Ppu::default()));
        Nes::new(cpu, ppu)
    }
}
