use crate::emulation::emu::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};

/// Messages sent from the frontend to the emulator
#[derive(Debug, Clone)]
pub enum FrontendMessage {
    /// Request to quit the emulator
    Quit,
    /// Controller input events
    ControllerInput(ControllerEvent),
    /// Request to pause emulation
    Pause,
    /// Request to resume emulation
    Resume,
    /// Request to reset the console
    Reset,
    /// Request to step one frame
    StepFrame,
}

/// Controller input events
#[derive(Debug, Clone, Copy)]
pub enum ControllerEvent {
    // TODO: Add actual NES controller buttons
    // For now, keep the existing events
    IncPalette,
}

/// Messages sent from the emulator to the frontend
#[derive(Debug)]
pub enum EmulatorMessage {
    /// A new frame is ready to be displayed
    FrameReady(Box<[u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>),
    /// Emulator has stopped/quit
    Stopped,
}
