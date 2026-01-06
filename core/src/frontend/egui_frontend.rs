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
use crate::emulation::emu::{Console, Consoles};
use crate::emulation::messages::{EmulatorMessage, FrontendMessage};
use crate::emulation::messages::{
    ControllerEvent, EmulatorMessage, FrontendMessage, PaletteData, PatternTableViewerData,
    SpriteData, TileData, NAMETABLE_HEIGHT, NAMETABLE_WIDTH, TOTAL_OUTPUT_HEIGHT,
    TOTAL_OUTPUT_WIDTH,
};
use crate::emulation::nes::Nes;
use crate::emulation::ppu::TILE_SIZE;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::fps_counter::FpsCounter;
use crate::frontend::egui::input::handle_keyboard_input;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::{add_emulator_views, add_options_panel, add_status_bar};
use crate::frontend::Frontends;

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
        let speed = self.config.speed_config.app_speed.get_fps(&self.config.speed_config);

        if speed == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / speed as u64)
    }

    /// Calculate the debug viewers frame budget based on current speed settings
    fn get_debug_viewers_frame_budget(&self) -> Duration {
        let fps = self.config.speed_config.debug_speed.get_fps(&self.config.speed_config);

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

    fn foreground_for_background(bg: u32) -> egui::Color32 {
        let r = (bg >> 16) & 0xFF;
        let g = (bg >> 8) & 0xFF;
        let b = bg & 0xFF;

        let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;

        if luminance > 128.0 {
            egui::Color32::BLACK
        } else {
            egui::Color32::WHITE
        }
    }

    fn add_speed_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("Speed", |ui| {
            ui.label("Emulation Speed")
                .on_hover_text("Sets the speed at which the emulation runs");
            ui.radio_value(
                &mut self.config.speed_config.app_speed,
                AppSpeed::DefaultSpeed,
                "Default (60fps)",
            );
            ui.radio_value(
                &mut self.config.speed_config.app_speed,
                AppSpeed::Custom,
                "Custom",
            );
            ui.radio_value(
                &mut self.config.speed_config.app_speed,
                AppSpeed::Uncapped,
                "Uncapped",
            );

            if self.config.speed_config.app_speed == AppSpeed::Custom {
                ui.add(
                    egui::Slider::new(&mut self.config.speed_config.custom_speed, 0..=500)
                        .text("Speed")
                        .suffix("%")
                        .fixed_decimals(0)
                        .logarithmic(true),
                );
            }
            ui.separator();
            ui.label("Debug Viewer Speed")
                .on_hover_text("Sets the speed at which the debug views update");
            ui.radio_value(
                &mut self.config.speed_config.debug_speed,
                DebugSpeed::Default,
                "10fps",
            );
            ui.radio_value(
                &mut self.config.speed_config.debug_speed,
                DebugSpeed::Custom,
                "Custom",
            );
            ui.radio_value(
                &mut self.config.speed_config.debug_speed,
                DebugSpeed::InStep,
                "Realtime",
            );
            if self.config.speed_config.debug_speed == DebugSpeed::Custom {
                ui.add(
                    egui::Slider::new(&mut self.config.speed_config.debug_custom_speed, 0..=100)
                        .text("Debug Speed")
                        .suffix("%")
                        .fixed_decimals(0)
                        .logarithmic(true),
                )
                .on_hover_text("% of main view fps");
            }
        });
    }

    fn add_view_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("View", |ui| {
            ui.checkbox(
                &mut self.config.view_config.show_pattern_table,
                "Pattern Table Viewer",
            );
            ui.checkbox(
                &mut self.config.view_config.show_nametable,
                "Nametable Viewer",
            );
        });
    }

    fn add_options_panel(&mut self, ctx: &Context) {
        egui::SidePanel::right("options")
            .min_width(100.0)
            .default_width(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.take_available_width();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Settings");

                    self.add_view_settings(ui);
                    self.add_speed_settings(ui)
                });
            });
    }

    fn add_status_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("FPS: {:.1}", self.fps_counter.fps()));
                ui.separator();
                if self.config.speed_config.is_paused {
                    ui.label("Emulator: Paused");
                } else if self.emu_textures.current_frame.is_some() {
                    ui.label("Emulator: Running");
                } else {
                    ui.label("Emulator: Initializing");
                }
            });
        });
    }

    fn draw_pattern_table(&self, ui: &mut Ui, pattern_table: usize) {
        let available = ui.available_width();
        let base_size = 32.0;
        let logical_width = 16.0 * base_size;
        let scale = available / logical_width;
        let tex_size = egui::vec2(base_size, base_size) * scale;
        let pattern_data = &self.emu_textures.pattern_table_data;

        egui::Grid::new("pattern_table")
            .num_columns(16)
            .min_row_height(scale * base_size)
            .min_col_width(scale * base_size)
            .max_col_width(scale * base_size)
            .spacing(egui::vec2(0.0, 0.0))
            .show(ui, |ui| {
                for (i, texture) in self.emu_textures.tile_textures[pattern_table]
                    .iter()
                    .enumerate()
                {
                    let tile_data = if pattern_table == 0 {
                        pattern_data
                            .as_ref()
                            .map(|pattern_data| pattern_data.left.tiles[i])
                    } else {
                        pattern_data
                            .as_ref()
                            .map(|pattern_data| pattern_data.right.tiles[i])
                    };

                    let palette_data = pattern_data
                        .as_ref()
                        .map(|pattern_data| pattern_data.palette);

                    ui.image((texture.id(), tex_size)).on_hover_ui(|ui| {
                        if let Some(tile_data) = tile_data
                            && let Some(palette_data) = palette_data
                        {
                            ui.label(format!("Rom address: ${:0X}", tile_data.address));

                            let plane_0_string = format!("{:064b}", tile_data.plane_0);
                            let plane_1_string = format!("{:064b}", tile_data.plane_1);

                            let mut res = String::new();
                            let mut res_line = String::new();
                            let mut res_vec = Vec::new();

                            let mut char_width = 0.0;
                            let mut line_height = 0.0;
                            for (i, c) in plane_0_string.chars().enumerate() {
                                res_line.push(c);
                                res_line.push(plane_1_string.chars().nth(i).unwrap());

                                if (i + 1) % 8 == 0 {
                                    res += res_line.as_str();
                                    res.push('\n');
                                    res_vec.push(res_line);
                                    res_line = String::new();
                                }
                            }

                            ui.label("Pattern:");

                            let painter = ui.painter();
                            let font_id = egui::FontId::monospace(14.0);

                            let start_pos = ui.cursor().min;
                            let mut y = start_pos.y;
                            for (row, string) in res_vec.iter().enumerate() {
                                let mut x = start_pos.x;
                                let mut current_line_height: f32 = 0.0;

                                for (col, ch) in string.chars().enumerate() {
                                    let bitmap_col = col / 2;
                                    let bit = 63 - (row * 8 + bitmap_col);
                                    let lo = (tile_data.plane_0 >> bit & 1) as u8;
                                    let hi = (tile_data.plane_1 >> bit & 1) as u8;
                                    let color_id = hi << 1 | lo;

                                    let color = palette_data.colors[color_id as usize];

                                    let galley = ui.fonts_mut(|f| {
                                        f.layout_no_wrap(
                                            ch.to_string(),
                                            font_id.clone(),
                                            Self::foreground_for_background(color),
                                        )
                                    });
                                    let rect =
                                        egui::Rect::from_min_size(egui::pos2(x, y), galley.size());

                                    painter.rect_filled(
                                        rect,
                                        0.0,
                                        egui::Color32::from_rgb(
                                            (color >> 16) as u8,
                                            (color >> 8) as u8,
                                            color as u8,
                                        ),
                                    );
                                    painter.galley(rect.min, galley, egui::Color32::WHITE);

                                    x += rect.width();
                                    char_width = rect.width();
                                    current_line_height = current_line_height.max(rect.height());
                                    line_height = current_line_height;
                                }

                                y += current_line_height;
                            }

                            let total_height = res_vec.len() as f32 * line_height;
                            let total_width = res_vec[0].len() as f32 * char_width;
                            ui.allocate_space(egui::vec2(total_width, total_height));
                        }
                    });

                    if (i + 1) % 16 == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn add_emulator_views(&mut self, ctx: &Context) {
        egui::Window::new("Emulator Output")
            .default_size([512.0, 480.0])
            .default_pos([50.0, 50.0])
            .show(ctx, |ui| {
                if let Some(ref texture) = self.emu_textures.emulator_texture {
                    // Get available content region
                    let available = ui.available_size();

                    // Calculate display size (scale to fit window while maintaining aspect ratio)
                    let scale = (available.x / TOTAL_OUTPUT_WIDTH as f32)
                        .min(available.y / TOTAL_OUTPUT_HEIGHT as f32);

                    let display_width = TOTAL_OUTPUT_WIDTH as f32 * scale;
                    let display_height = TOTAL_OUTPUT_HEIGHT as f32 * scale;

                    ui.label(format!(
                        "{}x{} at {:.1}x scale",
                        TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale
                    ));

                    ui.image((texture.id(), egui::vec2(display_width, display_height)));
                } else {
                    ui.label("Waiting for first frame...");
                }
            });

        // Pattern Table Viewer window (optional)
        if self.config.view_config.show_pattern_table {
            egui::Window::new("Pattern Table Viewer")
                .default_size([580.0, 300.0])
                .default_pos([700.0, 50.0])
                .open(&mut true)
                .resizable(true)
                .max_height(0.0)
                .show(ctx, |ui| {
                    if !self.emu_textures.tile_textures.is_empty() {
                        let full_width = ui.available_width();
                        let half_width = (full_width - ui.spacing().item_spacing.x * 3.0) * 0.5;

                        ui.label(format!(
                            "Pattern Tables (128x128x2 at {:.1}x scale)",
                            (half_width / 256.0) - 0.1
                        ));

                        ui.horizontal_top(|ui| {
                            ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                                self.draw_pattern_table(ui, 0);
                            });

                            ui.separator();
                            ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                                self.draw_pattern_table(ui, 1);
                            });
                        });
                    } else {
                        ui.label("Waiting for pattern table data...");
                    }
                });
        }

        // Nametable Viewer window (optional)
        if self.config.view_config.show_nametable {
            egui::Window::new("Nametable Viewer")
                .default_size([600.0, 560.0])
                .default_pos([700.0, 370.0])
                .open(&mut self.config.view_config.show_nametable)
                .show(ctx, |ui| {
                    if let Some(ref texture) = self.emu_textures.nametable_texture {
                        let available = ui.available_size();
                        let scale = (available.x / NAMETABLE_WIDTH as f32)
                            .min(available.y / NAMETABLE_HEIGHT as f32);

                        let display_width = NAMETABLE_WIDTH as f32 * scale;
                        let display_height = NAMETABLE_HEIGHT as f32 * scale;

                        ui.label(format!(
                            "Nametables ({}x{} at {:.1}x)",
                            NAMETABLE_WIDTH, NAMETABLE_HEIGHT, scale
                        ));

                        ui.image((texture.id(), egui::vec2(display_width, display_height)));
                    } else {
                        ui.label("Waiting for nametable data...");
                    }
                });
        }

        if self.config.view_config.show_sprite_viewer {
            egui::Window::new("Sprite Viewer")
                .default_size([600.0, 560.0])
                .default_pos([700.0, 370.0])
                .open(&mut self.config.view_config.show_nametable)
                .show(ctx, |ui| {
                    ui.label("Waiting for Sprite data...");
                });
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("EguiApp")
    }
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
