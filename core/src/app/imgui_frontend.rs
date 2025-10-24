use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender};
use imgui::TextureId;
use imgui_sdl3::ImGuiSdl3;
use sdl3::event::Event;
use sdl3::gpu::{
    ColorTargetInfo, Device, Filter, LoadOp, SampleCount, SamplerAddressMode, SamplerCreateInfo,
    SamplerMipmapMode, ShaderFormat, StoreOp, Texture, TextureCreateInfo, TextureFormat,
    TextureRegion, TextureSamplerBinding, TextureTransferInfo, TextureUsage, TransferBuffer,
    TransferBufferUsage,
};
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::video::{GLProfile, Window};
use sdl3::{EventPump, Sdl};

use crate::app::frontends::Frontend;
use crate::app::{AppState, AppToEmuMessages, EmuToAppMessages};
use crate::emulation::emu::{InputEvent, SCREEN_HEIGHT, SCREEN_WIDTH};

const FRAME_BUFFER_BYTES: usize = (SCREEN_WIDTH as usize) * (SCREEN_HEIGHT as usize) * 4;

pub struct FrontendState {
    show_pattern_table: bool,
    show_nametable: bool,
    show_settings: bool,
}

pub struct TextureData {
    texture: Texture<'static>,
    _sampler: sdl3::gpu::Sampler,
    _binding: Box<TextureSamplerBinding>,
    texture_id: TextureId,
}

pub struct ImguiFrontend {
    sdl: Sdl,
    window: Window,
    imgui_sdl3: ImGuiSdl3,
    device: Device,
    frontend_state: FrontendState,
    app_state: Arc<Mutex<AppState>>,
    app_sender: Sender<AppToEmuMessages>,
    app_receiver: Receiver<EmuToAppMessages>,
    input_queue: VecDeque<InputEvent>,
    screen_texture: TextureData,
    transfer_buffer: TransferBuffer,
}

impl TextureData {
    fn new(device: &Device) -> Self {
        let texture_info = TextureCreateInfo::new()
            .with_format(TextureFormat::R8g8b8a8Unorm)
            .with_width(SCREEN_WIDTH)
            .with_height(SCREEN_HEIGHT)
            .with_layer_count_or_depth(1)
            .with_num_levels(1)
            .with_sample_count(SampleCount::NoMultiSampling)
            .with_usage(TextureUsage::SAMPLER | TextureUsage::GRAPHICS_STORAGE_READ);

        let texture = device.create_texture(texture_info).unwrap();

        let sampler_info = SamplerCreateInfo::new()
            .with_mag_filter(Filter::Nearest)
            .with_min_filter(Filter::Nearest)
            .with_mipmap_mode(SamplerMipmapMode::Nearest)
            .with_address_mode_u(SamplerAddressMode::ClampToEdge)
            .with_address_mode_v(SamplerAddressMode::ClampToEdge)
            .with_address_mode_w(SamplerAddressMode::ClampToEdge);

        let sampler = device.create_sampler(sampler_info).unwrap();

        let binding = Box::new(
            TextureSamplerBinding::new()
                .with_texture(&texture)
                .with_sampler(&sampler),
        );

        let binding_ptr: *const TextureSamplerBinding = &*binding;
        let texture_id = TextureId::from(binding_ptr);

        Self {
            texture,
            _sampler: sampler,
            _binding: binding,
            texture_id,
        }
    }

    fn texture(&self) -> &Texture<'static> { &self.texture }

    fn id(&self) -> TextureId { self.texture_id }
}

impl ImguiFrontend {
    pub fn new(
        sender: Sender<AppToEmuMessages>,
        receiver: Receiver<EmuToAppMessages>,
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

        let screen_texture = TextureData::new(&device);

        let transfer_buffer = device
            .create_transfer_buffer()
            .with_usage(TransferBufferUsage::UPLOAD)
            .with_size(FRAME_BUFFER_BYTES as u32)
            .build()
            .unwrap();

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
            app_sender: sender,
            app_receiver: receiver,
            input_queue: VecDeque::new(),
            screen_texture,
            transfer_buffer,
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

            self.collect_inputs(&mut event_pump);

            while let Some(i) = self.input_queue.pop_front() {
                if self.handle_input(i).is_err() {
                    running = false;
                }
            }

            let mut command_buffer = self.device.acquire_command_buffer().unwrap();

            {
                let mut app_state = self.app_state.lock().unwrap();
                if app_state.emulator_state.frame_ready {
                    app_state.emulator_state.frame_ready = false;

                    let mut map = self.transfer_buffer.map::<u8>(&self.device, true);
                    let slice = map.mem_mut();

                    slice[..FRAME_BUFFER_BYTES].copy_from_slice(bytemuck::cast_slice(
                        app_state.emulator_state.pixel_buffer.as_slice(),
                    ));
                    map.unmap();

                    let copy_pass = self.device.begin_copy_pass(&command_buffer).unwrap();
                    copy_pass.upload_to_gpu_texture(
                        TextureTransferInfo::new()
                            .with_transfer_buffer(&self.transfer_buffer)
                            .with_pixels_per_row(SCREEN_WIDTH)
                            .with_rows_per_layer(SCREEN_HEIGHT),
                        TextureRegion::new()
                            .with_texture(self.screen_texture.texture())
                            .with_height(SCREEN_HEIGHT)
                            .with_width(SCREEN_WIDTH),
                        true,
                    );
                    self.device.end_copy_pass(copy_pass);
                }
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
                            &mut self.input_queue,
                            &self.screen_texture,
                            self.app_state.lock().unwrap(),
                        );
                    },
                );

                command_buffer.submit().unwrap();
            } else {
                println!("Swapchain unavailable, cancel work");
                command_buffer.cancel();
            }

            if running {
                while let Some(i) = self.input_queue.pop_front() {
                    if self.handle_input(i).is_err() {
                        running = false;
                        break;
                    }
                }
            }
        }

        self.quit()
    }

    fn set_message_sender(&mut self, sender: Sender<AppToEmuMessages>) { self.app_sender = sender; }

    fn set_message_receiver(&mut self, receiver: Receiver<EmuToAppMessages>) {
        self.app_receiver = receiver
    }
}

