use std::ops::RangeInclusive;

use crossbeam_channel::{Receiver, Sender};

use crate::app::{AppToEmuMessages, EmuToAppMessages};
use crate::emulation::nes::{EmuExecutionFinishedType, Nes};

pub const SCREEN_WIDTH: u32 = 256;
pub const SCREEN_HEIGHT: u32 = 240;

pub enum Consoles {
    Nes(Nes),
}

impl Console for Consoles {
    #[inline(always)]
    fn with_pixel_buffer<R>(
        &self,
        f: impl FnOnce(&[u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]) -> R,
    ) -> R {
        match self {
            Consoles::Nes(nes) => nes.with_pixel_buffer(f),
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
    fn run(&mut self) -> Result<EmuExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.run(),
        }
    }

    #[inline(always)]
    fn run_until(&mut self, last_cycle: u128) -> Result<EmuExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.run_until(last_cycle),
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
    fn step(&mut self) -> Result<EmuExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.step(u128::MAX),
        }
    }

    #[inline(always)]
    fn step_frame(&mut self) -> Result<EmuExecutionFinishedType, String> {
        match self {
            Consoles::Nes(nes) => nes.step_frame(),
        }
    }

    fn set_message_receiver(&mut self, receiver: Receiver<AppToEmuMessages>) {
        match self {
            Consoles::Nes(nes) => nes.set_message_receiver(receiver),
        }
    }

    fn set_message_sender(&mut self, sender: Sender<EmuToAppMessages>) {
        match self {
            Consoles::Nes(nes) => nes.set_message_sender(sender),
        }
    }
}

pub trait Console {
    fn with_pixel_buffer<R>(
        &self,
        f: impl FnOnce(&[u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]) -> R,
    ) -> R;
    #[allow(clippy::ptr_arg)]
    fn load_rom(&mut self, path: &String);
    fn reset(&mut self);
    fn power(&mut self);

    fn run(&mut self) -> Result<EmuExecutionFinishedType, String>;
    fn run_until(&mut self, last_cycle: u128) -> Result<EmuExecutionFinishedType, String>;

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>>;
    fn set_trace_log_path(&mut self, path: Option<String>);
    fn flush_trace_log(&mut self);

    fn step(&mut self) -> Result<EmuExecutionFinishedType, String>;
    fn step_frame(&mut self) -> Result<EmuExecutionFinishedType, String>;

    fn set_message_receiver(&mut self, receiver: Receiver<AppToEmuMessages>);
    fn set_message_sender(&mut self, sender: Sender<EmuToAppMessages>);
}

pub enum InputEvent {
    IncPalette,
    Quit,
    Pause,
    Resume,
    TogglePause,
    Power,
    Reset,
    LoadRom(String),
}
