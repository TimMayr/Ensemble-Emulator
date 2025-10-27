use std::ops::{Deref, RangeInclusive};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};

use crate::app::{AppToEmuMessages, EmuToAppMessages};
use crate::emulation::cpu::{Cpu, CpuExecutionResult, MicroOp};
use crate::emulation::emu::{Console, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::emulation::mem::Memory;
use crate::emulation::mem::mirror_memory::MirrorMemory;
use crate::emulation::mem::ppu_registers::PpuRegisters;
use crate::emulation::ppu::Ppu;
use crate::emulation::rom::{RomFile, RomFileConvertible};
use crate::emulation::savestate;
use crate::emulation::savestate::{CpuState, PpuState, SaveState};
use crate::trace::TraceLog;
use crate::util;

pub const CPU_CYCLES_PER_FRAME: u16 = 29780;
pub const FRAME_DURATION: Duration = Duration::from_nanos(16_666_667);
pub const MASTER_CYCLES_PER_FRAME: u32 = 357366;

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Arc<Mutex<Ppu>>,
    pub emu_sender: Option<Sender<EmuToAppMessages>>,
    pub emu_receiver: Option<Receiver<AppToEmuMessages>>,
    pub total_cycles: u128,
    pub rom_file: Option<RomFile>,
    pub trace_log: Option<TraceLog>,
    pub cpu_cycle_counter: u8,
    pub ppu_cycle_counter: u8,
    pub is_paused: bool,
}

