use std::cell::{Ref, RefCell};
use std::mem::discriminant;
use std::ops::{Deref, RangeInclusive};
use std::rc::Rc;
use std::time::Duration;

use crate::emulation::cpu::{Cpu, MicroOp, MicroOpCallback};
use crate::emulation::emu::{Console, HEIGHT, WIDTH};
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::ppu_registers::PpuRegisters;
use crate::emulation::mem::Memory;
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::{RomFile, RomFileConvertible};
use crate::emulation::savestate;
use crate::emulation::savestate::{CpuState, PpuState, SaveState};
use crate::frontend::{Frontend, Frontends};
use crate::trace::TraceLog;

pub const CPU_CYCLES_PER_FRAME: u16 = 29780;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
pub const MASTER_CYCLES_PER_FRAME: u32 = 357366;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<Ppu>>,
    pub cycles: u128,
    pub rom_file: Option<RomFile>,
    pub trace_log: RefCell<Option<TraceLog>>,
}

impl Console for Nes {
    fn get_pixel_buffer(&self) -> Ref<'_, [u32; (WIDTH * HEIGHT) as usize]> {
        Ref::map(self.ppu.borrow(), |ppu| ppu.get_pixel_buffer())
    }

    fn load_rom(&mut self, path: &String) { self.load_rom(path); }

    fn reset(&mut self) { self.reset() }

    fn run(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        self.run_until(frontend, u128::MAX)
    }

    fn run_until(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String> {
        loop {
            self.cycles += 1;
            let res = self.step(frontend, last_cycle);
            match res {
                Ok(ExecutionFinishedType::CycleCompleted) => {
                    continue;
                }
                Ok(res) => {
                    if let Some(trace) = self.trace_log.borrow_mut().as_mut() {
                        trace.flush();
                    }

                    return Ok(res);
                }
                Err(err) => {
                    if let Some(ref mut trace) = self.trace_log.borrow_mut().as_mut() {
                        trace.flush();
                    }

                    panic!("{}", err)
                }
            };
        }
    }

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>> {
        vec![
            self.cpu.get_memory_debug(range.clone()),
            self.ppu.borrow().get_memory_debug(range.clone()),
        ]
    }

    fn set_trace_log_path(&mut self, path: Option<String>) {
        if path.is_none() {
            self.trace_log = RefCell::new(None);
            return;
        }

        if self.trace_log.borrow_mut().as_mut().is_none() {
            self.trace_log
                .replace(Some(TraceLog::new(path.clone().unwrap())));
        }

        if let Some(trace) = self.trace_log.borrow_mut().as_mut() {
            trace.output = path.unwrap();
        }
    }

    fn flush_trace_log(&mut self) {
        if let Some(trace) = self.trace_log.borrow_mut().as_mut() {
            trace.flush()
        }
    }

    fn step(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        self.step(frontend, u128::MAX)
    }

    fn step_frame(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        self.run_until(frontend, self.cycles + MASTER_CYCLES_PER_FRAME as u128)
    }
}

impl Nes {
    pub fn new(cpu: Cpu, ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            cpu,
            ppu,
            cycles: 0,
            rom_file: None,
            trace_log: RefCell::new(None),
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

    pub fn step(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String> {
        if self.cycles >= last_cycle {
            return Ok(ExecutionFinishedType::ReachedLastCycle);
        };

        if self.cycles.is_multiple_of(4) {
            self.ppu.borrow_mut().step(self.cycles);
        }

        let mut cpu_res = Ok(ExecutionFinishedType::CycleCompleted);

        if self.cycles.is_multiple_of(12) {
            let mut do_trace = false;

            if let Some(_) = &self.trace_log.get_mut()
                && discriminant(&self.cpu.current_op)
                    == discriminant(&MicroOp::FetchOpcode(MicroOpCallback::None))
            {
                do_trace = true;
            }

            cpu_res = self.cpu.step(self.cycles);

            if do_trace && let Some(trace) = self.trace_log.borrow_mut().as_mut() {
                trace.trace(self);
            }
        }

        if self.cycles.is_multiple_of(MASTER_CYCLES_PER_FRAME as u128)
            && discriminant(frontend) != discriminant(&Frontends::None())
        {
            self.ppu.borrow_mut().frame();
            frontend.show_frame(self.get_pixel_buffer())?
        }

        cpu_res
    }
}

impl Default for Nes {
    fn default() -> Self {
        let cpu = Cpu::new();
        let ppu = Rc::new(RefCell::new(Ppu::default()));
        Nes::new(cpu, ppu)
    }
}

#[derive(Debug)]
pub enum ExecutionFinishedType {
    ReachedLastCycle,
    ReachedHlt,
    CycleCompleted,
}
