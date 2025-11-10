pub mod godot_frontend;
#[cfg(feature = "imgui-frontend")]
pub mod imgui_frontend;
#[cfg(feature = "sdl2-frontend")]
pub mod sdl_frontend;

use std::cell::Ref;

use crate::emulation::emu::{InputEvent, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::godot_frontend::GodotFrontend;
#[cfg(feature = "sdl2-frontend")]
use crate::frontend::sdl_frontend::SdlFrontend;

#[derive(Debug)]
pub enum Frontends {
    #[cfg(feature = "sdl2-frontend")]
    Sdl2(SdlFrontend),
    #[cfg(feature = "imgui-frontend")]
    Imgui(),
    Godot(GodotFrontend),
    None(),
}

impl Default for Frontends {
    fn default() -> Self { Frontends::None() }
}

impl Frontend for Frontends {
    #[inline(always)]
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
    ) -> Result<(), String> {
        match self {
            #[cfg(feature = "sdl2-frontend")]
            Frontends::Sdl2(frontend) => frontend.show_frame(pixel_buffer),
            #[cfg(feature = "imgui-frontend")]
            Frontends::Imgui() => Ok(()),
            Frontends::Godot(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::None() => Ok(()),
        }
    }

    #[inline(always)]
    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        match self {
            #[cfg(feature = "sdl2-frontend")]
            Frontends::Sdl2(frontend) => frontend.poll_input_events(),
            #[cfg(feature = "imgui-frontend")]
            Frontends::Imgui() => Ok(Vec::new()),
            Frontends::Godot(frontend) => frontend.poll_input_events(),
            Frontends::None() => Ok(Vec::new()),
        }
    }
}

pub trait Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
    ) -> Result<(), String>;

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String>;
}
