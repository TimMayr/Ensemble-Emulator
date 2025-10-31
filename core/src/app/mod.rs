#[cfg(feature = "imgui-frontend")]
use crate::emulation::channel_emu::ChannelEmulator;
#[cfg(feature = "imgui-frontend")]
use crate::emulation::emu::{Console, Consoles};
#[cfg(feature = "imgui-frontend")]
use crate::emulation::nes::Nes;
#[cfg(feature = "imgui-frontend")]
use crate::frontend::imgui_frontend::ImGuiFrontend;

#[cfg(feature = "imgui-frontend")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the emulator instance
    let mut console = Consoles::Nes(Nes::default());
    
    // Load a ROM
    // TODO: Make this configurable via command line or file dialog
    console.load_rom(&String::from("./core/tests/Pac-Man (USA) (Namco).nes"));
    console.power();

    // Create channel-based emulator wrapper
    let (mut channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);

    // Initialize SDL2
    let sdl = sdl2::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl.video().map_err(|e| e.to_string())?;

    // Setup OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    // Create window
    let window = video_subsystem
        .window("NES Emulator - ImGui", 1024, 768)
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    // Create frontend
    let mut frontend = ImGuiFrontend::new(
        &window,
        tx_to_emu,
        rx_from_emu,
    )?;

    // Run the main loop
    frontend.run(&sdl, &window, &mut channel_emu)?;

    Ok(())
}
