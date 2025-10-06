use std::cell::Ref;
use std::ops::RangeInclusive;

use crate::emulation::nes::{ExecutionFinishedType, Nes};
use crate::frontend::Frontends;

pub const WIDTH: u32 = 272;
pub const HEIGHT: u32 = 128;

pub enum Consoles {
    Nes(Nes),
}

impl Console for Consoles {
    fn get_pixel_buffer(&self) -> Ref<'_, [u32; (WIDTH * HEIGHT) as usize]> {
        match self {
            Consoles::Nes(nes) => nes.get_pixel_buffer(),
        }
    }

    fn load_rom(&mut self, path: &String) {
        match self {
            Consoles::Nes(nes) => nes.load_rom(path),
        }
    }

    fn reset(&mut self) {
        match self {
            Consoles::Nes(nes) => nes.reset(),
        }
    }

    fn run(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.run(frontend),
        }
    }

    fn run_until(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.run_until(frontend, last_cycle),
        }
    }

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>> {
        match self {
            Consoles::Nes(nes) => nes.get_memory_debug(range),
        }
    }

    fn set_trace_log_path(&mut self, path: Option<String>) {
        match self {
            Consoles::Nes(nes) => nes.set_trace_log_path(path),
        }
    }

    fn step(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.step(frontend, u128::MAX, None),
        }
    }

    fn step_frame(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.step_frame(frontend),
        }
    }
}

pub trait Console {
    fn get_pixel_buffer(&self) -> Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>;
    #[allow(clippy::ptr_arg)]
    fn load_rom(&mut self, path: &String);
    fn reset(&mut self);
    fn run(&mut self, option: &mut Frontends) -> Result<ExecutionFinishedType, String>;
    fn run_until(
        &mut self,
        frontend: &mut Frontends,
        last_cycle: u128,
    ) -> Result<ExecutionFinishedType, String>;

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>>;
    fn set_trace_log_path(&mut self, path: Option<String>);

    fn step(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String>;
    fn step_frame(&mut self, frontend: &mut Frontends) -> Result<ExecutionFinishedType, String>;
}
