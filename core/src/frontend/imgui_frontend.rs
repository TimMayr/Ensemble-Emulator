use std::fmt::{Debug, Formatter, Pointer};
/// ImGui-based frontend for the NES emulator using SDL2 + OpenGL.
///
/// This frontend provides a modern, multi-window debugging interface with:
/// - Real-time emulator output display with proper texture rendering
/// - Pattern table viewer (planned)
/// - Nametable viewer (planned)
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

#[cfg(feature = "imgui-frontend")]
use std::time::{Duration, Instant};

#[cfg(feature = "imgui-frontend")]
use crossbeam_channel::{Receiver, Sender};
#[cfg(feature = "imgui-frontend")]
use glow::HasContext;
#[cfg(feature = "imgui-frontend")]
use imgui_sdl2_support::SdlPlatform;
#[cfg(feature = "imgui-frontend")]
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
#[cfg(feature = "imgui-frontend")]
use sdl2::video::Window;

#[cfg(feature = "imgui-frontend")]
use crate::emulation::emu::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::emulation::messages::ControllerEvent;
#[cfg(feature = "imgui-frontend")]
use crate::emulation::messages::{
    EmulatorMessage, FrontendMessage, NAMETABLE_HEIGHT, NAMETABLE_WIDTH, PATTERN_TABLE_HEIGHT,
    PATTERN_TABLE_WIDTH,
};

