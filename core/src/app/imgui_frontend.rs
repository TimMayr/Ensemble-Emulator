use std::sync::{Arc, Mutex};

use crossbeam_channel::{Receiver, Sender};
use imgui::TextureId;
use imgui_sdl3::ImGuiSdl3;
use sdl3::event::Event;
use sdl3::gpu::{
    ColorTargetInfo, Device, Filter, LoadOp, SampleCount, SamplerAddressMode, SamplerCreateInfo,
    SamplerMipmapMode, ShaderFormat, StoreOp, Texture, TextureCreateInfo, TextureFormat,
    TextureRegion, TextureSamplerBinding, TextureTransferInfo, TextureUsage, TransferBufferUsage,
};
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::video::{GLProfile, Window};
use sdl3::{EventPump, Sdl};

use crate::app::frontends::Frontend;
use crate::app::{AppState, AppToEmuMessages, EmuToAppMessages};
use crate::emulation::emu::{Consoles, InputEvent, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct FrontendState {
    show_pattern_table: bool,
    show_nametable: bool,
    show_settings: bool,
}

pub struct TextureData<'a> {
    texture: Texture<'a>,
    texture_id: TextureId,
}

pub struct ImguiFrontend<'a> {
    sdl: Sdl,
    window: Window,
    imgui_sdl3: ImGuiSdl3,
    device: Device,
    frontend_state: FrontendState,
    app_state: Arc<Mutex<AppState>>,
    emu: Arc<Mutex<Consoles>>,
    app_sender: Sender<AppToEmuMessages>,
    app_receiver: Receiver<EmuToAppMessages>,
    input_queue: Vec<InputEvent>,
    screen_texture: TextureData<'a>,
    sampler: sdl3::gpu::Sampler,
}

impl ImguiFrontend<'_> {
    pub fn new(
        sender: Sender<AppToEmuMessages>,
        receiver: Receiver<EmuToAppMessages>,
        emu: Arc<Mutex<Consoles>>,
        state: Arc<Mutex<AppState>>,
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

        let info = TextureCreateInfo::new()
            .with_format(TextureFormat::R8g8b8a8Unorm)
            .with_width(SCREEN_WIDTH)
            .with_height(SCREEN_HEIGHT)
            .with_layer_count_or_depth(1)
            .with_num_levels(1)
            .with_sample_count(SampleCount::NoMultiSampling)
            .with_usage(TextureUsage::SAMPLER | TextureUsage::GRAPHICS_STORAGE_READ);

        let screen_texture = device.create_texture(info).unwrap();

        let info = SamplerCreateInfo::new()
            .with_mag_filter(Filter::Nearest)
            .with_min_filter(Filter::Nearest)
            .with_mipmap_mode(SamplerMipmapMode::Nearest)
            .with_address_mode_u(SamplerAddressMode::ClampToEdge)
            .with_address_mode_v(SamplerAddressMode::ClampToEdge)
            .with_address_mode_w(SamplerAddressMode::ClampToEdge);
        let sampler = device.create_sampler(info).unwrap();

        let screen_binding = Box::new(
            TextureSamplerBinding::new()
                .with_texture(&screen_texture)
                .with_sampler(&sampler),
        );

        let screen_texture_id =
            imgui::TextureId::from(screen_binding.as_ref() as *const _ as usize);

        Self {
            sdl,
            window,
            imgui_sdl3,
            device,
            frontend_state: FrontendState {
                show_nametable: false,
                show_pattern_table: false,
                show_settings: false,
            },
            app_state: state,
            emu,
            app_sender: sender,
            app_receiver: receiver,
            input_queue: vec![],
            screen_texture: TextureData {
                texture: screen_texture,
                texture_id: screen_texture_id,
            },
            sampler,
        }
    }
}

