#![cfg(feature = "imgui-sdl3-frontend")]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use glow::HasContext;
use imgui::{Condition, ConfigFlags, TextureId, Ui};
use imgui_opengl_renderer::Renderer;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;
use sdl3::video::gl_attr::GLProfile;

use crate::emulation::emu::{InputEvent, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::Frontend;

const DISPLAY_SCALE: f32 = 2.0;
const FRAME_STRIDE: usize = (TOTAL_OUTPUT_WIDTH as usize) * 4;

pub struct ImguiSdl3Frontend {
    frame: Arc<SharedFrame>,
    input_rx: Receiver<InputEvent>,
    command_tx: Sender<FrontendCommand>,
    result_rx: Receiver<Result<(), String>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Default for ImguiSdl3Frontend {
    fn default() -> Self { Self::new().expect("failed to start ImGui SDL3 frontend") }
}

impl ImguiSdl3Frontend {
    pub fn new() -> Result<Self, String> {
        let frame = Arc::new(SharedFrame::new(
            (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT * 4) as usize,
        ));
        let (input_tx, input_rx) = mpsc::channel();
        let (command_tx, command_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();

        let frame_clone = frame.clone();
        let thread_input = input_tx.clone();

        let handle = thread::Builder::new()
            .name("imgui-sdl3-frontend".into())
            .spawn(move || {
                let res = run_event_loop(frame_clone, thread_input, command_rx);
                let _ = result_tx.send(res);
            })
            .map_err(|err| err.to_string())?;

        Ok(Self {
            frame,
            input_rx,
            command_tx,
            result_rx,
            thread_handle: Some(handle),
        })
    }
}

impl Frontend for ImguiSdl3Frontend {
    fn show_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), String> {
        let bytes: &[u8] = bytemuck::cast_slice(pixel_buffer);
        self.frame.store(bytes);
        Ok(())
    }

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        let mut events = Vec::new();
        for event in self.input_rx.try_iter() {
            events.push(event);
        }
        Ok(events)
    }
}

impl Drop for ImguiSdl3Frontend {
    fn drop(&mut self) {
        let _ = self.command_tx.send(FrontendCommand::Shutdown);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        // Drain the result channel to surface any error during drop.
        while let Ok(result) = self.result_rx.try_recv() {
            if let Err(err) = result {
                log::error!("Frontend thread terminated with error: {err}");
            }
        }
    }
}

struct SharedFrame {
    pixels: Mutex<Vec<u8>>,
    version: AtomicU64,
}

impl SharedFrame {
    fn new(len: usize) -> Self {
        Self {
            pixels: Mutex::new(vec![0; len]),
            version: AtomicU64::new(0),
        }
    }

    fn store(&self, data: &[u8]) {
        let mut guard = self.pixels.lock().expect("frame mutex poisoned");
        if guard.len() != data.len() {
            guard.resize(data.len(), 0);
        }
        guard.copy_from_slice(data);
        self.version.fetch_add(1, Ordering::Release);
    }

    fn copy_if_new(&self, last_version: &mut u64, target: &mut Vec<u8>) -> bool {
        let current = self.version.load(Ordering::Acquire);
        if current == *last_version {
            return false;
        }

        let guard = self.pixels.lock().expect("frame mutex poisoned");
        if guard.len() != target.len() {
            target.resize(guard.len(), 0);
        }
        target.copy_from_slice(&guard);
        *last_version = current;
        true
    }
}

#[derive(Copy, Clone, Debug)]
enum FrontendCommand {
    Shutdown,
}

struct EmulatorOutputView {
    texture: glow::Texture,
    texture_id: TextureId,
    width: i32,
    height: i32,
    version: u64,
    staging: Vec<u8>,
}

impl EmulatorOutputView {
    fn new(gl: &glow::Context, renderer: &mut Renderer) -> Result<Self, String> {
        unsafe {
            let texture = gl
                .create_texture()
                .map_err(|err| format!("Failed to create texture: {err}"))?;
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
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

            let empty = vec![0u8; FRAME_STRIDE * TOTAL_OUTPUT_HEIGHT as usize];
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                TOTAL_OUTPUT_WIDTH as i32,
                TOTAL_OUTPUT_HEIGHT as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(glow::PixelUnpackData::Slice(&empty)),
            );

            let texture_id = renderer.textures().insert(texture);

            Ok(Self {
                texture,
                texture_id,
                width: TOTAL_OUTPUT_WIDTH as i32,
                height: TOTAL_OUTPUT_HEIGHT as i32,
                version: 0,
                staging: vec![0u8; FRAME_STRIDE * TOTAL_OUTPUT_HEIGHT as usize],
            })
        }
    }

    fn sync_texture(&mut self, gl: &glow::Context, frame: &SharedFrame) -> Result<(), String> {
        if !frame.copy_if_new(&mut self.version, &mut self.staging) {
            return Ok(());
        }

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                self.width,
                self.height,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&self.staging),
            );
        }

        Ok(())
    }

    fn draw(&self, ui: &Ui) {
        let size = [
            self.width as f32 * DISPLAY_SCALE,
            self.height as f32 * DISPLAY_SCALE,
        ];
        imgui::Image::new(self.texture_id, size).build(ui);
    }

    unsafe fn destroy(&mut self, gl: &glow::Context, renderer: &mut Renderer) {
        renderer.textures().remove(self.texture_id);
        gl.delete_texture(self.texture);
    }
}

