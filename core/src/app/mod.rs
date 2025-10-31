#[cfg(feature = "imgui-frontend")]
use sdl3::gpu::*;

#[cfg(feature = "imgui-frontend")]
use crate::emulation::emu::Consoles;
#[cfg(feature = "imgui-frontend")]
use crate::emulation::nes::Nes;
#[cfg(feature = "imgui-frontend")]
use crate::emulation::threaded::ThreadedEmulator;
#[cfg(feature = "imgui-frontend")]
use crate::frontend::imgui_frontend::ImGuiFrontend;

#[cfg(feature = "imgui-frontend")]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the emulator instance
    let mut emu = Consoles::Nes(Nes::default());
    
    // Load a ROM
    // TODO: Make this configurable via command line or file dialog
    emu.load_rom(&String::from("./core/tests/Pac-Man (USA) (Namco).nes"));
    emu.power();

    // Create threaded emulator
    let threaded_emu = ThreadedEmulator::new(emu);

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
        threaded_emu.to_emulator(),
        threaded_emu.from_emulator(),
    )?;

    // Run the frontend (this blocks until the window is closed)
    frontend.run(&mut sdl, &window, &device)?;

    // Wait for emulator thread to finish
    threaded_emu.join().map_err(|e| format!("Thread join error: {:?}", e))?;

    Ok(())
}