impl Frontend for ImguiFrontend<'_> {
    fn run(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let mut running = true;
        while running {
            for msg in self.app_receiver.try_iter() {
                self.handle_message(msg);
            }

            for i in self.get_inputs(&mut event_pump) {
                self.input_queue.push(i);
            }

            while let Some(i) = self.input_queue.pop() {
                if let Err(_) = self.handle_input(i) {
                    running = false;
                }
            }

            let mut command_buffer = self.device.acquire_command_buffer().unwrap();

            let mut app_state = self.app_state.lock().unwrap();
            if app_state.emulator_state.frame_ready {
                app_state.emulator_state.frame_ready = false;

                let byte_size = (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize;
                let mut tb = self
                    .device
                    .create_transfer_buffer()
                    .with_usage(TransferBufferUsage::UPLOAD)
                    .with_size(byte_size as u32)
                    .build()
                    .unwrap();

                let mut map = tb.map::<u8>(&self.device, true);
                let slice = map.mem_mut();
                slice[..byte_size].copy_from_slice(bytemuck::cast_slice(
                    app_state.emulator_state.pixel_buffer.as_slice(),
                ));

                let mut copy_pass = self.device.begin_copy_pass(&command_buffer).unwrap();
                copy_pass.upload_to_gpu_texture(
                    TextureTransferInfo::new()
                        .with_transfer_buffer(&tb)
                        .with_pixels_per_row(SCREEN_WIDTH)
                        .with_rows_per_layer(SCREEN_HEIGHT),
                    TextureRegion::new()
                        .with_texture(&self.screen_texture.texture)
                        .with_height(SCREEN_HEIGHT)
                        .with_width(SCREEN_WIDTH),
                    true,
                );
                self.device.end_copy_pass(copy_pass)
            }

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
                        self.frontend_state.render_ui(
                            ui,
                            &self.emu,
                            &mut self.input_queue,
                            &self.screen_texture,
                        );
                    },
                );

                command_buffer.submit().unwrap();
            } else {
                println!("Swapchain unavailable, cancel work");
                command_buffer.cancel();
            }
        }

        self.quit()
    }

    fn set_message_sender(&mut self, sender: Sender<AppToEmuMessages>) { self.app_sender = sender; }

    fn set_message_receiver(&mut self, receiver: Receiver<EmuToAppMessages>) {
        self.app_receiver = receiver
    }
}

impl ImguiFrontend<'_> {
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
            InputEvent::Quit => {
                self.quit();
                Err("Quit".to_string())
            }
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

    fn quit(&mut self) { self.app_sender.send(AppToEmuMessages::Quit).ok(); }
}

impl FrontendState {
    fn render_ui(
        &mut self,
        ui: &mut imgui::Ui,
        emu: &Arc<Mutex<Consoles>>,
        input_queue: &mut Vec<InputEvent>,
        screen_texture: &TextureData,
    ) {
        // === Menu bar ===
        ui.main_menu_bar(|| {
            ui.menu("View", || {
                ui.menu_item_config("Pattern Table")
                    .selected(self.show_pattern_table)
                    .build();
                ui.menu_item_config("Nametable Viewer")
                    .selected(self.show_nametable)
                    .build();
                ui.menu_item_config("Settings")
                    .selected(self.show_settings)
                    .build();
            });

            if ui.menu_item("Quit") {
                input_queue.push(InputEvent::Quit);
            }
        });

        // === Main emulator window ===
        ui.window("Main Screen")
            .size(
                [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
                imgui::Condition::FirstUseEver,
            )
            .build(|| {
                imgui::Image::new(
                    screen_texture.texture_id,
                    [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
                )
            });

        // === Optional windows ===
        if self.show_pattern_table {
            self.render_pattern_table(ui, emu);
        }
        if self.show_nametable {
            self.render_nametable(ui, emu);
        }
        if self.show_settings {
            self.render_settings(ui, input_queue);
        }
    }

    fn render_pattern_table(&self, ui: &imgui::Ui, emu: &Arc<Mutex<Consoles>>) {
        ui.window("Pattern Table").build(|| {
            ui.text("Pattern table visualizer here");
            // TODO: use emu.lock() and pull pattern table data
        });
    }

    fn render_nametable(&self, ui: &imgui::Ui, emu: &Arc<Mutex<Consoles>>) {
        ui.window("Nametable Viewer").build(|| {
            ui.text("Nametable viewer here");
        });
    }

    fn render_settings(&self, ui: &imgui::Ui, input_queue: &mut Vec<InputEvent>) {
        ui.window("Settings").build(|| {
            if ui.button("Reset Emulator") {
                input_queue.push(InputEvent::Reset);
            }
            ui.same_line();
            if ui.button("Power Cycle") {
                input_queue.push(InputEvent::Power);
            }
        });
    }
}
