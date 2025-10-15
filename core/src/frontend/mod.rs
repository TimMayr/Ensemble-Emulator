pub mod godot_frontend;
#[cfg(feature = "sdl2")]
pub mod sdl_frontend;

use crate::emulation::emu::{Console, HEIGHT, WIDTH};
use crate::frontend::godot_frontend::GodotFrontend;
#[cfg(feature = "sdl2")]
use crate::frontend::sdl_frontend::SdlFrontend;

pub enum Frontends {
    #[cfg(feature = "sdl2")]
    Sdl2(SdlFrontend),
    Godot(GodotFrontend),
    None(),
}

impl Default for Frontends {
    fn default() -> Self { Frontends::None() }
}

impl Frontend for Frontends {
    fn show_frame(
        &mut self,
        pixel_buffer: &[u32; (WIDTH * HEIGHT) as usize],
        console: &mut dyn Console,
    ) -> Result<(), String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => frontend.show_frame(pixel_buffer, console),
            Frontends::Godot(frontend) => frontend.show_frame(pixel_buffer, console),
            Frontends::None() => Ok(()),
        }
    }
}

pub trait Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: &[u32; (WIDTH * HEIGHT) as usize],
        console: &mut dyn Console,
    ) -> Result<(), String>;
}
