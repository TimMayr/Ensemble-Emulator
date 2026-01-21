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
/// - `egui::tiles`: Tile tree behavior and pane management
/// - `egui::ui`: UI rendering components
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::time::{Duration, Instant};


use crossbeam_channel::{Receiver, Sender};
use egui::{Context, Style, Visuals};

use crate::emulation::channel_emu::ChannelEmulator;
use crate::emulation::messages::{EmulatorFetchable, EmulatorMessage, FrontendMessage, RgbPalette};
use crate::emulation::nes::Nes;
use crate::frontend::egui::config::{AppConfig, AppSpeed};
use crate::frontend::egui::fps_counter::FpsCounter;
use crate::frontend::egui::input::handle_keyboard_input;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::tiles::{
    add_pane_if_missing, compute_required_fetches_from_tree, create_tree, Pane, TreeBehavior,
};
use crate::frontend::egui::ui::add_status_bar;
use crate::frontend::messages::AsyncFrontendMessage;
use crate::frontend::palettes::parse_palette_from_file;
use crate::frontend::util::pick_rom;

/// Main egui application state
pub struct EguiApp {
    channel_emu: ChannelEmulator,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
    from_async: Receiver<AsyncFrontendMessage>,
    async_sender: Sender<AsyncFrontendMessage>,
    emu_textures: EmuTextures,
    fps_counter: FpsCounter,
    accumulator: Duration,
    config: AppConfig,
    /// The tile tree for docking behavior
    tree: egui_tiles::Tree<Pane>,
}