#[cfg(feature = "imgui-frontend")]
pub struct ImGuiFrontend {
    imgui: imgui::Context,
    platform: SdlPlatform,
    renderer: imgui_glow_renderer::AutoRenderer,
    _gl_context: sdl2::video::GLContext,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
    current_frame: Option<Box<[u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>>,
    emulator_texture: Option<glow::Texture>,
    pattern_table_data: Option<Box<[u32; (PATTERN_TABLE_WIDTH * PATTERN_TABLE_HEIGHT) as usize]>>,
    pattern_table_texture: Option<glow::Texture>,
    nametable_data: Option<Box<[u32; (NAMETABLE_WIDTH * NAMETABLE_HEIGHT) as usize]>>,
    nametable_texture: Option<glow::Texture>,
    should_quit: bool,
    show_pattern_table: bool,
    show_nametable: bool,
    fps_counter: FpsCounter,
}

#[cfg(feature = "imgui-frontend")]
struct FpsCounter {
    frame_times: Vec<Instant>,
    last_update: Instant,
    current_fps: f32,
}

#[cfg(feature = "imgui-frontend")]
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

#[cfg(feature = "imgui-frontend")]
impl ImGuiFrontend {
    pub fn new(
        window: &Window,
        to_emulator: Sender<FrontendMessage>,
        from_emulator: Receiver<EmulatorMessage>,
    ) -> Result<Self, String> {
        // Create OpenGL context
        let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
        window
            .gl_make_current(&gl_context)
            .map_err(|e| e.to_string())?;
        window
            .subsystem()
            .gl_set_swap_interval(1)
            .map_err(|e| e.to_string())?;

        // Create glow context
        let gl = unsafe {
            glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
        };

        // Create ImGui context
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        // Setup fonts with proper DPI handling
        let (window_width, _window_height) = window.size();
        let (drawable_width, _drawable_height) = window.drawable_size();
        let scale_factor = (drawable_width as f32 / window_width as f32).max(1.0);

        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    size_pixels: (13.0 * scale_factor),
                    oversample_h: 2,
                    oversample_v: 2,
                    ..Default::default()
                }),
            }]);

        imgui.io_mut().font_global_scale = 1.0 / scale_factor;

        // Setup platform and renderer
        let platform = SdlPlatform::new(&mut imgui);
        let renderer =
            imgui_glow_renderer::AutoRenderer::new(gl, &mut imgui).map_err(|e| e.to_string())?;

        Ok(Self {
            imgui,
            platform,
            renderer,
            _gl_context: gl_context,
            to_emulator,
            from_emulator,
            current_frame: None,
            emulator_texture: None,
            pattern_table_data: None,
            pattern_table_texture: None,
            nametable_data: None,
            nametable_texture: None,
            should_quit: false,
            show_pattern_table: false,
            show_nametable: false,
            fps_counter: FpsCounter::new(),
        })
    }

    pub fn run(
        &mut self,
        sdl: &sdl2::Sdl,
        window: &Window,
        channel_emu: &mut crate::emulation::channel_emu::ChannelEmulator,
    ) -> Result<(), String> {
        let mut event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

        'main: loop {
            // Handle SDL events
            for event in event_pump.poll_iter() {
                self.platform.handle_event(&mut self.imgui, &event);

                match event {
                    Event::Quit {
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.should_quit = true;
                        break 'main;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::N),
                        ..
                    } => self
                        .to_emulator
                        .send(FrontendMessage::ControllerInput(
                            ControllerEvent::IncPalette,
                        ))
                        .unwrap(),
                    _ => {}
                }
            }

            // Step the emulator one frame
            if let Err(e) = channel_emu.step_frame() {
                eprintln!("Emulator error: {}", e);
                self.should_quit = true;
                break 'main;
            }

            // Check for messages from emulator
            while let Ok(msg) = self.from_emulator.try_recv() {
                match msg {
                    EmulatorMessage::FrameReady(frame) => {
                        self.current_frame = Some(frame);
                        self.fps_counter.update();
                        self.update_emulator_texture()?;
                    }
                    EmulatorMessage::PatternTableReady(data) => {
                        self.pattern_table_data = Some(data);
                        self.update_pattern_table_texture()?;
                    }
                    EmulatorMessage::NametableReady(data) => {
                        self.nametable_data = Some(data);
                        self.update_nametable_texture()?;
                    }
                    EmulatorMessage::Stopped => {
                        self.should_quit = true;
                        break 'main;
                    }
                }
            }

            // Render UI
            self.render(window, &event_pump)?;

            if self.should_quit {
                break 'main;
            }
        }

        // Send quit message to emulator
        let _ = self.to_emulator.send(FrontendMessage::Quit);

        Ok(())
    }

    fn update_emulator_texture(&mut self) -> Result<(), String> {
        if let Some(ref frame) = self.current_frame {
            let gl = self.renderer.gl_context();

            unsafe {
                // Create texture if it doesn't exist
                if self.emulator_texture.is_none() {
                    let texture = gl.create_texture().map_err(|e| e.to_string())?;
                    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MIN_FILTER,
                        glow::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MAG_FILTER,
                        glow::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_S,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_T,
                        glow::CLAMP_TO_EDGE as i32,
                    );

                    self.emulator_texture = Some(texture);
                }

                // Update texture with frame data
                if let Some(texture) = self.emulator_texture {
                    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

                    let bytes: &[u8] = bytemuck::cast_slice(&**frame);
                    // Use BGRA format because our u32 palette is RGBA but when cast to bytes
                    // on little-endian systems it becomes BGRA (0xRRGGBBAA -> [AA, BB, GG, RR])
                    gl.tex_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA as i32,
                        TOTAL_OUTPUT_WIDTH as i32,
                        TOTAL_OUTPUT_HEIGHT as i32,
                        0,
                        glow::BGRA, // Changed from RGBA to BGRA for correct color order
                        glow::UNSIGNED_BYTE,
                        Some(bytes),
                    );
                }
            }
        }

        Ok(())
    }

    fn update_pattern_table_texture(&mut self) -> Result<(), String> {
        if let Some(ref data) = self.pattern_table_data {
            let gl = self.renderer.gl_context();

            unsafe {
                // Create texture if it doesn't exist
                if self.pattern_table_texture.is_none() {
                    let texture = gl.create_texture().map_err(|e| e.to_string())?;
                    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MIN_FILTER,
                        glow::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MAG_FILTER,
                        glow::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_S,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_T,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    self.pattern_table_texture = Some(texture);
                }

                // Update texture with pattern table data
                if let Some(texture) = self.pattern_table_texture {
                    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                    let bytes: &[u8] = bytemuck::cast_slice(&**data);
                    gl.tex_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA as i32,
                        PATTERN_TABLE_WIDTH as i32,
                        PATTERN_TABLE_HEIGHT as i32,
                        0,
                        glow::BGRA,
                        glow::UNSIGNED_BYTE,
                        Some(bytes),
                    );
                }
            }
        }

        Ok(())
    }

    fn update_nametable_texture(&mut self) -> Result<(), String> {
        if let Some(ref data) = self.nametable_data {
            let gl = self.renderer.gl_context();

            unsafe {
                // Create texture if it doesn't exist
                if self.nametable_texture.is_none() {
                    let texture = gl.create_texture().map_err(|e| e.to_string())?;
                    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MIN_FILTER,
                        glow::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_MAG_FILTER,
                        glow::NEAREST as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_S,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    gl.tex_parameter_i32(
                        glow::TEXTURE_2D,
                        glow::TEXTURE_WRAP_T,
                        glow::CLAMP_TO_EDGE as i32,
                    );
                    self.nametable_texture = Some(texture);
                }

                // Update texture with nametable data
                if let Some(texture) = self.nametable_texture {
                    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                    let bytes: &[u8] = bytemuck::cast_slice(&**data);
                    gl.tex_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA as i32,
                        NAMETABLE_WIDTH as i32,
                        NAMETABLE_HEIGHT as i32,
                        0,
                        glow::BGRA,
                        glow::UNSIGNED_BYTE,
                        Some(bytes),
                    );
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, window: &Window, event_pump: &sdl2::EventPump) -> Result<(), String> {
        // Handle window visibility changes and send enable/disable messages
        if self.show_pattern_table && self.pattern_table_data.is_none() {
            let _ = self
                .to_emulator
                .send(FrontendMessage::EnablePatternTableRendering(true));
        } else if !self.show_pattern_table && self.pattern_table_data.is_some() {
            let _ = self
                .to_emulator
                .send(FrontendMessage::EnablePatternTableRendering(false));
            self.pattern_table_data = None;
        }

        if self.show_nametable && self.nametable_data.is_none() {
            let _ = self
                .to_emulator
                .send(FrontendMessage::EnableNametableRendering(true));
        } else if !self.show_nametable && self.nametable_data.is_some() {
            let _ = self
                .to_emulator
                .send(FrontendMessage::EnableNametableRendering(false));
            self.nametable_data = None;
        }

        self.platform
            .prepare_frame(&mut self.imgui, window, event_pump);

        let ui = self.imgui.new_frame();

        // Render UI elements
        let show_pattern_table = self.show_pattern_table;
        let show_nametable = self.show_nametable;
        let fps = self.fps_counter.fps();
        let has_frame = self.current_frame.is_some();
        let emulator_texture = self.emulator_texture;
        let pattern_table_texture = self.pattern_table_texture;
        let nametable_texture = self.nametable_texture;

        render_ui_static(
            ui,
            show_pattern_table,
            show_nametable,
            fps,
            has_frame,
            emulator_texture,
            pattern_table_texture,
            nametable_texture,
            &mut self.show_pattern_table,
            &mut self.show_nametable,
        );

        // Render to screen
        let gl = self.renderer.gl_context();
        unsafe {
            gl.clear_color(0.18, 0.18, 0.19, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        let draw_data = self.imgui.render();
        self.renderer.render(draw_data).map_err(|e| e.to_string())?;

        window.gl_swap_window();

        Ok(())
    }
}

#[cfg(feature = "imgui-frontend")]
fn render_ui_static(
    ui: &imgui::Ui,
    show_pattern_table: bool,
    show_nametable: bool,
    fps: f32,
    has_frame: bool,
    emulator_texture: Option<glow::Texture>,
    pattern_table_texture: Option<glow::Texture>,
    nametable_texture: Option<glow::Texture>,
    show_pattern_table_mut: &mut bool,
    show_nametable_mut: &mut bool,
) {
    // Main menu bar
    if let Some(_menu_bar) = ui.begin_main_menu_bar() {
        if let Some(_menu) = ui.begin_menu("View") {
            ui.checkbox("Pattern Table Viewer", show_pattern_table_mut);
            ui.checkbox("Nametable Viewer", show_nametable_mut);
        }
    }

    // Emulator output window (always visible)
    ui.window("Emulator Output")
        .size([640.0, 480.0], imgui::Condition::FirstUseEver)
        .position([50.0, 50.0], imgui::Condition::FirstUseEver)
        .build(|| {
            if let Some(texture) = emulator_texture {
                // Get available content region
                let content_region = ui.content_region_avail();

                // Calculate display size (scale to fit window while maintaining aspect ratio)
                let scale = (content_region[0] / TOTAL_OUTPUT_WIDTH as f32)
                    .min(content_region[1] / TOTAL_OUTPUT_HEIGHT as f32)
                    .min(4.0); // Cap at 4x scale

                let display_width = TOTAL_OUTPUT_WIDTH as f32 * scale;
                let display_height = TOTAL_OUTPUT_HEIGHT as f32 * scale;

                ui.text(format!(
                    "{}x{} at {:.1}x scale",
                    TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale
                ));

                // Create texture ID from OpenGL texture
                let texture_id = imgui::TextureId::new(texture.0.get() as usize);

                imgui::Image::new(texture_id, [display_width, display_height]).build(ui);
            } else {
                ui.text("Waiting for first frame...");
            }
        });

    // Optional windows
    if show_pattern_table {
        ui.window("Pattern Table Viewer")
            .size([580.0, 300.0], imgui::Condition::FirstUseEver)
            .position([700.0, 50.0], imgui::Condition::FirstUseEver)
            .opened(show_pattern_table_mut)
            .build(|| {
                if let Some(texture) = pattern_table_texture {
                    let content_region = ui.content_region_avail();
                    let scale = (content_region[0] / PATTERN_TABLE_WIDTH as f32)
                        .min(content_region[1] / PATTERN_TABLE_HEIGHT as f32)
                        .min(2.0);

                    let display_width = PATTERN_TABLE_WIDTH as f32 * scale;
                    let display_height = PATTERN_TABLE_HEIGHT as f32 * scale;

                    ui.text(format!(
                        "Pattern Tables ({}x{} at {:.1}x)",
                        PATTERN_TABLE_WIDTH, PATTERN_TABLE_HEIGHT, scale
                    ));

                    let texture_id = imgui::TextureId::new(texture.0.get() as usize);
                    imgui::Image::new(texture_id, [display_width, display_height]).build(ui);
                } else {
                    ui.text("Waiting for pattern table data...");
                }
            });
    }

    if show_nametable {
        ui.window("Nametable Viewer")
            .size([600.0, 560.0], imgui::Condition::FirstUseEver)
            .position([700.0, 370.0], imgui::Condition::FirstUseEver)
            .opened(show_nametable_mut)
            .build(|| {
                if let Some(texture) = nametable_texture {
                    let content_region = ui.content_region_avail();
                    let scale = (content_region[0] / NAMETABLE_WIDTH as f32)
                        .min(content_region[1] / NAMETABLE_HEIGHT as f32)
                        .min(1.0);

                    let display_width = NAMETABLE_WIDTH as f32 * scale;
                    let display_height = NAMETABLE_HEIGHT as f32 * scale;

                    ui.text(format!(
                        "Nametables ({}x{} at {:.1}x)",
                        NAMETABLE_WIDTH, NAMETABLE_HEIGHT, scale
                    ));

                    let texture_id = imgui::TextureId::new(texture.0.get() as usize);
                    imgui::Image::new(texture_id, [display_width, display_height]).build(ui);
                } else {
                    ui.text("Waiting for nametable data...");
                }
            });
    }

    // Status bar
    let window_size = ui.io().display_size;
    let status_height = 25.0;

    ui.window("StatusBar")
        .position(
            [0.0, window_size[1] - status_height],
            imgui::Condition::Always,
        )
        .size([window_size[0], status_height], imgui::Condition::Always)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .scrollable(false)
        .build(|| {
            ui.text(format!("FPS: {:.1}", fps));
            ui.same_line();
            ui.text("|");
            ui.same_line();
            if has_frame {
                ui.text("Emulator: Running");
            } else {
                ui.text("Emulator: Initializing");
            }
        });
}

impl Debug for ImGuiFrontend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("ImguiFrontend") }
}
