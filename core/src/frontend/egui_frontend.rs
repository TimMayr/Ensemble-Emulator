use std::cmp::max;
use std::collections::HashMap;
/// Egui-based frontend for the NES emulator using eframe.
///
/// This frontend provides a modern, multi-window debugging interface with:
/// - Real-time emulator output display with proper texture rendering
/// - Pattern table viewer
/// - Nametable viewer
/// - FPS counter and status display
/// - Extensible window system for future debug features
///
/// # Architecture
///
/// The frontend uses message-based communication with the emulator:
/// - Sends `FrontendMessage` for commands (pause, resume, quit, etc.)
/// - Receives `EmulatorMessage` for updates (frame ready, stopped, etc.)
///
/// # Threading
///
/// Currently runs in a single thread with the emulator for simplicity.
/// The architecture supports future multi-threading once the emulator
/// core is refactored to use `Arc<Mutex<>>` instead of `Rc<RefCell<>>`.
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use egui::{ColorImage, Context, Style, TextBuffer, TextureHandle, TextureOptions, Ui, Visuals};

use crate::emulation::channel_emu::ChannelEmulator;
use crate::emulation::emu::{Console, Consoles};
use crate::emulation::messages::{
    ControllerEvent, EmulatorMessage, FrontendMessage, NAMETABLE_HEIGHT, NAMETABLE_WIDTH,
    PaletteData, PatternTableViewerData, SpriteData, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH,
    TileData,
};
use crate::emulation::nes::Nes;
use crate::emulation::ppu::TILE_SIZE;
use crate::frontend::Frontends;

/// FPS counter that tracks frame times over the last second
#[derive(PartialEq, Clone)]
struct FpsCounter {
    frame_times: Vec<Instant>,
    last_update: Instant,
    current_fps: f32,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            frame_times: Vec::new(),
            last_update: Instant::now(),
            current_fps: 0.0,
        }
    }
}

impl FpsCounter {
    fn update(&mut self) {
        let now = Instant::now();
        self.frame_times.push(now);

        // Keep only frames from the last second
        self.frame_times
            .retain(|&t| now.duration_since(t) < Duration::from_secs(1));

        // Update FPS counter every 0.5 seconds
        if now.duration_since(self.last_update) >= Duration::from_millis(500) {
            self.current_fps = self.frame_times.len() as f32;
            self.last_update = now;
        }
    }

    fn fps(&self) -> f32 { self.current_fps }
}

#[derive(Eq, PartialEq, Clone)]
pub struct EmuTextures {
    current_frame: Option<Vec<u32>>,
    emulator_texture: Option<TextureHandle>,
    pattern_table_data: Option<Box<PatternTableViewerData>>,
    nametable_data: Option<Vec<u32>>,
    nametable_texture: Option<TextureHandle>,
    sprite_viewer_data: Option<Vec<SpriteData>>,
    sprite_viewer_textures: Option<HashMap<u8, TextureHandle>>,
    last_pattern_table_request: Instant,
    last_nametable_request: Instant,
    last_frame_request: Instant,
    last_sprite_viewer_request: Instant,
    sprite_height: usize,
    tile_textures: Vec<Vec<TextureHandle>>,
}

