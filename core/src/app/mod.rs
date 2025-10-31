#[cfg(feature = "imgui-frontend")]
use sdl3::gpu::*;

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

    // Initialize SDL
    let mut sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl.video().map_err(|e| e.to_string())?;

    // Create window
    let window = video_subsystem
        .window("NES Emulator - ImGui", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    // Create GPU device
    let device = Device::new(ShaderFormat::SPIRV, true)
        .map_err(|e| format!("Failed to create device: {:?}", e))?
        .with_window(&window)
        .map_err(|e| format!("Failed to associate window: {:?}", e))?;

    // Create frontend
    let mut frontend = ImGuiFrontend::new(
        &device,
        &window,
        tx_to_emu,
        rx_from_emu,
    )?;

    // Run the main loop
    frontend.run(&mut sdl, &window, &device, &mut channel_emu)?;

    Ok(())
}

