pub mod frontends;

#[cfg(feature = "frontend")]
pub mod imgui_frontend;

use std::sync::{Arc, Mutex};
use std::time::Instant;

use crossbeam_channel::unbounded;

use crate::app::frontends::{Frontend, Frontends};
#[cfg(feature = "frontend")]
use crate::app::imgui_frontend::ImguiFrontend;
use crate::emulation::emu::{Console, PATTERN_TABLE_HEIGHT, PATTERN_TABLE_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::emulation::nes::{EmuExecutionFinishedType, Nes};

#[cfg(feature = "frontend")]
pub struct App {
    frontend: Frontends,
    pub emulator: Arc<Mutex<Nes>>,
    pub state: Arc<Mutex<AppState>>,
}

#[cfg(not(feature = "frontend"))]
pub struct App {
    frontend: Frontends,
    pub emulator: Arc<Mutex<Nes>>,
    #[cfg(feature = "frontend")]
    pub state: Arc<Mutex<AppState>>,
}

#[cfg(not(feature = "frontend"))]
impl Default for App {
    fn default() -> Self {
        let (app_sender, emu_receiver) = unbounded::<AppToEmuMessages>();
        let (emu_sender, _) = unbounded::<EmuToAppMessages>();

        let emu = Arc::new(Mutex::new(Nes::with_communication(
            emu_sender,
            emu_receiver,
        )));

        let frontend = Frontends::None(app_sender);

        Self {
            frontend,
            emulator: emu,
            #[cfg(feature = "frontend")]
            state: Arc::new(Mutex::new(AppState::default())),
        }
    }
}

#[cfg(feature = "frontend")]
impl Default for App {
    fn default() -> Self {
        let (app_sender, emu_receiver) = unbounded::<AppToEmuMessages>();
        let (emu_sender, app_receiver) = unbounded::<EmuToAppMessages>();

        let emu = Arc::new(Mutex::new(Nes::with_communication(
            emu_sender,
            emu_receiver,
        )));

        let state = Arc::new(Mutex::new(AppState::default()));

        let frontend = Frontends::Imgui(Box::from(ImguiFrontend::new(
            app_sender,
            app_receiver,
            state.clone(),
        )));

        Self {
            frontend,
            emulator: emu,
            state,
        }
    }
}

impl App {
    pub fn new(mut frontend: Frontends, mut emulator: Nes) -> Self {
        let (app_sender, emu_receiver) = unbounded::<AppToEmuMessages>();
        let (emu_sender, app_receiver) = unbounded::<EmuToAppMessages>();

        emulator.set_message_sender(emu_sender);
        emulator.set_message_receiver(emu_receiver);
        frontend.set_message_sender(app_sender);
        frontend.set_message_receiver(app_receiver);

        Self {
            frontend,
            emulator: Arc::new(Mutex::new(emulator)),
            #[cfg(feature = "frontend")]
            state: Arc::new(Mutex::new(AppState::default())),
        }
    }
}

#[cfg(not(feature = "frontend"))]
impl App {
    pub fn run(&mut self) {
        let emu = self.emulator.clone();
        emu.lock().unwrap().power();
        let t = std::thread::spawn(move || {
            loop {
                let mut emu = emu.lock().unwrap();
                let step_result = emu.step(u128::MAX);

                match step_result {
                    Ok(t) => match t {
                        EmuExecutionFinishedType::ReachedLastCycle
                        | EmuExecutionFinishedType::Quit
                        | EmuExecutionFinishedType::CpuHlt => {
                            emu.flush_trace_log();
                            println!("Emu Thread Quit");
                            return;
                        }
                        EmuExecutionFinishedType::Running(_) | EmuExecutionFinishedType::Paused => {
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("Emulator thread error: {e}");
                        break;
                    }
                }
            }
        });

        self.frontend.run();
        t.join().ok();

        println!("App shutdown gracefully")
    }
}

#[cfg(feature = "frontend")]
impl App {
    pub fn run(&mut self) {
        let emu_state = self.state.clone();
        let emu = self.emulator.clone();
        emu.lock().unwrap().power();
        let t = std::thread::spawn(move || {
            loop {
                let mut emu = emu.lock().unwrap();
                let step_result = emu.step(u128::MAX);

                match step_result {
                    Ok(t) => match t {
                        EmuExecutionFinishedType::Running(r) => {
                            if let Some(r) = r
                                && r.frame_ready
                                && r.nametable_changed
                            {
                                let mut shared = emu_state.lock().unwrap();


                                if r.frame_ready {
                                    emu.with_pixel_buffer(|pixels| {
                                        shared.emulator_state.pixel_buffer.copy_from_slice(pixels)
                                    });
                                }

                                if r.nametable_changed {
                                    emu.with_nametable_pixel_buffer(|pixels| {
                                        shared
                                            .emulator_state
                                            .nametable_pixel_buffer
                                            .copy_from_slice(&pixels)
                                    });
                                }
                            }
                        }
                        EmuExecutionFinishedType::ReachedLastCycle
                        | EmuExecutionFinishedType::Quit => {
                            emu.flush_trace_log();
                            println!("Emu Thread Quit");
                            return;
                        }
                        _ => {
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("Emulator thread error: {e}");
                        break;
                    }
                }
            }
        });

        self.frontend.run();
        t.join().ok();

        println!("App shutdown gracefully")
    }
}

#[cfg(feature = "frontend")]
#[derive(Debug, Clone, Default)]
pub struct AppState {
    emulator_state: EmulatorSharedState,
}

#[derive(Debug, Clone)]
pub struct EmulatorSharedState {
    pub pixel_buffer: Box<[u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]>,
    pub frame_ready: bool,
    pub last_frame_time: Instant,
    pub nametable_ready: bool,
    pub pattern_table_ready: bool,
    pub nametable_pixel_buffer: Box<[u32; (SCREEN_WIDTH * 2 * SCREEN_HEIGHT * 2) as usize]>,
    pub pattern_pixel_buffer: Box<[u32; (PATTERN_TABLE_HEIGHT * PATTERN_TABLE_WIDTH) as usize]>,
}

impl Default for EmulatorSharedState {
    fn default() -> Self {
        Self {
            pixel_buffer: Box::new([0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]),
            frame_ready: false,
            last_frame_time: Instant::now(),
            nametable_ready: false,
            pattern_table_ready: false,
            nametable_pixel_buffer: Box::new([0; (SCREEN_WIDTH * 2 * SCREEN_HEIGHT * 2) as usize]),
            pattern_pixel_buffer: Box::new([0; (PATTERN_TABLE_HEIGHT * PATTERN_TABLE_WIDTH) as usize]),
        }
    }
}

pub enum AppToEmuMessages {
    Quit,
    IncPalette,
    Pause,
    Resume,
    Reset,
    Power,
    TogglePause,
    ToggleNametable,
    ShowNametable,
    HideNametable,
    TogglePatternTable,
    ShowPatternTable,
    HidePatternTable,
    LoadRom(String),
}

pub enum EmuToAppMessages {
    Halted,
    Error(String),
    FrameReady,
    NametableChanged,
    PatternTableChanged,
}
