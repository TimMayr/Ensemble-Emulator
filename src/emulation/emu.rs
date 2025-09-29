use std::ops::RangeInclusive;

use crate::emulation::nes::Nes;
use crate::frontend::Frontends;

pub const WIDTH: u32 = 272;
pub const HEIGHT: u32 = 128;

pub enum Consoles {
    Nes(Nes),
}

impl Console for Consoles {
    fn get_pixel_buffer(&self) -> [u32; (WIDTH * HEIGHT) as usize] {
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

    fn run(&mut self, frontend: &mut Option<Frontends>) -> Result<(), String> {
        match self {
            Consoles::Nes(nes) => nes.run(frontend),
        }
    }

    fn run_until(&mut self,
                 frontend: &mut Option<Frontends>,
                 last_cycle: u128)
                 -> Result<(), String> {
        match self {
            Consoles::Nes(nes) => nes.run_until(frontend, last_cycle),
        }
    }

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>> {
        match self {
            Consoles::Nes(nes) => nes.get_memory_debug(range),
        }
    }
}

pub trait Console {
    fn get_pixel_buffer(&self) -> [u32; (WIDTH * HEIGHT) as usize];
    #[allow(clippy::ptr_arg)]
    fn load_rom(&mut self, path: &String);
    fn reset(&mut self);
    fn run(&mut self, option: &mut Option<Frontends>) -> Result<(), String>;
    fn run_until(&mut self,
                 frontend: &mut Option<Frontends>,
                 last_cycle: u128)
                 -> Result<(), String>;

    fn get_memory_debug(&self, range: Option<RangeInclusive<u16>>) -> Vec<Vec<u8>>;
}
