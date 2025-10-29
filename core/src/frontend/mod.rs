pub mod godot_frontend;
#[cfg(feature = "imgui-sdl3-frontend")]
pub mod imgui_sdl3_frontend;
#[cfg(feature = "sdl2")]
pub mod sdl_frontend;

use crate::emulation::emu::InputEvent;
use crate::frontend::godot_frontend::GodotFrontend;
#[cfg(feature = "imgui-sdl3-frontend")]
use crate::frontend::imgui_sdl3_frontend::ImguiSdl3Frontend;
#[cfg(feature = "sdl2")]
use crate::frontend::sdl_frontend::SdlFrontend;

pub enum Frontends {
    #[cfg(feature = "sdl2")]
    Sdl2(SdlFrontend),
    #[cfg(feature = "imgui-sdl3-frontend")]
    ImguiSdl3(ImguiSdl3Frontend),
    Godot(GodotFrontend),
    None,
}

impl Default for Frontends {
    fn default() -> Self { Frontends::None }
}

impl Frontend for Frontends {
    #[inline(always)]
    fn show_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => frontend.show_frame(pixel_buffer),
            #[cfg(feature = "imgui-sdl3-frontend")]
            Frontends::ImguiSdl3(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::Godot(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::None => Ok(()),
        }
    }

    #[inline(always)]
    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        match self {
            #[cfg(feature = "sdl2")]
            Frontends::Sdl2(frontend) => frontend.poll_input_events(),
            #[cfg(feature = "imgui-sdl3-frontend")]
            Frontends::ImguiSdl3(frontend) => frontend.poll_input_events(),
            Frontends::Godot(frontend) => frontend.poll_input_events(),
            Frontends::None => Ok(Vec::new()),
        }
    }
}

pub trait Frontend {
    fn show_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), String>;

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String>;
}