impl ImguiFrontend {
    fn collect_inputs(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            self.imgui_sdl3.handle_event(&event);

            match event {
                Event::Quit {
                    ..
                } => self.input_queue.push_back(InputEvent::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => self.input_queue.push_back(InputEvent::IncPalette),
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    ..
                } => self.input_queue.push_back(InputEvent::TogglePause),
                Event::KeyDown {
                    keycode: Some(Keycode::Comma),
                    ..
                } => self.input_queue.push_back(InputEvent::Pause),
                Event::KeyDown {
                    keycode: Some(Keycode::Slash),
                    ..
                } => self.input_queue.push_back(InputEvent::Resume),
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => self.input_queue.push_back(InputEvent::Power),
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => self.input_queue.push_back(InputEvent::Reset),
                _ => {}
            }
        }
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<(), String> {
        match event {
            InputEvent::Quit => Err("Quit".to_string()),
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
            EmuToAppMessages::FrameReady => {
                self.app_state.lock().unwrap().emulator_state.frame_ready = true;
                self.app_state
                    .lock()
                    .unwrap()
                    .emulator_state
                    .last_frame_time = Instant::now();
            }
        }
    }

    fn quit(&mut self) { self.app_sender.send(AppToEmuMessages::Quit).ok(); }
}

impl FrontendState {
    fn render_ui(
        &mut self,
        ui: &mut imgui::Ui,
        input_queue: &mut VecDeque<InputEvent>,
        screen_texture: &TextureData,
        app_state: MutexGuard<AppState>,
    ) {
        // === Menu bar ===
        ui.main_menu_bar(|| {
            ui.menu("View", || {
                if ui
                    .menu_item_config("Pattern Table")
                    .selected(self.show_pattern_table)
                    .build()
                {
                    self.show_pattern_table = !self.show_pattern_table;
                }
                if ui
                    .menu_item_config("Nametable Viewer")
                    .selected(self.show_nametable)
                    .build()
                {
                    self.show_nametable = !self.show_nametable;
                }
                if ui
                    .menu_item_config("Settings")
                    .selected(self.show_settings)
                    .build()
                {
                    self.show_settings = !self.show_settings;
                }
            });

            if ui.menu_item("Quit") {
                input_queue.push_back(InputEvent::Quit);
            }
        });

        // === Main emulator window ===
        ui.window("Main Screen")
            .size(
                [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
                imgui::Condition::FirstUseEver,
            )
            .build(|| {
                ui.text(format!(
                    "Frame Time: {:?}",
                    app_state.emulator_state.last_frame_time.elapsed()
                ));
                imgui::Image::new(
                    screen_texture.id(),
                    [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32],
                )
                .build(ui)
            });

        // === Optional windows ===
        if self.show_pattern_table {
            self.render_pattern_table(ui);
        }
        if self.show_nametable {
            self.render_nametable(ui);
        }
        if self.show_settings {
            self.render_settings(ui, input_queue);
        }
    }

    fn render_pattern_table(&self, ui: &imgui::Ui) {
        ui.window("Pattern Table").build(|| {
            ui.text("Pattern table visualizer here");
        });
    }

    fn render_nametable(&self, ui: &imgui::Ui) {
        ui.window("Nametable Viewer").build(|| {
            ui.text("Nametable viewer here");
        });
    }

    fn render_settings(&self, ui: &imgui::Ui, input_queue: &mut VecDeque<InputEvent>) {
        ui.window("Settings").build(|| {
            if ui.button("Reset Emulator") {
                input_queue.push_back(InputEvent::Reset);
            }
            ui.same_line();
            if ui.button("Power Cycle") {
                input_queue.push_back(InputEvent::Power);
            }
        });
    }
}
