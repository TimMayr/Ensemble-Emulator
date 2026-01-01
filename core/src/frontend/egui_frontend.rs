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
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use egui::{ColorImage, TextureHandle, TextureOptions};

use crate::emulation::channel_emu::ChannelEmulator;
use crate::emulation::emu::{Console, Consoles, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::emulation::messages::{
    ControllerEvent, EmulatorMessage, FrontendMessage, NAMETABLE_HEIGHT, NAMETABLE_WIDTH,
    PATTERN_TABLE_HEIGHT, PATTERN_TABLE_WIDTH,
};
use crate::emulation::nes::Nes;
use crate::frontend::Frontends;

/// FPS counter that tracks frame times over the last second
struct FpsCounter {
    frame_times: Vec<Instant>,
    last_update: Instant,
    current_fps: f32,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_times: Vec::new(),
            last_update: Instant::now(),
            current_fps: 0.0,
        }
    }

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

/// Main egui application state
pub struct EguiApp {
    channel_emu: ChannelEmulator,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
    current_frame: Option<Box<[u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>>,
    emulator_texture: Option<TextureHandle>,
    pattern_table_data: Option<Box<[u32; (PATTERN_TABLE_WIDTH * PATTERN_TABLE_HEIGHT) as usize]>>,
    pattern_table_texture: Option<TextureHandle>,
    nametable_data: Option<Box<[u32; (NAMETABLE_WIDTH * NAMETABLE_HEIGHT) as usize]>>,
    nametable_texture: Option<TextureHandle>,
    show_pattern_table: bool,
    show_nametable: bool,
    fps_counter: FpsCounter,
    last_pattern_table_request: Instant,
    last_nametable_request: Instant,
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
            current_frame: None,
            emulator_texture: None,
            pattern_table_data: None,
            pattern_table_texture: None,
            nametable_data: None,
            nametable_texture: None,
            show_pattern_table: false,
            show_nametable: false,
            fps_counter: FpsCounter::new(),
            last_pattern_table_request: Instant::now(),
            last_nametable_request: Instant::now(),
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

    fn update_emulator_texture(&mut self, ctx: &egui::Context) {
        if let Some(ref frame) = self.current_frame {
            let image = Self::u32_to_color_image(
                frame.as_ref(),
                TOTAL_OUTPUT_WIDTH as usize,
                TOTAL_OUTPUT_HEIGHT as usize,
            );

            let texture = ctx.load_texture(
                "emulator_output",
                image,
                TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                },
            );
            self.emulator_texture = Some(texture);
        }
    }

