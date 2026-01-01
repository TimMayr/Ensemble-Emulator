use std::thread;

use crossbeam_channel::{Receiver, Sender};

use crate::emulation::emu::{Console, Consoles};
use crate::emulation::messages::{ControllerEvent, EmulatorMessage, FrontendMessage};
use crate::emulation::nes::ExecutionFinishedType;
use crate::frontend::Frontends;

/// A threaded wrapper around the emulator that communicates via channels
pub struct ThreadedEmulator {
    handle: Option<thread::JoinHandle<()>>,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
}

impl ThreadedEmulator {
    pub fn new(console: Consoles) -> Self {
        let (tx_to_emu, rx_from_frontend) = crossbeam_channel::unbounded();
        let (tx_from_emu, rx_to_frontend) = crossbeam_channel::unbounded();

        let handle = thread::spawn(move || {
            run_emulator_thread(console, rx_from_frontend, tx_from_emu);
        });

        Self {
            handle: Some(handle),
            to_emulator: tx_to_emu,
            from_emulator: rx_to_frontend,
        }
    }

    pub fn to_emulator(&self) -> Sender<FrontendMessage> { self.to_emulator.clone() }

    pub fn from_emulator(&self) -> Receiver<EmulatorMessage> { self.from_emulator.clone() }

    pub fn join(mut self) -> thread::Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.join()
        } else {
            Ok(())
        }
    }
}

impl Drop for ThreadedEmulator {
    fn drop(&mut self) {
        // Send quit message if thread is still running
        let _ = self.to_emulator.send(FrontendMessage::Quit);

        // Wait for thread to finish
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn run_emulator_thread(
    mut console: Consoles,
    from_frontend: Receiver<FrontendMessage>,
    to_frontend: Sender<EmulatorMessage>,
) {
    // Use a headless frontend for the emulator thread
    let mut frontend = Frontends::None();
    let mut paused = false;
    let mut should_quit = false;

    // Create a channel-based frontend adapter
    let mut channel_frontend = ChannelFrontend::new(from_frontend.clone(), to_frontend.clone());

    loop {
        // Check for messages from frontend
        while let Ok(msg) = from_frontend.try_recv() {
            match msg {
                FrontendMessage::Quit => {
                    should_quit = true;
                }
                FrontendMessage::Pause => {
                    paused = true;
                }
                FrontendMessage::Resume => {
                    paused = false;
                }
                FrontendMessage::Reset => {
                    console.reset();
                }
                FrontendMessage::StepFrame => {
                    let _ = console.step_frame(&mut frontend);
                    // Send frame after stepping
                    let frame = console.get_pixel_buffer();
                    let frame_data = Box::new(*frame);
                    let _ = to_frontend.send(EmulatorMessage::FrameReady(frame_data));
                }
                FrontendMessage::ControllerInput(event) => {
                    channel_frontend.handle_controller_event(event);
                }
            }
        }

        if should_quit {
            break;
        }

        if paused {
            // Sleep a bit when paused to avoid busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(16));
            continue;
        }

        // Step one frame
        match console.step_frame(&mut frontend) {
            Ok(ExecutionFinishedType::CycleCompleted)
            | Ok(ExecutionFinishedType::ReachedLastCycle) => {
                // Frame completed, send it to frontend
                let frame = console.get_pixel_buffer();
                let frame_data = Box::new(*frame);
                if to_frontend
                    .send(EmulatorMessage::FrameReady(frame_data))
                    .is_err()
                {
                    // Frontend disconnected
                    break;
                }
            }
            Ok(ExecutionFinishedType::ReachedHlt) => {
                // Emulator halted
                break;
            }
            Err(e) => {
                eprintln!("Emulator error: {}", e);
                break;
            }
        }

        // Handle any input events from the channel frontend
        if let Ok(events) = channel_frontend.get_pending_events() {
            for event in events {
                // TODO: Handle input events properly
                // For now, just pass them through
                match event {
                    crate::emulation::emu::InputEvent::Quit => {
                        should_quit = true;
                    }
                    crate::emulation::emu::InputEvent::IncPalette => {
                        // Handle palette increment
                        if let Consoles::Nes(ref mut nes) = console {
                            nes.inc_current_palette();
                        }
                    }
                }
            }
        }
    }

    // Send stopped message
    let _ = to_frontend.send(EmulatorMessage::Stopped);
}

/// A frontend adapter that translates between channels and the Frontend trait
struct ChannelFrontend {
    from_frontend: Receiver<FrontendMessage>,
    to_frontend: Sender<EmulatorMessage>,
    pending_events: Vec<crate::emulation::emu::InputEvent>,
}

impl ChannelFrontend {
    fn new(from_frontend: Receiver<FrontendMessage>, to_frontend: Sender<EmulatorMessage>) -> Self {
        Self {
            from_frontend,
            to_frontend,
            pending_events: Vec::new(),
        }
    }

    fn handle_controller_event(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::IncPalette => {
                self.pending_events
                    .push(crate::emulation::emu::InputEvent::IncPalette);
            }
        }
    }

    fn get_pending_events(&mut self) -> Result<Vec<crate::emulation::emu::InputEvent>, String> {
        Ok(std::mem::take(&mut self.pending_events))
    }
}
