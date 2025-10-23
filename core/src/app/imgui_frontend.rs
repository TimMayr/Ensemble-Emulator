use std::sync::{Arc, Mutex};

use crossbeam_channel::{Receiver, Sender};
use imgui_sdl3::ImGuiSdl3;
use sdl3::event::Event;
use sdl3::gpu::{ColorTargetInfo, Device, LoadOp, ShaderFormat, StoreOp};
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::video::{GLProfile, Window};
use sdl3::{EventPump, Sdl};

use crate::app::frontends::Frontend;
use crate::app::{AppToEmuMessages, EmuToAppMessages};
use crate::emulation::emu::{Consoles, InputEvent};

pub struct FrontendState {
    show_pattern_table: bool,
    show_nametable: bool,
}

pub struct ImguiFrontend {
    sdl: Sdl,
    window: Window,
    imgui_sdl3: ImGuiSdl3,
    device: Device,
    state: FrontendState,
    emu: Arc<Mutex<Consoles>>,
    app_sender: Sender<AppToEmuMessages>,
    app_receiver: Receiver<EmuToAppMessages>,
}

impl ImguiFrontend {
    pub fn new(
        sender: Sender<AppToEmuMessages>,
        receiver: Receiver<EmuToAppMessages>,
        emu: Arc<Mutex<Consoles>>,
    ) -> Self {
        // Initialize SDL3
        let sdl = sdl3::init().expect("Error initializing SDL");
        let video = sdl.video().expect("Error initializing video system");

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Create window
        let window = video
            .window("NES Emulator", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .expect("Error creating window");

        let device = Device::new(ShaderFormat::SPIRV, false)
            .unwrap()
            .with_window(&window)
            .unwrap();

        let imgui_sdl3 = ImGuiSdl3::new(&device, &window, |ctx| {
            ctx.set_ini_filename(None);
            ctx.set_log_filename(None);

            ctx.fonts().add_font(&[imgui::FontSource::DefaultFontData {
                config: None,
            }]);
        });

        Self {
            sdl,
            window,
            imgui_sdl3,
            device,
            state: FrontendState {
                show_nametable: false,
                show_pattern_table: false,
            },
            emu,
            app_sender: sender,
            app_receiver: receiver,
        }
    }
}

impl Frontend for ImguiFrontend {
    fn run(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut running = true;
        while running {
            for msg in self.app_receiver.try_iter() {
                self.handle_message(msg);
            }

            for i in self.get_inputs(&mut event_pump) {
                if let Err(_) = self.handle_input(i) {
                    running = false;
                }
            }

            let mut command_buffer = self.device.acquire_command_buffer().unwrap();

            if let Ok(swapchain) = command_buffer.wait_and_acquire_swapchain_texture(&self.window) {
                let color_targets = [ColorTargetInfo::default()
                    .with_texture(&swapchain)
                    .with_load_op(LoadOp::CLEAR)
                    .with_store_op(StoreOp::STORE)
                    .with_clear_color(Color::RGB(128, 128, 128))];

                self.imgui_sdl3.render(
                    &mut self.sdl,
                    &self.device,
                    &self.window,
                    &event_pump,
                    &mut command_buffer,
                    &color_targets,
                    |ui| {
                        // create imgui UI here
                        ui.show_demo_window(&mut true);
                    },
                );

                command_buffer.submit().unwrap();
            } else {
                println!("Swapchain unavailable, cancel work");
                command_buffer.cancel();
            }
        }

        self.quit().unwrap()
    }

    fn set_message_sender(&mut self, sender: Sender<AppToEmuMessages>) { self.app_sender = sender; }

    fn set_message_receiver(&mut self, receiver: Receiver<EmuToAppMessages>) {
        self.app_receiver = receiver
    }
}

impl ImguiFrontend {
    fn get_inputs(&mut self, event_pump: &mut EventPump) -> Vec<InputEvent> {
        let mut events = Vec::new();

        // Handle events
        for event in event_pump.poll_iter() {
            self.imgui_sdl3.handle_event(&event);

            match event {
                Event::Quit {
                    ..
                } => events.push(InputEvent::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    events.push(InputEvent::IncPalette);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    ..
                } => {
                    events.push(InputEvent::TogglePause);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Comma),
                    ..
                } => {
                    events.push(InputEvent::Pause);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Slash),
                    ..
                } => {
                    events.push(InputEvent::Resume);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    events.push(InputEvent::Power);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    events.push(InputEvent::Reset);
                }
                _ => {}
            }
        }

        events
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<(), String> {
        match event {
            InputEvent::Quit => self.quit(),
            InputEvent::IncPalette => {
                self.app_sender.send(AppToEmuMessages::IncPalette).ok();
                Ok(())
            }
            InputEvent::TogglePause => {
                self.app_sender.send(AppToEmuMessages::TogglePause).ok();
                Ok(())
            }
            InputEvent::Pause => {
                self.app_sender.send(AppToEmuMessages::Pause).ok();
                Ok(())
            }
            InputEvent::Resume => {
                self.app_sender.send(AppToEmuMessages::Resume).ok();
                Ok(())
            }
            InputEvent::Power => {
                self.app_sender.send(AppToEmuMessages::Power).ok();
                Ok(())
            }
            InputEvent::Reset => {
                self.app_sender.send(AppToEmuMessages::Reset).ok();
                Ok(())
            }
            InputEvent::LoadRom(path) => {
                self.app_sender.send(AppToEmuMessages::LoadRom(path)).ok();
                Ok(())
            }
        }
    }

    fn handle_message(&self, msg: EmuToAppMessages) {
        match msg {
            EmuToAppMessages::Halted => {
                eprintln!("Emulator is halted")
            }
            EmuToAppMessages::Error(e) => {
                eprintln!("{e}")
            }
        }
    }

    fn quit(&mut self) -> Result<(), String> {
        self.app_sender.send(AppToEmuMessages::Quit).ok();
        Err("quit".to_string())
    }
}
