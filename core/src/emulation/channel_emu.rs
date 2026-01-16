use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

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
/// let console = Nes::default();
/// let (mut emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);
///
/// // In your main loop:
/// emu.step_frame()?; // Run one frame
/// ```
use crossbeam_channel::{Receiver, Sender};

use crate::emulation::messages::{
    ControllerEvent, EmulatorFetchable, EmulatorMessage, FrontendMessage,
};
use crate::emulation::nes::{ExecutionFinishedType, Nes};

/// A non-threaded emulator wrapper that communicates via channels
/// This provides a clean interface for the frontend without threading complications.
/// The emulator runs in the same thread but is decoupled via message passing.
pub struct ChannelEmulator {
    nes: Nes,
    to_frontend: Sender<EmulatorMessage>,
    from_frontend: Receiver<FrontendMessage>,
    input: u8,
}

pub static FETCH_DEPS: OnceLock<HashMap<EmulatorFetchable, Vec<EmulatorFetchable>>> =
    OnceLock::new();

#[allow(irrefutable_let_patterns)]
impl ChannelEmulator {
    fn setup_fetch_deps() {
        let mut deps = HashMap::new();

        deps.insert(
            EmulatorFetchable::Tiles(None),
            vec![EmulatorFetchable::Palettes(None)],
        );
        deps.insert(
            EmulatorFetchable::Nametables(None),
            vec![EmulatorFetchable::Tiles(None)],
        );

        FETCH_DEPS.get_or_init(|| deps);
    }

    pub fn new(nes: Nes) -> (Self, Sender<FrontendMessage>, Receiver<EmulatorMessage>) {
        Self::setup_fetch_deps();

        let (tx_to_emu, rx_from_frontend) = crossbeam_channel::unbounded();
        let (tx_from_emu, rx_to_frontend) = crossbeam_channel::unbounded();

        let emu = Self {
            nes,
            to_frontend: tx_from_emu,
            from_frontend: rx_from_frontend,
            input: 0,
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
                FrontendMessage::Reset => {
                    self.nes.reset();
                }
                FrontendMessage::StepFrame => {
                    // Execute one frame regardless of pause state
                    self.execute_frame()?;
                }
                FrontendMessage::ControllerInput(event) => {
                    self.handle_controller_event(event);
                }
                FrontendMessage::RequestDebugData(fetchable) => match fetchable {
                    EmulatorFetchable::Palettes(_) => {
                        let _ = self.to_frontend.send(EmulatorMessage::DebugData(
                            self.nes.ppu.borrow().get_palettes_debug(),
                        ));
                    }
                    EmulatorFetchable::Tiles(_) => {
                        let _ = self.to_frontend.send(EmulatorMessage::DebugData(
                            self.nes.ppu.borrow().get_tiles_debug(),
                        ));
                    }
                    EmulatorFetchable::Nametables(_) => {
                        let _ = self.to_frontend.send(EmulatorMessage::DebugData(
                            self.nes.ppu.borrow().get_nametable_debug(),
                        ));
                    }
                },
            }
        }

        self.nes.cpu.memory.init(0x4016, self.input);

        self.execute_frame()?;
        self.input = 0;

        Ok(())
    }

    fn execute_frame(&mut self) -> Result<(), String> {
        match self.nes.step_frame() {
            Ok(
                ExecutionFinishedType::CycleCompleted
                | ExecutionFinishedType::ReachedLastCycle
                | ExecutionFinishedType::ReachedHlt,
            ) => {
                // Frame completed, send it to frontend
                let frame = self.nes.get_pixel_buffer();
                let frame_data = (*frame).to_vec();
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

    pub fn compute_required_fetches(
        enabled: &HashSet<EmulatorFetchable>,
        deps: &HashMap<EmulatorFetchable, Vec<EmulatorFetchable>>,
    ) -> HashSet<EmulatorFetchable> {
        let mut fetch = HashSet::new();
        let mut stack: Vec<_> = Vec::with_capacity(fetch.len());

        for x in enabled.iter() {
            stack.push(EmulatorFetchable::get_empty(x));
        }

        while let Some(to_fetch) = stack.pop() {
            if fetch.insert(EmulatorFetchable::get_empty(&to_fetch))
                && let Some(reqs) = deps.get(&to_fetch)
            {
                for x in reqs {
                    fetch.insert(EmulatorFetchable::get_empty(x));
                }
            }
        }

        fetch
    }
}
