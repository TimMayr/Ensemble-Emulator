/// Channel-based emulator wrapper for clean frontend/emulator separation.
///
/// This module provides a non-threaded emulator wrapper that uses channels for
/// communication. While it doesn't provide true multi-threading (due to the
/// emulator core using `Rc<RefCell<>>` which is not `Send`), it provides:
///
/// - Clean separation of concerns between frontend and emulator
/// - Message-based communication protocol
/// - Easy upgrade path to multi-threading once core is refactored
/// - Testable architecture
///
/// # Architecture
///
/// ```text
/// Frontend → FrontendMessage → ChannelEmulator → Emulator Core
///              (channels)              ↓
/// Frontend ← EmulatorMessage ←─────────┘
/// ```
///
/// # Example
///
/// ```ignore
/// use nes_core::emulation::channel_emu::ChannelEmulator;
/// use nes_core::emulation::emu::{Console, Consoles};
/// use nes_core::emulation::nes::Nes;
///
/// let console = Consoles::Nes(Nes::default());
/// let (mut emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);
///
/// // In your main loop:
/// emu.step_frame()?; // Run one frame
/// ```
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
    input: u8,
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
            input: 0,
        };

        (emu, tx_to_emu, rx_to_frontend)
    }

    pub fn set_frontend(&mut self, frontend: Frontends) { self.frontend = frontend; }

    /// Run one frame of emulation and handle messages
    pub fn step_frame(&mut self) -> Result<(), String> {
        // Check for messages from frontend
        while let Ok(msg) = self.from_frontend.try_recv() {
            match msg {
                FrontendMessage::Quit => {
                    let _ = self.to_frontend.send(EmulatorMessage::Stopped);
                    return Err("Quit requested".to_string());
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
                FrontendMessage::EnablePatternTableRendering(enabled) => {
                    if let Consoles::Nes(ref mut nes) = self.console {
                        nes.ppu.borrow_mut().set_render_pattern_tables(enabled);
                    }
                }
                FrontendMessage::EnableNametableRendering(enabled) => {
                    if let Consoles::Nes(ref mut nes) = self.console {
                        nes.ppu.borrow_mut().set_render_nametables(enabled);
                    }
                }
                FrontendMessage::RequestPatternTableData => {
                    // Render pattern tables on demand and send
                    match self.console {
                        Consoles::Nes(ref nes) => {
                            let ppu = nes.ppu.borrow_mut();
                            let data = ppu.get_pattern_table_data_debug();
                            let _ = self
                                .to_frontend
                                .send(EmulatorMessage::PatternTableReady(data));
                        }
                    }
                }
                FrontendMessage::RequestNametableData => {
                    // Render nametables on demand and send
                    if let Consoles::Nes(ref mut nes) = self.console {
                        let mut ppu = nes.ppu.borrow_mut();
                        ppu.render_nametables();
                        let nametable_data = Box::new(*ppu.get_nametable_buffer());
                        let _ = self
                            .to_frontend
                            .send(EmulatorMessage::NametableReady(nametable_data));
                    }
                }
                FrontendMessage::RequestSpriteData => {
                    // if let Consoles::Nes(ref mut nes)  = self.console{
                    //     let ppu = nes.ppu.borrow_mut();
                    //     let _ = self.to_frontend.send(EmulatorMessage::SpritesReady(ppu.render_sprites_debug()));
                    // }
                }
            }
        }

        if let Consoles::Nes(ref mut nes) = self.console {
            nes.cpu.memory.init(0x4016, self.input);
        }
        self.execute_frame()?;
        self.input = 0;

        Ok(())
    }

    fn execute_frame(&mut self) -> Result<(), String> {
        match self.console.step_frame(&mut self.frontend) {
            Ok(
                ExecutionFinishedType::CycleCompleted
                | ExecutionFinishedType::ReachedLastCycle
                | ExecutionFinishedType::ReachedHlt,
            ) => {
                // Frame completed, send it to frontend
                let frame = self.console.get_pixel_buffer();
                let frame_data = Box::new(*frame);
                if self
                    .to_frontend
                    .send(EmulatorMessage::FrameReady(frame_data))
                    .is_err()
                {
                    // Frontend disconnected
                    return Err("Frontend disconnected".to_string());
                }

                // Debug views are now sent only on explicit request via RequestPatternTableData/RequestNametableData
                // This eliminates the overhead of rendering and sending large buffers every frame

                Ok(())
            }
            Err(e) => Err(format!("Emulator error: {}", e)),
        }
    }

    fn handle_controller_event(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::IncPalette => {
                // Since we only have NES consoles for now, we can safely unwrap
                let Consoles::Nes(ref mut nes) = self.console;
                nes.inc_current_palette();
            }
            ControllerEvent::Left => self.input |= 64,
            ControllerEvent::Right => self.input |= 128,
            ControllerEvent::Up => self.input |= 16,
            ControllerEvent::Down => self.input |= 32,
            ControllerEvent::Start => self.input |= 8,
            ControllerEvent::Select => self.input |= 4,
            ControllerEvent::A => self.input |= 1,
            ControllerEvent::B => self.input |= 2,
        }
    }
}
