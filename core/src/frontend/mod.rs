pub mod godot_frontend;
#[cfg(feature = "sdl2")]
pub mod sdl_frontend;

use std::cell::Ref;

use crate::emulation::emu::{Console, InputEvent, HEIGHT, WIDTH};
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
        pixel_buffer: Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>,
    ) -> Result<(), String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::Godot(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::None() => Ok(()),
        }
    }

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => frontend.poll_input_events(),
            Frontends::Godot(frontend) => frontend.poll_input_events(),
            Frontends::None() => Ok(Vec::new()),
        }
    }
}

pub trait Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>,
    ) -> Result<(), String>;

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String>;
}
