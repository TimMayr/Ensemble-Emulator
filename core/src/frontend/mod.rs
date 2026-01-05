#[cfg(feature = "egui-frontend")]
pub mod egui_frontend;
#[cfg(feature = "sdl2-frontend")]
pub mod sdl_frontend;

#[cfg(feature = "sdl2-frontend")]
use std::cell::Ref;

use crate::emulation::emu::InputEvent;
#[cfg(feature = "sdl2-frontend")]
use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
#[cfg(feature = "sdl2-frontend")]
use crate::frontend::sdl_frontend::SdlFrontend;

#[derive(Debug)]
pub enum Frontends {
    #[cfg(feature = "sdl2-frontend")]
    Sdl2(SdlFrontend),
    #[cfg(feature = "egui-frontend")]
    Egui(),
    None(),
}

impl Default for Frontends {
    fn default() -> Self { Frontends::None() }
}

impl Frontend for Frontends {
    #[cfg(not(feature = "sdl2-frontend"))]
    #[inline(always)]
    fn show_frame(&mut self) -> Result<(), String> {
        match self {
            #[cfg(feature = "egui-frontend")]
            Frontends::Egui() => Ok(()),
            Frontends::None() => Ok(()),
        }
    }

    #[cfg(feature = "sdl2-frontend")]
    #[inline(always)]
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT]>,
    ) -> Result<(), String> {
        match self {
            #[cfg(feature = "sdl2-frontend")]
            Frontends::Sdl2(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::None() => Ok(()),
        }
    }

    #[inline(always)]
    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        match self {
            #[cfg(feature = "sdl2-frontend")]
            Frontends::Sdl2(frontend) => frontend.poll_input_events(),
            #[cfg(feature = "egui-frontend")]
            Frontends::Egui() => Ok(Vec::new()),
            Frontends::None() => Ok(Vec::new()),
        }
    }
}

pub trait Frontend {
    #[cfg(feature = "sdl2-frontend")]
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT]>,
    ) -> Result<(), String>;

    #[cfg(not(feature = "sdl2-frontend"))]
    fn show_frame(&mut self) -> Result<(), String>;

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String>;
}
