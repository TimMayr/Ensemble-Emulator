/// ImGui-based frontend for the NES emulator using SDL3.
///
/// This frontend provides a modern, multi-window debugging interface with:
/// - Real-time emulator output display
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
use imgui_sdl3::ImGuiSdl3;
#[cfg(feature = "imgui-frontend")]
use sdl3::event::Event;
#[cfg(feature = "imgui-frontend")]
use sdl3::gpu::*;
#[cfg(feature = "imgui-frontend")]
use sdl3::pixels::Color;

#[cfg(feature = "imgui-frontend")]
use crate::emulation::emu::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
#[cfg(feature = "imgui-frontend")]
use crate::emulation::messages::{EmulatorMessage, FrontendMessage};

#[cfg(feature = "imgui-frontend")]
pub struct ImGuiFrontend {
    imgui: ImGuiSdl3,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
    current_frame: Option<Box<[u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>>,
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
        device: &Device,
        window: &sdl3::video::Window,
        to_emulator: Sender<FrontendMessage>,
        from_emulator: Receiver<EmulatorMessage>,
    ) -> Result<Self, String> {
        // Create ImGui context
        let imgui = ImGuiSdl3::new(device, window, |ctx| {
            // Disable creation of files on disc
            ctx.set_ini_filename(None);
            ctx.set_log_filename(None);

            // Get display scale for proper DPI handling
            let scale_factor = window.display_scale().max(1.0);
            
            // Setup fonts with proper scaling for high DPI displays
            ctx.fonts().add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    size_pixels: (13.0 * scale_factor),
                    oversample_h: 2,
                    oversample_v: 2,
                    pixel_snap_h: true,
                    ..Default::default()
                }),
            }]);
            
            // Set display scale to handle high DPI displays
            let style = ctx.style_mut();
            style.scale_all_sizes(scale_factor);
        });

        Ok(Self {
            imgui,
            to_emulator,
            from_emulator,
            current_frame: None,
            should_quit: false,
            show_pattern_table: false,
            show_nametable: false,
            fps_counter: FpsCounter::new(),
        })
    }

    pub fn run(
        &mut self,
        sdl: &mut sdl3::Sdl,
        window: &sdl3::video::Window,
        device: &Device,
        channel_emu: &mut crate::emulation::channel_emu::ChannelEmulator,
    ) -> Result<(), String> {
        let mut event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

        'main: loop {
            // Handle SDL events
            for event in event_pump.poll_iter() {
                self.imgui.handle_event(&event);

                match event {
                    Event::Quit { .. } => {
                        self.should_quit = true;
                        break 'main;
                    }
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
                    }
                    EmulatorMessage::Stopped => {
                        self.should_quit = true;
                        break 'main;
                    }
                }
            }

            // Render UI
            self.render(sdl, window, device, &mut event_pump)?;

            if self.should_quit {
                break 'main;
            }
        }

        // Send quit message to emulator
        let _ = self.to_emulator.send(FrontendMessage::Quit);

        Ok(())
    }

    fn render(
        &mut self,
        sdl: &mut sdl3::Sdl,
        window: &sdl3::video::Window,
        device: &Device,
        event_pump: &mut sdl3::EventPump,
    ) -> Result<(), String> {
        let mut command_buffer = device
            .acquire_command_buffer()
            .map_err(|e| format!("Failed to acquire command buffer: {:?}", e))?;

        if let Ok(swapchain) = command_buffer.wait_and_acquire_swapchain_texture(window) {
            let color_targets = [ColorTargetInfo::default()
                .with_texture(&swapchain)
                .with_load_op(LoadOp::CLEAR)
                .with_store_op(StoreOp::STORE)
                .with_clear_color(Color::RGB(45, 45, 48))];

            // Prepare UI data before rendering
            let current_fps = self.fps_counter.fps();
            let frame_data = self.current_frame.clone();

            self.imgui.render(
                sdl,
                device,
                window,
                event_pump,
                &mut command_buffer,
                &color_targets,
                |ui| {
                    render_ui_static(
                        ui,
                        &mut self.show_pattern_table,
                        &mut self.show_nametable,
                        current_fps,
                        frame_data.as_deref(),
                    );
                },
            );

            command_buffer
                .submit()
                .map_err(|e| format!("Failed to submit command buffer: {:?}", e))?;
        } else {
            command_buffer.cancel();
        }

        Ok(())
    }
}

