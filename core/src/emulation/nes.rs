use std::cell::{Ref, RefCell};
use std::ops::{Deref, RangeInclusive};
use std::rc::Rc;
use std::time::Duration;

use crate::emulation::cpu::{Cpu, MicroOp};
use crate::emulation::emu::{Console, InputEvent};
use crate::emulation::mem::Memory;
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::ppu_registers::PpuRegisters;
use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::{RomFile, RomFileConvertible};
use crate::emulation::savestate;
use crate::emulation::savestate::{CpuState, PpuState, SaveState};
use crate::frontend::{Frontend, Frontends};
use crate::trace::TraceLog;
use crate::util;

pub const CPU_CYCLES_PER_FRAME: u16 = 29780;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
pub const MASTER_CYCLES_PER_FRAME: u32 = 357366;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<Ppu>>,
    pub total_cycles: u128,
    pub rom_file: Option<RomFile>,
    pub trace_log: Option<TraceLog>,
    pub cpu_cycle_counter: u8,
    pub ppu_cycle_counter: u8,
}

impl Console for Nes {
    #[inline]
    fn get_pixel_buffer(&self) -> Ref<'_, [u32; TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT]> {
        Ref::map(self.ppu.borrow(), |ppu| ppu.get_pixel_buffer())
    }

    fn load_rom(&mut self, path: &String) { self.load_rom(path); }

    fn reset(&mut self) { self.reset() }

    fn power(&mut self) {
        self.cpu.ppu = Some(self.ppu.clone());

        self.cpu.memory.add_memory(
            0x2000..=0x3FFF,
            Memory::MirrorMemory(MirrorMemory::new(
                Box::new(Memory::PpuRegisters(PpuRegisters::new(self.ppu.clone()))),
                0x0007,
            )),
        );

        self.cpu.reset();
    }

    fn run(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        self.run_until(frontend, u128::MAX)
    }

    fn run_until(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String> {
        loop {
            let res = self.step(frontend, last_cycle);
            match res {
                Ok(ExecutionFinishedType::CycleCompleted) => {
                    continue;
                }
                Ok(res) => {
                    if let Some(ref mut trace) = self.trace_log {
                        trace.flush();
                    }

                    return Ok(res);
                }
                Err(err) => {
                    if let Some(ref mut trace) = self.trace_log {
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
            self.trace_log = None;
            return;
        }

        if self.trace_log.is_none() {
            self.trace_log = Some(TraceLog::new(path.clone().unwrap()));
        }

        if let Some(ref mut trace) = self.trace_log {
            trace.output = path.unwrap();
        }
    }

    fn flush_trace_log(&mut self) {
        if let Some(ref mut trace) = self.trace_log {
            trace.flush()
        }
    }

    #[inline]
    fn step(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        self.step(frontend, u128::MAX)
    }

    #[inline]
    fn step_frame(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        self.run_until(
            frontend,
            self.total_cycles + MASTER_CYCLES_PER_FRAME as u128,
        )
    }
}

impl Nes {
    pub fn new(cpu: Cpu, ppu: Rc<RefCell<Ppu>>) -> Self {
        Self {
            cpu,
            ppu,
            rom_file: None,
            trace_log: None,
            total_cycles: 0,
            cpu_cycle_counter: 0,
            ppu_cycle_counter: 0,
        }
    }

    pub fn reset(&mut self) {
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
            cycle: self.cpu_cycle_counter,
            total_cycles: self.total_cycles,
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
        self.cpu_cycle_counter = state.cycle;

        self.cpu.memory.load(&state.cpu.memory);
        self.ppu.borrow_mut().memory.load(&state.ppu.memory);
    }

    #[inline]
    pub fn step(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String> {
        self.total_cycles += 1;
        self.cpu_cycle_counter = self.cpu_cycle_counter.wrapping_add(1);
        self.ppu_cycle_counter = self.ppu_cycle_counter.wrapping_add(1);

        let ppu = self.ppu.borrow();
        if ppu.vbl_clear_scheduled.get().is_some() {
            ppu.vbl_reset_counter.set(ppu.vbl_reset_counter.get() + 1);
            ppu.process_vbl_clear_scheduled();
        }
        drop(ppu);

        if self.total_cycles > last_cycle {
            self.total_cycles -= 1;
            return Ok(ExecutionFinishedType::ReachedLastCycle);
        };

        let mut cpu_res = Ok(ExecutionFinishedType::CycleCompleted);

        let mut frame_ready = false;
        if self.ppu_cycle_counter == 4 {
            frame_ready = self.ppu.borrow_mut().step();
            self.ppu_cycle_counter = 0;
        }

        // if frame_ready && !matches!(frontend, Frontends::None()) {
        //     self.ppu.borrow_mut().frame();
        // }

        if frame_ready && !matches!(frontend, Frontends::None()) {
            // Debug views are now rendered on-demand only via explicit frontend requests
            // This eliminates the massive performance cost of rendering 245,760+ pixels every frame
            #[cfg(feature = "sdl2-frontend")]
            {
                let pixel_buffer = self.get_pixel_buffer();
                frontend.show_frame(pixel_buffer)?;
            }

            #[cfg(not(feature = "sdl2-frontend"))]
            {
                frontend.show_frame()?;
            }

            let res = frontend.poll_input_events();
            if let Ok(events) = res {
                for event in events {
                    self.handle_input_event(event);
                }
            } else if let Err(s) = res {
                panic!("{s}")
            }
        }

        if self.cpu_cycle_counter.wrapping_add(2) == 12 {
            let mut do_trace = false;

            if self.trace_log.is_some() && matches!(&self.cpu.current_op, &MicroOp::FetchOpcode(..))
            {
                do_trace = true;
            }

            cpu_res = self.cpu.step();

            if do_trace && let Some(ref mut trace) = self.trace_log {
                let ppu_state = {
                    let ppu_ref = self.ppu.borrow();
                    PpuState::from(ppu_ref.deref())
                };

                let state = SaveState {
                    cpu: CpuState::from(&self.cpu),
                    ppu: ppu_state,
                    cycle: self.cpu_cycle_counter,
                    total_cycles: self.total_cycles,
                    rom_file: self.rom_file.as_ref().unwrap().clone(),
                    version: 1,
                };

                trace.trace(state)
            }
        }

        if self.cpu_cycle_counter == 12 {
            self.cpu_cycle_counter = 0;
        }

        cpu_res
    }

    pub fn handle_input_event(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::IncPalette => {
                self.inc_current_palette();
            }
            InputEvent::Quit => {
                util::write_to_file("log/log.log", self.ppu.borrow().log.as_bytes().to_vec());
                panic!()
            }
        }
    }

    pub fn inc_current_palette(&mut self) { self.ppu.borrow_mut().inc_current_palette(); }
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
