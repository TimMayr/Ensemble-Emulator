pub mod godot_frontend;
#[cfg(feature = "sdl2")]
pub mod sdl_frontend;

use std::cell::Ref;

use crate::emulation::emu::{InputEvent, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
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
    #[inline(always)]
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
    ) -> Result<(), String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => match frontend.show_frame(pixel_buffer) {
                Ok(()) => Ok(()),
                Err(err) => {
                    let original_error = err;

                    log::warn!(
                        "SDL2 frontend failed while presenting a frame ({}). Attempting to reinitialize the frontend.",
                        original_error
                    );

                    *frontend = SdlFrontend::default();

                    frontend.show_frame(pixel_buffer).map_err(|retry_err| {
                        format!(
                            "SDL2 frontend failed to recover after reinitialization. First error: {original_error}. Second error: {retry_err}."
                        )
                    })
                }
            },
            Frontends::Godot(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::None() => Ok(()),
        }
    }

    #[inline(always)]
    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => match frontend.poll_input_events() {
                Ok(events) => Ok(events),
                Err(err) => {
                    let original_error = err;

                    log::warn!(
                        "SDL2 frontend failed while polling input ({}). Attempting to reinitialize the frontend.",
                        original_error
                    );

                    *frontend = SdlFrontend::default();

                    frontend.poll_input_events().map_err(|retry_err| {
                        format!(
                            "SDL2 frontend failed to recover after reinitialization. First error: {original_error}. Second error: {retry_err}."
                        )
                    })
                }
            },
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