#[cfg(feature = "imgui-frontend")]
#[allow(unused_variables)] // show_pattern_table and show_nametable will be used when we add keyboard shortcuts
fn render_ui_static(
    ui: &imgui::Ui,
    show_pattern_table: &mut bool,
    show_nametable: &mut bool,
    current_fps: f32,
    frame_data: Option<&[u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
) {
    // Main menu bar
    if let Some(_menu_bar) = ui.begin_main_menu_bar()
        && let Some(_menu) = ui.begin_menu("View") {
            ui.checkbox("Pattern Table Viewer", show_pattern_table);
            ui.checkbox("Nametable Viewer", show_nametable);
        }

    // Emulator output window (always visible)
    ui.window("Emulator Output")
        .size([640.0, 480.0], imgui::Condition::FirstUseEver)
        .position([50.0, 50.0], imgui::Condition::FirstUseEver)
        .build(|| {
            if let Some(frame) = frame_data {
                // Draw the frame using ImGui's draw list
                let draw_list = ui.get_window_draw_list();
                let cursor_pos = ui.cursor_screen_pos();
                
                // Calculate display size (scale to fit window while maintaining aspect ratio)
                let content_region = ui.content_region_avail();
                let scale = (content_region[0] / TOTAL_OUTPUT_WIDTH as f32)
                    .min(content_region[1] / TOTAL_OUTPUT_HEIGHT as f32)
                    .min(3.0); // Cap at 3x scale
                
                let display_width = (TOTAL_OUTPUT_WIDTH as f32 * scale) as usize;
                let display_height = (TOTAL_OUTPUT_HEIGHT as f32 * scale) as usize;
                
                ui.text(format!("Rendering {}x{} at {:.1}x scale", 
                    TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale));
                ui.text(format!("Display size: {}x{}", display_width, display_height));
                
                // For now, draw a downsampled preview using colored rectangles
                // This is a fallback until we have proper texture rendering
                let pixel_size = scale.max(1.0);
                let skip = if scale < 1.0 { (1.0 / scale) as usize } else { 1 };
                
                for y in (0..TOTAL_OUTPUT_HEIGHT as usize).step_by(skip) {
                    for x in (0..TOTAL_OUTPUT_WIDTH as usize).step_by(skip) {
                        let pixel_index = y * TOTAL_OUTPUT_WIDTH as usize + x;
                        if pixel_index < frame.len() {
                            let color_u32 = frame[pixel_index];
                            
                            // Convert RGBA to ImGui color (ABGR format)
                            let r = ((color_u32 >> 24) & 0xFF) as f32 / 255.0;
                            let g = ((color_u32 >> 16) & 0xFF) as f32 / 255.0;
                            let b = ((color_u32 >> 8) & 0xFF) as f32 / 255.0;
                            let a = (color_u32 & 0xFF) as f32 / 255.0;
                            
                            let x_pos = cursor_pos[0] + (x as f32 * pixel_size);
                            let y_pos = cursor_pos[1] + 20.0 + (y as f32 * pixel_size);
                            
                            draw_list
                                .add_rect(
                                    [x_pos, y_pos],
                                    [x_pos + pixel_size, y_pos + pixel_size],
                                    [r, g, b, a],
                                )
                                .filled(true)
                                .build();
                        }
                    }
                }
                
                // Reserve space for the display
                ui.dummy([display_width as f32, display_height as f32 + 40.0]);
            } else {
                ui.text("Waiting for first frame...");
            }
        });

    // Optional windows
    if *show_pattern_table {
        ui.window("Pattern Table Viewer")
            .size([400.0, 300.0], imgui::Condition::FirstUseEver)
            .position([700.0, 50.0], imgui::Condition::FirstUseEver)
            .opened(show_pattern_table)
            .build(|| {
                ui.text("Pattern Table visualization");
                ui.separator();
                ui.text("TODO: Implement pattern table rendering");
                ui.text_wrapped(
                    "This window will display the CHR ROM pattern tables \
                     used for sprites and background tiles.",
                );
            });
    }

    if *show_nametable {
        ui.window("Nametable Viewer")
            .size([400.0, 300.0], imgui::Condition::FirstUseEver)
            .position([700.0, 370.0], imgui::Condition::FirstUseEver)
            .opened(show_nametable)
            .build(|| {
                ui.text("Nametable visualization");
                ui.separator();
                ui.text("TODO: Implement nametable rendering");
                ui.text_wrapped(
                    "This window will display the nametables showing \
                     the background tile arrangement.",
                );
            });
    }

    // Status bar
    let window_size = ui.io().display_size;
    let status_height = 25.0;

    ui.window("StatusBar")
        .position([0.0, window_size[1] - status_height], imgui::Condition::Always)
        .size([window_size[0], status_height], imgui::Condition::Always)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .scrollable(false)
        .build(|| {
            ui.text(format!("FPS: {:.1}", current_fps));
            ui.same_line();
            ui.text("|");
            ui.same_line();
            if frame_data.is_some() {
                ui.text("Emulator: Running");
            } else {
                ui.text("Emulator: Initializing");
            }
        });
}
