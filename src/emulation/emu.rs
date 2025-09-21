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
}

pub trait Console {
    fn get_pixel_buffer(&self) -> [u32; (WIDTH * HEIGHT) as usize];
    fn load_rom(&mut self, path: &String);
    fn reset(&mut self);
    fn run(&mut self, option: &mut Option<Frontends>) -> Result<(), String>;
}
