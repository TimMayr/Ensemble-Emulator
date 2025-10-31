use crossbeam_channel::{Receiver, Sender};

use crate::emulation::emu::{Console, Consoles};
use crate::emulation::messages::{ControllerEvent, EmulatorMessage, FrontendMessage};
use crate::emulation::nes::ExecutionFinishedType;
use crate::frontend::Frontends;

/// A non-threaded emulator wrapper that communicates via channels
/// This provides a clean interface for the frontend without threading complications.
/// The emulator runs in the same thread but is decoupled via message passing.
pub struct ChannelEmulator {
    console: Consoles,
    to_frontend: Sender<EmulatorMessage>,
    from_frontend: Receiver<FrontendMessage>,
    frontend: Frontends,
    paused: bool,
}

impl ChannelEmulator {
    pub fn new(console: Consoles) -> (Self, Sender<FrontendMessage>, Receiver<EmulatorMessage>) {
        let (tx_to_emu, rx_from_frontend) = crossbeam_channel::unbounded();
        let (tx_from_emu, rx_to_frontend) = crossbeam_channel::unbounded();

        let emu = Self {
            console,
            to_frontend: tx_from_emu,
            from_frontend: rx_from_frontend,
            frontend: Frontends::None(),
            paused: false,
        };

        (emu, tx_to_emu, rx_to_frontend)
    }

    /// Run one frame of emulation and handle messages
    pub fn step_frame(&mut self) -> Result<(), String> {
        // Check for messages from frontend
        while let Ok(msg) = self.from_frontend.try_recv() {
            match msg {
                FrontendMessage::Quit => {
                    let _ = self.to_frontend.send(EmulatorMessage::Stopped);
                    return Err("Quit requested".to_string());
                }
                FrontendMessage::Pause => {
                    self.paused = true;
                }
                FrontendMessage::Resume => {
                    self.paused = false;
                }
                FrontendMessage::Reset => {
                    self.console.reset();
                }
                FrontendMessage::StepFrame => {
                    // Execute one frame regardless of pause state
                    self.execute_frame()?;
                }
                FrontendMessage::ControllerInput(event) => {
                    self.handle_controller_event(event);
                }
            }
        }

        // If not paused, run a frame
        if !self.paused {
            self.execute_frame()?;
        }

        Ok(())
    }

    fn execute_frame(&mut self) -> Result<(), String> {
        match self.console.step_frame(&mut self.frontend) {
            Ok(ExecutionFinishedType::CycleCompleted) | Ok(ExecutionFinishedType::ReachedLastCycle) => {
                // Frame completed, send it to frontend
                let frame = self.console.get_pixel_buffer();
                let frame_data = Box::new(*frame);
                if self.to_frontend.send(EmulatorMessage::FrameReady(frame_data)).is_err() {
                    // Frontend disconnected
                    return Err("Frontend disconnected".to_string());
                }
                Ok(())
            }
            Ok(ExecutionFinishedType::ReachedHlt) => {
                // Emulator halted
                Err("Emulator halted".to_string())
            }
            Err(e) => {
                Err(format!("Emulator error: {}", e))
            }
        }
    }

    fn handle_controller_event(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::IncPalette => {
                if let Consoles::Nes(ref mut nes) = self.console {
                    nes.inc_current_palette();
                }
            }
        }
    }
}
