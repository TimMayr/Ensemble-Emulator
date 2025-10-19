use std::cell::Ref;
use std::ops::RangeInclusive;

use crate::emulation::nes::{ExecutionFinishedType, Nes};
use crate::frontend::Frontends;

pub const TOTAL_OUTPUT_WIDTH: u32 = 256 * 1;
pub const TOTAL_OUTPUT_HEIGHT: u32 = 240 * 1;

pub enum Consoles {
    Nes(Nes),
}

impl Console for Consoles {
    #[inline(always)]
    fn get_pixel_buffer(
        &self,
    ) -> Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]> {
        match self {
            Consoles::Nes(nes) => nes.get_pixel_buffer(),
        }
    }

    #[inline(always)]
    fn load_rom(&mut self, path: &String) {
        match self {
            Consoles::Nes(nes) => nes.load_rom(path),
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        match self {
            Consoles::Nes(nes) => nes.reset(),
        }
    }

    #[inline(always)]
    fn power(&mut self) {
        match self {
            Consoles::Nes(nes) => nes.power(),
        }
    }

    #[inline(always)]
    fn run(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.run(frontend),
        }
    }

    #[inline(always)]
    fn run_until(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.run_until(frontend, last_cycle),
        }
    }

    #[inline(always)]
    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>> {
        match self {
            Consoles::Nes(nes) => nes.get_memory_debug(range),
        }
    }

    #[inline(always)]
    fn set_trace_log_path(&mut self, path: Option<String>) {
        match self {
            Consoles::Nes(nes) => nes.set_trace_log_path(path),
        }
    }

    #[inline(always)]
    fn flush_trace_log(&mut self) {
        match self {
            Consoles::Nes(nes) => nes.flush_trace_log(),
        }
    }

    #[inline(always)]
    fn step(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.step(frontend, u128::MAX),
        }
    }

    #[inline(always)]
    fn step_frame(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.step_frame(frontend),
        }
    }
}

pub trait Console {
    fn get_pixel_buffer(
        &self,
    ) -> Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>;
    #[allow(clippy::ptr_arg)]
    fn load_rom(&mut self, path: &String);
    fn reset(&mut self);
    fn power(&mut self);

    fn run(&mut self, option: &mut Frontends) -> Result<ExecutionFinishedType, String>;
    fn run_until(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String>;

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>>;
    fn set_trace_log_path(&mut self, path: Option<String>);
    fn flush_trace_log(&mut self);

    fn step(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String>;
    fn step_frame(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String>;
}

pub enum InputEvent {
    IncPalette,
    Quit,
}
