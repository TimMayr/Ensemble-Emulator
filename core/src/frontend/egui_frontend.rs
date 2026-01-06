/// Main application entry point for the egui frontend.
///
/// This module provides the main `EguiApp` struct and the `run` function
/// to start the frontend. The implementation is split across several
/// submodules for maintainability:
///
/// - `egui::config`: Configuration types
/// - `egui::fps_counter`: FPS tracking
/// - `egui::input`: Input handling
/// - `egui::textures`: Texture management
/// - `egui::ui`: UI rendering components
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use egui::{Context, Style, TextBuffer, Visuals};

use crate::emulation::channel_emu::ChannelEmulator;
use crate::emulation::messages::{EmulatorMessage, FrontendMessage};
use crate::emulation::nes::Nes;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::fps_counter::FpsCounter;
use crate::frontend::egui::input::handle_keyboard_input;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::{add_emulator_views, add_options_panel, add_status_bar};

/// Main egui application state
pub struct EguiApp {
    channel_emu: ChannelEmulator,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
    emu_textures: EmuTextures,
    fps_counter: FpsCounter,
    accumulator: Duration,
    config: AppConfig,
}

impl EguiApp {
    pub fn new(
        channel_emu: ChannelEmulator,
        to_emulator: Sender<FrontendMessage>,
        from_emulator: Receiver<EmulatorMessage>,
    ) -> Self {
        Self {
            channel_emu,
            to_emulator,
            from_emulator,
            emu_textures: Default::default(),
            fps_counter: Default::default(),
            accumulator: Default::default(),
            config: Default::default(),
        }
    }

    /// Calculate the frame budget based on current speed settings
    fn get_frame_budget(&self) -> Duration {
        let speed = self
            .config
            .speed_config
            .app_speed
            .get_fps(&self.config.speed_config);

        if speed == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / speed as u64)
    }

    /// Calculate the debug viewers frame budget based on current speed settings
    fn get_debug_viewers_frame_budget(&self) -> Duration {
        let fps = self
            .config
            .speed_config
            .debug_speed
            .get_fps(&self.config.speed_config);

        if fps == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / fps as u64)
    }

    /// Process messages received from the emulator
    fn process_emu_messages(&mut self, ctx: &Context) {
        while let Ok(msg) = self.from_emulator.try_recv() {
            match msg {
                EmulatorMessage::FrameReady(frame) => {
                    self.emu_textures.current_frame = Some(frame);
                    self.fps_counter.update();
                    self.emu_textures.update_emulator_texture(ctx);
                }
                EmulatorMessage::PatternTableReady(data) => {
                    self.emu_textures.pattern_table_data = Some(data);
                    self.emu_textures.update_pattern_table_texture(ctx);
                }
                EmulatorMessage::NametableReady(data) => {
                    self.emu_textures.nametable_data = Some(data);
                    self.emu_textures.update_nametable_texture(ctx);
                }
                EmulatorMessage::SpritesReady(_data) => {
                    // Sprite viewer not yet implemented
                }
                EmulatorMessage::Stopped => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }

    /// Update emulator textures and run emulation frames
    fn update_emu_textures(&mut self, ctx: &Context) {
        let now = Instant::now();
        if !self.config.speed_config.is_paused {
            let delta = now.duration_since(self.emu_textures.last_frame_request);
            self.accumulator += delta;
            self.emu_textures.last_frame_request = now;

            let frame_budget = self.get_frame_budget();

            // Effectively paused, so we skip
            if frame_budget < Duration::from_secs(5) {
                while self.accumulator >= frame_budget {
                    let start = Instant::now();

                    if let Err(e) = self.channel_emu.step_frame() {
                        eprintln!("Emulator error: {}", e);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    let step_time = start.elapsed();

                    // Machine cannot keep up
                    if step_time > frame_budget {
                        self.accumulator = Duration::ZERO; // drop backlog
                        break;
                    }

                    self.accumulator -= frame_budget;
                }
            }

            self.request_debug_data(now);
        }
    }

    /// Request debug data from the emulator based on timing and visibility
    fn request_debug_data(&mut self, now: Instant) {
        let debug_frame_budget = self.get_debug_viewers_frame_budget();

        // Effectively paused, so we skip
        if debug_frame_budget < Duration::from_secs(5) {
            if self.config.view_config.show_pattern_table
                && now.duration_since(self.emu_textures.last_pattern_table_request)
                    >= debug_frame_budget
            {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::RequestPatternTableData);
                self.emu_textures.last_pattern_table_request = now;
            }

            if self.config.view_config.show_nametable
                && now.duration_since(self.emu_textures.last_nametable_request)
                    >= debug_frame_budget
            {
                let _ = self.to_emulator.send(FrontendMessage::RequestNametableData);
                self.emu_textures.last_nametable_request = now;
            }

            if self.config.view_config.show_sprite_viewer
                && now.duration_since(self.emu_textures.last_sprite_viewer_request)
                    >= debug_frame_budget
            {
                let _ = self.to_emulator.send(FrontendMessage::RequestSpriteData);
                self.emu_textures.last_sprite_viewer_request = now;
            }
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        handle_keyboard_input(
            ctx,
            &self.to_emulator,
            &mut self.config.speed_config,
            &mut self.emu_textures.last_frame_request,
        );

        // Check for escape to quit
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            let _ = self.to_emulator.send(FrontendMessage::Quit);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.update_emu_textures(ctx);

        // Process messages from emulator
        self.process_emu_messages(ctx);

        // Options side panel
        add_options_panel(ctx, &mut self.config);

        // Status bar at bottom
        add_status_bar(
            ctx,
            &self.fps_counter,
            &self.config.speed_config,
            &self.emu_textures,
        );

        // Emulator output windows
        add_emulator_views(ctx, &mut self.config.view_config, &self.emu_textures);

        // Request continuous repaint for animation
        ctx.request_repaint();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.to_emulator.send(FrontendMessage::Quit);
    }
}

impl Debug for EguiApp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("EguiApp") }
}

/// Run the egui frontend
pub fn run(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Create the emulator instance
    let mut console = Nes::default();

    // Load a ROM
    console.load_rom(&file.to_string_lossy().take());
    console.power();

    // Create channel-based emulator wrapper
    let (channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);

    // Configure eframe options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("NES Emulator - Egui"),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "NES Emulator - Egui",
        options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Default::default()
            };
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(EguiApp::new(channel_emu, tx_to_emu, rx_from_emu)))
        }),
    )?;

    Ok(())
}