fn run_event_loop(
    frame: Arc<SharedFrame>,
    input_tx: Sender<InputEvent>,
    command_rx: Receiver<FrontendCommand>,
) -> Result<(), String> {
    let sdl = sdl3::init().map_err(|err| err.to_string())?;
    let video = sdl.video().map_err(|err| err.to_string())?;

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(true);

    let window = video
        .window(
            "EmulatorThingimabob",
            (TOTAL_OUTPUT_WIDTH as f32 * DISPLAY_SCALE) as u32,
            (TOTAL_OUTPUT_HEIGHT as f32 * DISPLAY_SCALE) as u32,
        )
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|err| err.to_string())?;

    let gl_context = window.gl_create_context().map_err(|err| err.to_string())?;
    window
        .gl_make_current(&gl_context)
        .map_err(|err| err.to_string())?;

    // Enable vsync for smoother presentation when available.
    let gl = unsafe { glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as _) };

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    imgui.io_mut().config_flags |= ConfigFlags::DOCKING_ENABLE;
    imgui.io_mut().fonts.add_default_font();

    let mut renderer = unsafe {
        Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _)
            .map_err(|err| err.to_string())?
    };

    let mut output_view = EmulatorOutputView::new(&gl, &mut renderer)?;
    let mut event_pump = sdl.event_pump().map_err(|err| err.to_string())?;

    let mut last_frame = Instant::now();
    let mut running = true;

    while running {
        if let Ok(command) = command_rx.try_recv() {
            if matches!(command, FrontendCommand::Shutdown) {
                running = false;
                continue;
            }
        }

        for event in event_pump.poll_iter() {
            handle_event(&mut imgui, &event, &input_tx);
            if matches!(event, Event::Quit { .. }) {
                running = false;
            }
        }

        let now = Instant::now();
        let delta = now - last_frame;
        last_frame = now;

        prepare_frame(&mut imgui, &window, delta);

        output_view.sync_texture(&gl, &frame)?;

        let ui = imgui.frame();
        build_ui(&ui, &output_view, &input_tx);

        unsafe {
            let (width, height) = window.size();
            gl.viewport(0, 0, width as i32, height as i32);
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        renderer
            .render(imgui.render())
            .map_err(|err| err.to_string())?;

        window.gl_swap_window();
    }

    unsafe {
        output_view.destroy(&gl, &mut renderer);
    }

    Ok(())
}

fn prepare_frame(imgui: &mut imgui::Context, window: &sdl3::video::Window, delta: Duration) {
    let io = imgui.io_mut();
    io.update_delta_time(delta);

    let (width, height) = window.size();
    io.display_size = [width as f32, height as f32];
}

fn build_ui(ui: &Ui, output_view: &EmulatorOutputView, input_tx: &Sender<InputEvent>) {
    ui.window("Emulator")
        .size(
            [
                TOTAL_OUTPUT_WIDTH as f32 * DISPLAY_SCALE,
                TOTAL_OUTPUT_HEIGHT as f32 * DISPLAY_SCALE,
            ],
            Condition::FirstUseEver,
        )
        .build(|| {
            output_view.draw(ui);
        });

    ui.window("Pattern Tables")
        .size([320.0, 320.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("Pattern table viewer not implemented yet");
        });

    ui.window("Nametable")
        .size([320.0, 320.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("Nametable viewer not implemented yet");
        });

    ui.window("Controls")
        .size([260.0, 120.0], Condition::FirstUseEver)
        .build(|| {
            if ui.button("Next Palette") {
                let _ = input_tx.send(InputEvent::IncPalette);
            }
            ui.same_line();
            if ui.button("Quit") {
                let _ = input_tx.send(InputEvent::Quit);
            }
        });
}

fn handle_event(imgui: &mut imgui::Context, event: &Event, input_tx: &Sender<InputEvent>) {
    let io = imgui.io_mut();

    match event {
        Event::Quit {
            ..
        } => {
            let _ = input_tx.send(InputEvent::Quit);
        }
        Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            let _ = input_tx.send(InputEvent::Quit);
            io.add_key_event(imgui::Key::Escape, true);
        }
        Event::KeyUp {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            io.add_key_event(imgui::Key::Escape, false);
        }
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => {
            let _ = input_tx.send(InputEvent::IncPalette);
            io.add_key_event(imgui::Key::Space, true);
        }
        Event::KeyUp {
            keycode: Some(Keycode::Space),
            ..
        } => {
            io.add_key_event(imgui::Key::Space, false);
        }
        Event::MouseMotion {
            x,
            y,
            ..
        } => {
            io.add_mouse_pos_event([*x as f32, *y as f32]);
        }
        Event::MouseButtonDown {
            mouse_btn, ..
        } => {
            if let Some(button) = map_mouse_button(*mouse_btn) {
                io.add_mouse_button_event(button, true);
            }
        }
        Event::MouseButtonUp {
            mouse_btn, ..
        } => {
            if let Some(button) = map_mouse_button(*mouse_btn) {
                io.add_mouse_button_event(button, false);
            }
        }
        Event::MouseWheel {
            x,
            y,
            ..
        } => {
            io.add_mouse_wheel_event([*x as f32, *y as f32]);
        }
        Event::TextInput {
            text, ..
        } => {
            io.add_input_characters_utf8(text);
        }
        _ => {}
    }
}

fn map_mouse_button(button: MouseButton) -> Option<imgui::MouseButton> {
    match button {
        MouseButton::Left => Some(imgui::MouseButton::Left),
        MouseButton::Right => Some(imgui::MouseButton::Right),
        MouseButton::Middle => Some(imgui::MouseButton::Middle),
        MouseButton::X1 => Some(imgui::MouseButton::Extra1),
        MouseButton::X2 => Some(imgui::MouseButton::Extra2),
        _ => None,
    }
}