impl Console for Nes {
    #[inline]
    fn with_pixel_buffer<R>(
        &self,
        f: impl FnOnce(&[u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]) -> R,
    ) -> R {
        let ppu = self.ppu.lock().unwrap();
        f(ppu.get_pixel_buffer())
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

    fn run(&mut self) -> Result<EmuExecutionFinishedType, String> { self.run_until(u128::MAX) }

    fn run_until(&mut self, last_cycle: u128) -> Result<EmuExecutionFinishedType, String> {
        loop {
            match self.step(last_cycle) {
                Ok(t) => match t {
                    EmuExecutionFinishedType::ReachedLastCycle
                    | EmuExecutionFinishedType::Quit
                    | EmuExecutionFinishedType::CpuHlt => {
                        return Ok(t);
                    }
                    EmuExecutionFinishedType::Running(_) | EmuExecutionFinishedType::Paused => {
                        continue;
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            };
        }
    }

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>> {
        vec![
            self.cpu.get_memory_debug(range.clone()),
            self.ppu.lock().unwrap().get_memory_debug(range.clone()),
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
    fn step(&mut self) -> Result<EmuExecutionFinishedType, String> { self.step(u128::MAX) }

    #[inline]
    fn step_frame(&mut self) -> Result<EmuExecutionFinishedType, String> {
        self.run_until(self.total_cycles + MASTER_CYCLES_PER_FRAME as u128)
    }

    fn set_message_receiver(&mut self, receiver: Receiver<AppToEmuMessages>) {
        self.emu_receiver = Some(receiver);
    }

    fn set_message_sender(&mut self, sender: Sender<EmuToAppMessages>) {
        self.emu_sender = Some(sender);
    }
}

impl Nes {
    pub fn new(cpu: Cpu, ppu: Arc<Mutex<Ppu>>) -> Self {
        Self {
            cpu,
            ppu,
            rom_file: None,
            trace_log: None,
            total_cycles: 0,
            cpu_cycle_counter: 0,
            ppu_cycle_counter: 0,
            emu_sender: None,
            emu_receiver: None,
            is_paused: false,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.ppu.lock().unwrap().reset();
    }

    pub fn load_rom<T: RomFileConvertible>(&mut self, rom_get: &T) {
        let rom_file = rom_get.as_rom_file();
        self.cpu.load_rom(&rom_file);
        self.ppu.lock().unwrap().load_rom(&rom_file);
        self.rom_file = Some(rom_file);
    }

    pub fn save_state(&self, path: &str) {
        let ppu_state = {
            let ppu_ref = self.ppu.lock().unwrap();
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
        self.ppu = Arc::new(Mutex::new(Ppu::from(
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
        self.ppu.lock().unwrap().memory.load(&state.ppu.memory);
    }

    pub fn handle_messages(&mut self) -> Result<(), EmuExecutionFinishedType> {
        if let Some(rec) = &self.emu_receiver {
            for msg in rec.try_iter() {
                return match msg {
                    AppToEmuMessages::Pause => {
                        self.set_paused(true);
                        Ok(())
                    }
                    AppToEmuMessages::Resume => {
                        self.set_paused(false);
                        Ok(())
                    }
                    AppToEmuMessages::Reset => {
                        self.reset();
                        Ok(())
                    }
                    AppToEmuMessages::Quit => {
                        util::write_to_file(
                            "log/log.log",
                            self.ppu.lock().unwrap().log.as_bytes().to_vec(),
                        );
                        Err(EmuExecutionFinishedType::Quit)
                    }
                    AppToEmuMessages::Power => {
                        self.power();
                        Ok(())
                    }
                    AppToEmuMessages::IncPalette => {
                        self.inc_current_palette();
                        Ok(())
                    }
                    AppToEmuMessages::TogglePause => {
                        self.set_paused(!self.is_paused);
                        Ok(())
                    }
                    AppToEmuMessages::LoadRom(path) => {
                        self.load_rom(&path);
                        Ok(())
                    }
                };
            }

            Ok(())
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn step(&mut self, last_cycle: u128) -> Result<EmuExecutionFinishedType, String> {
        if let Err(e) = self.handle_messages() {
            return Ok(e);
        }

        if !self.is_paused {
            self.total_cycles += 1;
            self.cpu_cycle_counter += 1;
            self.ppu_cycle_counter += 1;

            let ppu = self.ppu.lock().unwrap();
            if ppu.vbl_clear_scheduled.get().is_some() {
                ppu.vbl_reset_counter.set(ppu.vbl_reset_counter.get() + 1);
                ppu.process_vbl_clear_scheduled();
            }
            drop(ppu);

            if self.total_cycles > last_cycle {
                self.total_cycles -= 1;
                return Ok(EmuExecutionFinishedType::ReachedLastCycle);
            };

            let mut frame_ready = false;
            if self.ppu_cycle_counter == 4 {
                frame_ready = self.ppu.lock().unwrap().step();
                self.ppu_cycle_counter = 0;
            }

            let mut cpu_res = Ok(CpuExecutionResult::CycleCompleted);

            if self.cpu_cycle_counter.wrapping_add(2) == 12 {
                let mut do_trace = false;

                if self.trace_log.is_some()
                    && matches!(&self.cpu.current_op, &MicroOp::FetchOpcode(..))
                {
                    do_trace = true;
                }

                cpu_res = self.cpu.step();

                if do_trace && let Some(ref mut trace) = self.trace_log {
                    let ppu_state = {
                        let ppu_ref = self.ppu.lock().unwrap();
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

            if frame_ready {
                Ok(EmuExecutionFinishedType::Running(true))
            } else {
                match cpu_res {
                    Ok(t) => match t {
                        CpuExecutionResult::CycleCompleted => {
                            Ok(EmuExecutionFinishedType::Running(false))
                        }
                        CpuExecutionResult::Hlt => Ok(EmuExecutionFinishedType::CpuHlt),
                    },
                    Err(e) => Err(e),
                }
            }
        } else {
            Ok(EmuExecutionFinishedType::Paused)
        }
    }

    pub fn inc_current_palette(&mut self) { self.ppu.lock().unwrap().inc_current_palette(); }

    pub fn with_communication(
        emu_sender: Sender<EmuToAppMessages>,
        emu_receiver: Receiver<AppToEmuMessages>,
    ) -> Nes {
        let mut nes = Nes::default();
        nes.emu_sender = Some(emu_sender);
        nes.emu_receiver = Some(emu_receiver);
        nes
    }

    pub fn set_paused(&mut self, val: bool) { self.is_paused = val; }
}

impl Default for Nes {
    fn default() -> Self {
        let cpu = Cpu::new();
        let ppu = Arc::new(Mutex::new(Ppu::default()));
        Nes::new(cpu, ppu)
    }
}

#[derive(Debug)]
pub enum EmuExecutionFinishedType {
    ReachedLastCycle,
    Running(bool),
    Quit,
    Paused,
    CpuHlt,
}