impl Default for EmuTextures {
    fn default() -> Self {
        Self {
            current_frame: Default::default(),
            emulator_texture: Default::default(),
            pattern_table_data: Default::default(),
            nametable_data: Default::default(),
            nametable_texture: Default::default(),
            sprite_viewer_data: Default::default(),
            sprite_viewer_textures: Default::default(),
            last_pattern_table_request: Instant::now(),
            last_nametable_request: Instant::now(),
            last_frame_request: Instant::now(),
            last_sprite_viewer_request: Instant::now(),
            sprite_height: 8,
            tile_textures: vec![vec![], vec![]],
        }
    }
}

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

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct ViewConfig {
    show_nametable: bool,
    show_pattern_table: bool,
    show_sprite_viewer: bool,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct AppConfig {
    view_config: ViewConfig,
    speed_config: SpeedConfig,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum AppSpeed {
    #[default]
    DefaultSpeed,
    Uncapped,
    Custom,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct SpeedConfig {
    app_speed: AppSpeed,
    debug_speed: DebugSpeed,
    is_paused: bool,
    custom_speed: u16,
    debug_custom_speed: u16,
}

impl Default for SpeedConfig {
    fn default() -> Self {
        Self {
            app_speed: Default::default(),
            debug_speed: Default::default(),
            is_paused: false,
            custom_speed: 100,
            debug_custom_speed: 10,
        }
    }
}

impl AppSpeed {
    pub fn get_fps(&self, app: &EguiApp) -> u16 {
        match self {
            AppSpeed::DefaultSpeed => 60,
            AppSpeed::Uncapped => u16::MAX,
            AppSpeed::Custom => {
                (60.0 * (app.config.speed_config.custom_speed as f32 / 100.0)) as u16
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum DebugSpeed {
    #[default]
    Default,
    InStep,
    Custom,
}

impl DebugSpeed {
    pub fn get_fps(&self, app: &EguiApp) -> u16 {
        match self {
            DebugSpeed::Default => 10,
            DebugSpeed::InStep => app.config.speed_config.app_speed.get_fps(app),
            DebugSpeed::Custom => {
                if app.config.speed_config.debug_custom_speed == 0 {
                    return 0;
                }

                if app.config.speed_config.app_speed == AppSpeed::Uncapped {
                    return 10;
                }

                max(
                    ((app.config.speed_config.debug_custom_speed as f64 / 100.0)
                        * app.config.speed_config.app_speed.get_fps(app) as f64)
                        as u16,
                    1,
                )
            }
        }
    }
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

    /// Convert u32 ARGB pixel data to egui ColorImage (RGBA)
    fn u32_to_color_image(data: &[u32], width: usize, height: usize) -> ColorImage {
        let mut pixels = Vec::with_capacity(width * height);
        for &pixel in data {
            // Input is ARGB (0xAARRGGBB), we need RGBA for egui
            let a = ((pixel >> 24) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            pixels.push(egui::Color32::from_rgba_unmultiplied(r, g, b, a));
        }
        ColorImage {
            size: [width, height],
            source_size: Default::default(),
            pixels,
        }
    }

    fn update_emulator_texture(&mut self, ctx: &Context) {
        if let Some(ref frame) = self.emu_textures.current_frame {
            let image =
                Self::u32_to_color_image(frame.as_ref(), TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT);

            let texture = ctx.load_texture(
                "emulator_output",
                image,
                TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                },
            );
            self.emu_textures.emulator_texture = Some(texture);
        }
    }

    fn get_texture_for_tile(
        tile: &TileData,
        palette: &PaletteData,
        ctx: &Context,
    ) -> TextureHandle {
        let mut data = [0u32; 64];

        for (i, color) in data.iter_mut().enumerate() {
            let bit = 63 - i;

            let lo = ((tile.plane_0 >> bit) & 1) as usize;
            let hi = ((tile.plane_1 >> bit) & 1) as usize;

            let color_index = lo | (hi << 1);
            *color = palette.colors[color_index];
        }

        let image = Self::u32_to_color_image(data.as_ref(), TILE_SIZE, TILE_SIZE);

        ctx.load_texture(
            format!("tile_{:16X}", tile.address),
            image,
            TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Nearest,
                ..Default::default()
            },
        )
    }

    fn update_pattern_table_texture(&mut self, ctx: &Context) {
        if let Some(ref data) = self.emu_textures.pattern_table_data {
            let palette = data.palette;

            self.emu_textures.tile_textures[0].clear();
            self.emu_textures.tile_textures[1].clear();
            for x in data.left.tiles {
                let texture = Self::get_texture_for_tile(&x, &palette, ctx);
                self.emu_textures.tile_textures[0].push(texture);
            }

            for x in data.right.tiles {
                let texture = Self::get_texture_for_tile(&x, &palette, ctx);
                self.emu_textures.tile_textures[1].push(texture);
            }
        }
    }

    fn update_nametable_texture(&mut self, ctx: &Context) {
        if let Some(ref data) = self.emu_textures.nametable_data {
            let image = Self::u32_to_color_image(data.as_ref(), NAMETABLE_WIDTH, NAMETABLE_HEIGHT);

            let texture = ctx.load_texture(
                "nametable",
                image,
                TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                },
            );
            self.emu_textures.nametable_texture = Some(texture);
        }
    }

    // fn update_sprite_viewer_textures(&mut self, ctx: &Context) {
    //     let mut textures = HashMap::with_capacity(64);
    //     if let Some(ref sprites) = self.emu_textures.sprite_viewer_data {
    //         for (i, sprite) in sprites.iter().enumerate() {
    //             let image = Self::u32_to_color_image(
    //                 sprite.as_ref(),
    //                 SPRITE_WIDTH,
    //                 self.emu_textures.sprite_height,
    //             );
    //
    //             let texture = ctx.load_texture(
    //                 format!("sprite{}", i),
    //                 image,
    //                 TextureOptions {
    //                     magnification: egui::TextureFilter::Nearest,
    //                     minification: egui::TextureFilter::Nearest,
    //                     ..Default::default()
    //                 },
    //             );
    //
    //             textures.insert(i as u8, texture);
    //         }
    //     }
    //
    //     self.emu_textures.sprite_viewer_textures = Some(textures);
    // }

    fn handle_keyboard_input(&mut self, ctx: &Context) {
        ctx.input(|i| {
            // Emulator controls
            if i.key_pressed(egui::Key::N) {
                let _ = self.to_emulator.send(FrontendMessage::ControllerInput(
                    ControllerEvent::IncPalette,
                ));
            }
            if i.key_pressed(egui::Key::Period) {
                self.config.speed_config.is_paused = !self.config.speed_config.is_paused;
                self.emu_textures.last_frame_request = Instant::now();
            }
            if i.key_pressed(egui::Key::R) {
                let _ = self.to_emulator.send(FrontendMessage::Reset);
            }

            // Controller input
            if i.key_pressed(egui::Key::ArrowLeft) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::Left));
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::Right));
            }
            if i.key_pressed(egui::Key::ArrowUp) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::Up));
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::Down));
            }
            if i.key_pressed(egui::Key::S) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::Start));
            }
            if i.key_pressed(egui::Key::Tab) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::Select));
            }
            if i.key_pressed(egui::Key::Space) {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::A));
            }

            if i.modifiers.shift {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(ControllerEvent::B));
            }
        });
    }

    fn get_frame_budget(&mut self) -> Duration {
        let speed = self.config.speed_config.app_speed.get_fps(self);

        if speed == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(
            1_000_000_000 / self.config.speed_config.app_speed.get_fps(self) as u64,
        )
    }

    fn get_debug_viewers_frame_budget(&mut self) -> Duration {
        let fps = self.config.speed_config.debug_speed.get_fps(self);

        if fps == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / fps as u64)
    }

    fn process_emu_messages(&mut self, ctx: &Context) {
        while let Ok(msg) = self.from_emulator.try_recv() {
            match msg {
                EmulatorMessage::FrameReady(frame) => {
                    self.emu_textures.current_frame = Some(frame);
                    self.fps_counter.update();
                    self.update_emulator_texture(ctx);
                }
                EmulatorMessage::PatternTableReady(data) => {
                    self.emu_textures.pattern_table_data = Some(data);
                    self.update_pattern_table_texture(ctx);
                }
                EmulatorMessage::NametableReady(data) => {
                    self.emu_textures.nametable_data = Some(data);
                    self.update_nametable_texture(ctx);
                }
                EmulatorMessage::SpritesReady(_data) => {
                    // let (data, size) = data;
                    // self.emu_textures.sprite_height = size;
                    // self.emu_textures.sprite_viewer_data = Some(data);
                    // self.update_sprite_viewer_textures(ctx);
                }
                EmulatorMessage::Stopped => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }

    fn update_emu_textures(&mut self, ctx: &Context) {
        let now = Instant::now();
        if !self.config.speed_config.is_paused {
            let delta = now - self.emu_textures.last_frame_request;
            self.accumulator += delta;
            self.emu_textures.last_frame_request = now;

            let frame_budget = self.get_frame_budget();

            //Effectively paused, so we skip
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

            let debug_frame_budget = self.get_debug_viewers_frame_budget();

            //Effectively paused, so we skip
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
        self.handle_keyboard_input(ctx);

        // Check for escape to quit
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            let _ = self.to_emulator.send(FrontendMessage::Quit);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.update_emu_textures(ctx);

        // Process messages from emulator
        self.process_emu_messages(ctx);

        // Options side panel
        self.add_options_panel(ctx);

        // Status bar at bottom
        self.add_status_bar(ctx);

        // Emulator output windows
        self.add_emulator_views(ctx);

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
    let mut console = Consoles::Nes(Nes::default());

    // Load a ROM
    // TODO: Make this configurable via command line or file dialog
    console.load_rom(&file.to_string_lossy().take());
    console.power();

    // Create channel-based emulator wrapper
    let (mut channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);

    channel_emu.set_frontend(Frontends::Egui);

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