impl EguiApp {
    pub fn new(
        channel_emu: ChannelEmulator,
        to_emulator: Sender<FrontendMessage>,
        from_emulator: Receiver<EmulatorMessage>,
        to_async: Sender<AsyncFrontendMessage>,
        from_async: Receiver<AsyncFrontendMessage>,
        rgb_palette: RgbPalette,
    ) -> Self {
        let mut s = Self {
            channel_emu,
            to_emulator,
            from_emulator,
            from_async,
            async_sender: to_async,
            emu_textures: Default::default(),
            fps_counter: Default::default(),
            accumulator: Default::default(),
            config: Default::default(),
            tree: create_tree(),
        };

        s.config.view_config.palette_rgb_data = rgb_palette;

        s
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
    fn process_messages(&mut self, ctx: &Context) {
        while let Ok(msg) = self.from_async.try_recv() {
            match msg {
                AsyncFrontendMessage::LoadPalette(p) => {
                    let palette = parse_palette_from_file(p.clone());
                    self.config.view_config.palette_rgb_data = palette;
                    if let Some(p) = p {
                        self.config.user_config.previous_palette_path = Some(p)
                    }
                    let _ = self
                        .to_emulator
                        .send(FrontendMessage::SetPalette(Box::new(palette)));
                    self.emu_textures
                        .update_tile_textures(ctx, &self.config.view_config.palette_rgb_data);
                }
                AsyncFrontendMessage::LoadRom(p) => {
                    if let Some(p) = p
                        && let Ok(p) = p.canonicalize()
                    {
                        let _ = self.to_emulator.send(FrontendMessage::PowerOff);
                        let _ = self.to_emulator.send(FrontendMessage::LoadRom(p.clone()));
                        let _ = self.to_emulator.send(FrontendMessage::Power);
                        self.config.user_config.previous_rom_path = Some(p)
                    }
                }
                AsyncFrontendMessage::RefreshPalette => {
                    self.emu_textures
                        .update_tile_textures(ctx, &self.config.view_config.palette_rgb_data);
                }
            }
        }

        while let Ok(msg) = self.from_emulator.try_recv() {
            match msg {
                EmulatorMessage::FrameReady(frame) => {
                    self.emu_textures.current_frame = Some(frame);
                    self.fps_counter.update();
                    self.emu_textures.update_emulator_texture(ctx);
                }
                EmulatorMessage::DebugData(data) => match data {
                    EmulatorFetchable::Palettes(p) => {
                        // Only rebuild textures if palette data actually changed
                        if self.emu_textures.palette_data != p {
                            self.emu_textures.palette_data = p;
                            self.emu_textures.update_tile_textures(
                                ctx,
                                &self.config.view_config.palette_rgb_data,
                            );
                        }
                    }
                    EmulatorFetchable::Tiles(t) => {
                        self.emu_textures.tile_data = t;
                        self.emu_textures
                            .update_tile_textures(ctx, &self.config.view_config.palette_rgb_data);
                    }
                    EmulatorFetchable::Nametables(n) => {
                        self.emu_textures.nametable_data = n;
                    }
                },
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
            let is_uncapped = self.config.speed_config.app_speed == AppSpeed::Uncapped;

            // Maximum time to spend emulating per UI update to keep UI responsive
            // For uncapped mode, allow more time; for normal mode, limit to prevent UI lag
            let max_emulation_time = if is_uncapped {
                Duration::from_millis(70) // Allow up to 70ms of emulation per UI frame
            } else {
                Duration::from_millis(50) // More conservative for normal speeds
            };
            let emulation_start = Instant::now();

            // Effectively paused, so we skip
            if frame_budget < Duration::from_secs(5) {
                while self.accumulator >= frame_budget {
                    if let Err(e) = self.channel_emu.step_frame() {
                        eprintln!("Emulator error: {}", e);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        break;
                    }

                    self.accumulator -= frame_budget;

                    // For uncapped mode, keep running frames until we hit our time budget
                    // For normal modes, check if we've spent too much time and need to drop frames
                    let elapsed = emulation_start.elapsed();
                    if elapsed > max_emulation_time {
                        if !is_uncapped {
                            // Drop backlog only in non-uncapped mode when falling behind
                            self.accumulator = Duration::ZERO;
                        }
                        break;
                    }
                }
            }

            self.request_debug_views(now);
        }
    }

    /// Request debug data from the emulator based on timing and visibility.
    ///
    /// Active fetches (like nametables) are requested on a regular interval.
    /// Passive fetches (like tiles and palettes) are only requested once initially,
    /// then re-requested when the emulator notifies of changes via
    /// `PatternTableChanged` or `PaletteChanged` messages.
    fn request_debug_views(&mut self, now: Instant) {
        let debug_frame_budget = self.get_debug_viewers_frame_budget();

        // Effectively paused, so we skip
        if debug_frame_budget < Duration::from_secs(5)
            && now.duration_since(self.emu_textures.last_debug_request) >= debug_frame_budget
        {
            for to_fetch in &self.config.view_config.required_debug_fetches {
                // Passive fetches (tiles, palettes) are only requested once initially.
                // After that, they're re-requested when the emulator notifies of changes.
                let should_skip_passive = to_fetch.is_passive()
                    && match to_fetch {
                        EmulatorFetchable::Tiles(_) => self.emu_textures.tile_textures.is_some(),
                        EmulatorFetchable::Palettes(_) => self.emu_textures.palette_data.is_some(),
                        _ => false,
                    };

                if !should_skip_passive {
                    let _ = self
                        .to_emulator
                        .send(FrontendMessage::RequestDebugData(to_fetch.clone()));
                }
            }

            self.emu_textures.last_debug_request = Instant::now();
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
            &mut self.config.view_config,
            &mut self.emu_textures.last_frame_request,
        );

        // Check for escape to quit
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            let _ = self.to_emulator.send(FrontendMessage::Quit);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.update_emu_textures(ctx);

        // Process messages from emulator
        self.process_messages(ctx);

        // Update required debug fetches based on visible panes
        self.config.view_config.required_debug_fetches =
            compute_required_fetches_from_tree(&self.tree);

        // Menu bar at the top
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load Rom").clicked() {
                        std::thread::spawn({
                            let sender = self.async_sender.clone();
                            let prev_path = self.config.user_config.previous_rom_path.clone();

                            let prev_dir = if let Some(prev_path) = prev_path {
                                if let Some(prev_path) = prev_path.parent() {
                                    prev_path.to_path_buf()
                                } else {
                                    PathBuf::default()
                                }
                            } else {
                                PathBuf::default()
                            };

                            move || {
                                let path = pick_rom(prev_dir);
                                sender.send(AsyncFrontendMessage::LoadRom(path))
                            }
                        });
                    }
                });
                ui.menu_button("Console", |ui| {
                    if ui.button("Reset").clicked() {
                        let _ = self.to_emulator.send(FrontendMessage::Reset);
                    }
                    if ui.button("Power cycle").clicked() {
                        let _ = self.to_emulator.send(FrontendMessage::PowerOff);
                        if let Some(p) = self.config.user_config.previous_rom_path.clone() {
                            let _ = self.to_emulator.send(FrontendMessage::LoadRom(p));
                        }
                        let _ = self.to_emulator.send(FrontendMessage::Power);
                        self.config.console_config.is_powered = true;
                    }
                    if !self.config.console_config.is_powered {
                        if ui.button("Power On").clicked() {
                            if let Some(p) = self.config.user_config.previous_rom_path.clone() {
                                let _ = self.to_emulator.send(FrontendMessage::LoadRom(p));
                            }
                            let _ = self.to_emulator.send(FrontendMessage::Power);
                            self.config.console_config.is_powered = true;
                        }
                    } else {
                        if ui.button("Power Off").clicked() {
                            let _ = self.to_emulator.send(FrontendMessage::PowerOff);
                            self.config.console_config.is_powered = false;
                        }
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Options").clicked() {
                        add_pane_if_missing(&mut self.tree, Pane::Options);
                        ui.close();
                    }
                    ui.separator();
                    ui.label("Debug Viewers");
                    if ui.button("Palettes").clicked() {
                        add_pane_if_missing(&mut self.tree, Pane::Palettes);
                        ui.close();
                    }
                    if ui.button("Pattern Tables").clicked() {
                        add_pane_if_missing(&mut self.tree, Pane::PatternTables);
                        ui.close();
                    }
                    if ui.button("Nametables").clicked() {
                        add_pane_if_missing(&mut self.tree, Pane::Nametables);
                        ui.close();
                    }
                });
            });
        });

        // Status bar at bottom
        add_status_bar(
            ctx,
            &self.fps_counter,
            &self.config.speed_config,
            &self.emu_textures,
        );

        // Central panel with tile tree
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = TreeBehavior::new(
                &mut self.config,
                &self.emu_textures,
                &self.to_emulator,
                &self.async_sender,
            );
            self.tree.ui(&mut behavior, ui);
        });

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
pub fn run(
    rom: Option<PathBuf>,
    palette: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the emulator instance
    let console = Nes::default();

    let palette = parse_palette_from_file(palette);

    // Create channel-based emulator wrapper
    let (channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);
    let (to_frontend, from_async) = crossbeam_channel::unbounded();

    // Setup Emulator State via messages
    let _ = to_frontend.send(AsyncFrontendMessage::LoadRom(rom));
    let _ = tx_to_emu.send(FrontendMessage::SetPalette(Box::new(palette)));

    // Configure eframe options
    // Disable vsync to allow uncapped frame rates - emulator handles its own timing
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("NES Emulator - Egui")
            .with_app_id("nes-emulator"),
        vsync: false, // Disable vsync for uncapped performance
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
            Ok(Box::new(EguiApp::new(
                channel_emu,
                tx_to_emu,
                rx_from_emu,
                to_frontend,
                from_async,
                palette,
            )))
        }),
    )?;

    Ok(())
}