    fn update_pattern_table_texture(&mut self, ctx: &egui::Context) {
        if let Some(ref data) = self.pattern_table_data {
            let image = Self::u32_to_color_image(
                data.as_ref(),
                PATTERN_TABLE_WIDTH as usize,
                PATTERN_TABLE_HEIGHT as usize,
            );

            let texture = ctx.load_texture(
                "pattern_table",
                image,
                TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                },
            );
            self.pattern_table_texture = Some(texture);
        }
    }

    fn update_nametable_texture(&mut self, ctx: &egui::Context) {
        if let Some(ref data) = self.nametable_data {
            let image = Self::u32_to_color_image(
                data.as_ref(),
                NAMETABLE_WIDTH as usize,
                NAMETABLE_HEIGHT as usize,
            );

            let texture = ctx.load_texture(
                "nametable",
                image,
                TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                },
            );
            self.nametable_texture = Some(texture);
        }
    }

    fn handle_keyboard_input(&self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Emulator controls
            if i.key_pressed(egui::Key::N) {
                let _ = self.to_emulator.send(FrontendMessage::ControllerInput(
                    ControllerEvent::IncPalette,
                ));
            }
            if i.key_pressed(egui::Key::Period) {
                let _ = self.to_emulator.send(FrontendMessage::Pause);
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
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        self.handle_keyboard_input(ctx);

        // Check for escape to quit
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            let _ = self.to_emulator.send(FrontendMessage::Quit);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Step the emulator one frame
        if let Err(e) = self.channel_emu.step_frame() {
            eprintln!("Emulator error: {}", e);
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Process messages from emulator
        while let Ok(msg) = self.from_emulator.try_recv() {
            match msg {
                EmulatorMessage::FrameReady(frame) => {
                    self.current_frame = Some(frame);
                    self.fps_counter.update();
                    self.update_emulator_texture(ctx);
                }
                EmulatorMessage::PatternTableReady(data) => {
                    self.pattern_table_data = Some(data);
                    self.update_pattern_table_texture(ctx);
                }
                EmulatorMessage::NametableReady(data) => {
                    self.nametable_data = Some(data);
                    self.update_nametable_texture(ctx);
                }
                EmulatorMessage::Stopped => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }

        // Request debug view updates at a reasonable rate (10 FPS for debug views)
        const DEBUG_UPDATE_INTERVAL: Duration = Duration::from_millis(100);

        if self.show_pattern_table {
            let now = Instant::now();
            if now.duration_since(self.last_pattern_table_request) >= DEBUG_UPDATE_INTERVAL {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::RequestPatternTableData);
                self.last_pattern_table_request = now;
            }
        } else if self.pattern_table_data.is_some() {
            self.pattern_table_data = None;
        }

        if self.show_nametable {
            let now = Instant::now();
            if now.duration_since(self.last_nametable_request) >= DEBUG_UPDATE_INTERVAL {
                let _ = self.to_emulator.send(FrontendMessage::RequestNametableData);
                self.last_nametable_request = now;
            }
        } else if self.nametable_data.is_some() {
            self.nametable_data = None;
        }

        // Main menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::containers::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_pattern_table, "Pattern Table Viewer");
                    ui.checkbox(&mut self.show_nametable, "Nametable Viewer");
                });
            });
        });

        // Status bar at bottom
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("FPS: {:.1}", self.fps_counter.fps()));
                ui.separator();
                if self.channel_emu.paused {
                    ui.label("Emulator: Paused");
                } else if self.current_frame.is_some() {
                    ui.label("Emulator: Running");
                } else {
                    ui.label("Emulator: Initializing");
                }
            });
        });

        // Emulator output window (always visible)
        egui::Window::new("Emulator Output")
            .default_size([640.0, 480.0])
            .default_pos([50.0, 50.0])
            .show(ctx, |ui| {
                if let Some(ref texture) = self.emulator_texture {
                    // Get available content region
                    let available = ui.available_size();

                    // Calculate display size (scale to fit window while maintaining aspect ratio)
                    let scale = (available.x / TOTAL_OUTPUT_WIDTH as f32)
                        .min(available.y / TOTAL_OUTPUT_HEIGHT as f32)
                        .min(4.0); // Cap at 4x scale

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
        if self.show_pattern_table {
            egui::Window::new("Pattern Table Viewer")
                .default_size([580.0, 300.0])
                .default_pos([700.0, 50.0])
                .open(&mut self.show_pattern_table)
                .show(ctx, |ui| {
                    if let Some(ref texture) = self.pattern_table_texture {
                        let available = ui.available_size();
                        let scale = (available.x / PATTERN_TABLE_WIDTH as f32)
                            .min(available.y / PATTERN_TABLE_HEIGHT as f32)
                            .min(2.0);

                        let display_width = PATTERN_TABLE_WIDTH as f32 * scale;
                        let display_height = PATTERN_TABLE_HEIGHT as f32 * scale;

                        ui.label(format!(
                            "Pattern Tables ({}x{} at {:.1}x)",
                            PATTERN_TABLE_WIDTH, PATTERN_TABLE_HEIGHT, scale
                        ));

                        ui.image((texture.id(), egui::vec2(display_width, display_height)));
                    } else {
                        ui.label("Waiting for pattern table data...");
                    }
                });
        }

        // Nametable Viewer window (optional)
        if self.show_nametable {
            egui::Window::new("Nametable Viewer")
                .default_size([600.0, 560.0])
                .default_pos([700.0, 370.0])
                .open(&mut self.show_nametable)
                .show(ctx, |ui| {
                    if let Some(ref texture) = self.nametable_texture {
                        let available = ui.available_size();
                        let scale = (available.x / NAMETABLE_WIDTH as f32)
                            .min(available.y / NAMETABLE_HEIGHT as f32)
                            .min(1.0);

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
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Create the emulator instance
    let mut console = Consoles::Nes(Nes::default());

    // Load a ROM
    // TODO: Make this configurable via command line or file dialog
    console.load_rom(&String::from("./core/tests/Galaga (U).nes"));
    console.power();

    // Create channel-based emulator wrapper
    let (mut channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);

    channel_emu.set_frontend(Frontends::Egui());

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
        Box::new(|_cc| Ok(Box::new(EguiApp::new(channel_emu, tx_to_emu, rx_from_emu)))),
    )?;

    Ok(())
}
