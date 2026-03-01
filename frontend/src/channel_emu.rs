use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use crossbeam_channel::{Receiver, Sender};
use monsoon_core::emulation::nes::{ExecutionFinishedType, Nes};
use monsoon_core::emulation::ppu_util::{EmulatorFetchable, PaletteData};
use monsoon_core::util::Hashable;

use crate::messages::{ControllerEvent, EmulatorMessage, FrontendMessage, SaveType};

/// Channel-based emulator wrapper that communicates with the frontend via
/// crossbeam channels.
///
/// On native targets, `ChannelEmulator` is moved to a dedicated thread via
/// [`run()`](ChannelEmulator::run). On WASM, it remains on the main thread
/// and the frontend drives it by calling
/// [`process_messages()`](ChannelEmulator::process_messages) and
/// [`execute_frame()`](ChannelEmulator::execute_frame) directly.
///
/// # Threading model
///
/// The emulator thread is *request-driven*: it executes a frame only when it
/// receives a [`FrontendMessage::StepFrame`] message from the frontend. The
/// one exception is **uncapped mode** — when enabled via
/// [`FrontendMessage::SetUncapped(true)`], the emulator runs frames as fast
/// as possible until the mode is disabled again.
///
/// # Architecture
///
/// ```text
/// Frontend → FrontendMessage → ChannelEmulator → Emulator Core
///              (channels)              ↓
/// Frontend ← EmulatorMessage ←─────────┘
/// ```
pub struct ChannelEmulator {
    pub nes: Nes,
    to_frontend: Sender<EmulatorMessage>,
    from_frontend: Receiver<FrontendMessage>,
    input: u8,
    /// When true, the emulator runs frames continuously without waiting
    /// for `StepFrame` messages.
    uncapped: bool,
    /// Cached palette data for change detection
    last_palette_data: Option<PaletteData>,
    /// Cached hash of pattern table data for efficient change detection
    last_pattern_table_hash: Option<u64>,
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
            uncapped: false,
            last_palette_data: None,
            last_pattern_table_hash: None,
        };

        (emu, tx_to_emu, rx_to_frontend)
    }

    /// Blocking event loop for the emulator thread (native only).
    ///
    /// The loop waits for messages from the frontend. When a `StepFrame`
    /// message arrives, one frame is executed. In uncapped mode, frames
    /// run continuously and messages are drained between frames via
    /// non-blocking `try_recv`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(mut self) {
        loop {
            if self.uncapped {
                // In uncapped mode, drain all pending messages without blocking,
                // then execute a frame immediately.
                match self.process_messages() {
                    Ok(()) => {}
                    Err(_) => break,
                }
                if self.execute_frame().is_err() {
                    break;
                }
            } else {
                // Normal mode: block until a message arrives.
                match self.from_frontend.recv() {
                    Ok(msg) => {
                        if self.handle_message(msg).is_err() {
                            break;
                        }
                    }
                    Err(_) => break, // Channel disconnected
                }
                // Drain any remaining messages that arrived while we were
                // processing the first one.
                match self.process_messages() {
                    Ok(()) => {}
                    Err(_) => break,
                }
            }
        }
    }

    /// Process all pending messages without blocking.
    pub fn process_messages(&mut self) -> Result<(), String> {
        while let Ok(msg) = self.from_frontend.try_recv() {
            self.handle_message(msg)?;
        }
        Ok(())
    }

    /// Handle a single frontend message. Returns `Err` when the emulator
    /// should shut down (quit requested or fatal error).
    fn handle_message(&mut self, msg: FrontendMessage) -> Result<(), String> {
        match msg {
            FrontendMessage::Quit => {
                let state = self.nes.save_state();
                if let Some(state) = state {
                    let _ = self.to_frontend.send(EmulatorMessage::SaveState(
                        Box::new(state),
                        SaveType::Autosave,
                    ));
                }
                let _ = self.to_frontend.send(EmulatorMessage::Stopped);
                return Err("Quit requested".to_string());
            }
            FrontendMessage::Reset => {
                self.nes.reset();
            }
            FrontendMessage::StepFrame => {
                self.execute_frame()?;
            }
            FrontendMessage::ControllerInput(event) => {
                self.handle_controller_event(event);
            }
            FrontendMessage::RequestDebugData(fetchable) => match fetchable {
                EmulatorFetchable::Palettes(_) => {
                    let _ = self
                        .to_frontend
                        .send(EmulatorMessage::DebugData(self.nes.get_palettes_debug()));
                }
                EmulatorFetchable::Tiles(_) => {
                    let _ = self
                        .to_frontend
                        .send(EmulatorMessage::DebugData(self.nes.get_tiles_debug()));
                }
                EmulatorFetchable::Nametables(_) => {
                    let _ = self
                        .to_frontend
                        .send(EmulatorMessage::DebugData(self.nes.get_nametable_debug()));
                }
            },
            FrontendMessage::WritePpu(address, data) => self.nes.ppu_mem_init(address, data),
            FrontendMessage::WriteCpu(address, data) => self.nes.cpu_mem_init(address, data),
            FrontendMessage::LoadRom((data, name)) => {
                let loadable = (&data[..], name);
                self.nes.load_rom(&loadable);
                let _ = self
                    .to_frontend
                    .send(EmulatorMessage::RomLoaded(self.nes.rom_file.clone()));
            }
            FrontendMessage::Power => {
                self.nes.power();
            }
            FrontendMessage::PowerOff => self.nes.power_off(),
            FrontendMessage::CreateSaveState(t) => {
                if self.nes.rom_file.is_some() {
                    let state = self.nes.save_state();
                    if let Some(state) = state {
                        let _ = self
                            .to_frontend
                            .send(EmulatorMessage::SaveState(Box::new(state), t));
                    }
                }
            }
            FrontendMessage::LoadSaveState(s) => self.nes.load_state(*s),
            FrontendMessage::SetUncapped(uncapped) => {
                self.uncapped = uncapped;
            }
        }

        Ok(())
    }

    pub fn execute_frame(&mut self) -> Result<(), String> {
        match self.nes.step_frame() {
            Ok(
                ExecutionFinishedType::CycleCompleted
                | ExecutionFinishedType::ReachedLastCycle
                | ExecutionFinishedType::ReachedHlt
                | ExecutionFinishedType::FrameDone,
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

                // Check if debug data has changed and notify frontend
                self.check_debug_data_changed();

                Ok(())
            }
            Err(e) => Err(format!("Emulator error: {}", e)),
        }
    }

    /// Check if debug data has changed since last check, and send the data if so.
    /// This enables passive fetching of debug data - the frontend only rebuilds
    /// textures when data actually changes, rather than on a regular interval.
    fn check_debug_data_changed(&mut self) {
        // Check palette data (32 bytes, cheap comparison)
        if let EmulatorFetchable::Palettes(Some(current_palette)) = self.nes.get_palettes_debug() {
            let current = *current_palette; // Copy the PaletteData (it's 32 bytes)
            let palette_changed = match &self.last_palette_data {
                Some(last) => *last != current,
                None => true, // First time, consider it changed
            };

            if palette_changed {
                self.last_palette_data = Some(current);
                let _ =
                    self.to_frontend
                        .send(EmulatorMessage::DebugData(EmulatorFetchable::Palettes(
                            Some(Box::new(current)),
                        )));
            }
        }

        // Check tile/pattern table data using a fast hash of raw PPU memory
        // Pattern tables occupy 0x0000-0x1FFF (8KB) in PPU address space
        let pattern_table_memory = self.nes.get_memory_debug(Some(0x0000..=0x1FFF))[1].to_vec();
        let current_hash = &pattern_table_memory.hash();

        let tiles_changed = match self.last_pattern_table_hash {
            Some(last_hash) => last_hash != *current_hash,
            None => true, // First time, consider it changed
        };

        if tiles_changed {
            self.last_pattern_table_hash = Some(*current_hash);
            // Send the actual tile data directly to avoid a round-trip request
            let _ = self
                .to_frontend
                .send(EmulatorMessage::DebugData(self.nes.get_tiles_debug()));
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
        let mut stack: Vec<_> = Vec::with_capacity(enabled.len());

        for x in enabled.iter() {
            stack.push(EmulatorFetchable::get_empty(x));
        }

        while let Some(to_fetch) = stack.pop() {
            // Only process if we haven't seen this fetchable before
            if fetch.insert(EmulatorFetchable::get_empty(&to_fetch)) {
                // If this fetchable has dependencies, add them to the stack for processing
                if let Some(reqs) = deps.get(&to_fetch) {
                    for x in reqs {
                        let empty = EmulatorFetchable::get_empty(x);
                        // Only add to stack if not already in fetch set
                        if !fetch.contains(&empty) {
                            stack.push(empty);
                        }
                    }
                }
            }
        }

        fetch
    }
}
