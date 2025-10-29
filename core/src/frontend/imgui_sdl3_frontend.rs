use std::cell::Ref;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender, unbounded};
use glow::HasContext as _;
use imgui::{Condition, ConfigFlags, Context as ImguiContext, TextureId, Ui, Window};
use imgui_opengl_renderer::{Renderer, Texture};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::mouse::MouseButton;

use crate::emulation::emu::{InputEvent, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::Frontend;

const FRAMEBUFFER_SIZE: usize = (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize;
const WINDOW_TITLE: &str = "NES Emulator (ImGui SDL3)";

#[derive(Debug)]
struct SharedFrameBuffer {
    pixels: Vec<u32>,
    dirty: bool,
}

impl SharedFrameBuffer {
    fn new() -> Self {
        Self {
            pixels: vec![0; FRAMEBUFFER_SIZE],
            dirty: false,
        }
    }
}

#[derive(Debug)]
enum ControlMessage {
    Shutdown,
}

pub struct ImguiSdl3Frontend {
    frame_buffer: Arc<Mutex<SharedFrameBuffer>>,
    input_rx: Receiver<InputEvent>,
    control_tx: Sender<ControlMessage>,
    ui_thread: Option<JoinHandle<Result<(), String>>>,
}

impl ImguiSdl3Frontend {
    pub fn new() -> Result<Self, String> {
        let frame_buffer = Arc::new(Mutex::new(SharedFrameBuffer::new()));
        let (input_tx, input_rx) = unbounded();
        let (control_tx, control_rx) = unbounded();
        let frame_buffer_clone = Arc::clone(&frame_buffer);

        let ui_thread = thread::Builder::new()
            .name("imgui-sdl3-ui".into())
            .spawn(move || run_ui(frame_buffer_clone, input_tx, control_rx))
            .map_err(|err| err.to_string())?;

        Ok(Self {
            frame_buffer,
            input_rx,
            control_tx,
            ui_thread: Some(ui_thread),
        })
    }
}

impl Default for ImguiSdl3Frontend {
    fn default() -> Self { Self::new().expect("Failed to initialize ImGui SDL3 frontend") }
}

impl Drop for ImguiSdl3Frontend {
    fn drop(&mut self) {
        if let Some(thread) = self.ui_thread.take() {
            let _ = self.control_tx.send(ControlMessage::Shutdown);
            if let Err(err) = thread.join() {
                eprintln!("Failed to join UI thread: {err:?}");
            }
        }
    }
}

impl Frontend for ImguiSdl3Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
    ) -> Result<(), String> {
        let mut shared = self
            .frame_buffer
            .lock()
            .map_err(|_| String::from("Failed to lock frame buffer"))?;
        shared.pixels.copy_from_slice(&*pixel_buffer);
        shared.dirty = true;
        Ok(())
    }

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        let mut events = Vec::new();
        while let Ok(event) = self.input_rx.try_recv() {
            events.push(event);
        }
        Ok(events)
    }
}

fn run_ui(
    frame_buffer: Arc<Mutex<SharedFrameBuffer>>,
    input_tx: Sender<InputEvent>,
    control_rx: Receiver<ControlMessage>,
) -> Result<(), String> {
    let sdl = sdl3::init().map_err(|err| err.to_string())?;
    let video = sdl.video().map_err(|err| err.to_string())?;

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let mut window = video
        .window(
            WINDOW_TITLE,
            (TOTAL_OUTPUT_WIDTH * 3) as u32,
            (TOTAL_OUTPUT_HEIGHT * 3) as u32,
        )
        .resizable()
        .opengl()
        .build()
        .map_err(|err| err.to_string())?;

    let gl_context = window.gl_create_context().map_err(|err| err.to_string())?;
    window
        .gl_make_current(&gl_context)
        .map_err(|err| err.to_string())?;

    let gl = unsafe {
        glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
    };

    let mut imgui = ImguiContext::create();
    imgui.set_ini_filename(None);
    {
        let io = imgui.io_mut();
        io.config_flags |= ConfigFlags::DOCKING_ENABLE;
        io.font_global_scale = 1.0;
    }

    let mut renderer = Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as *const _)
        .map_err(|err| err.to_string())?;

    let texture = unsafe {
        let tex = gl
            .create_texture()
            .map_err(|err| format!("Failed to create texture: {err}"))?;
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));
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
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA8 as i32,
            TOTAL_OUTPUT_WIDTH as i32,
            TOTAL_OUTPUT_HEIGHT as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            None,
        );
        gl.bind_texture(glow::TEXTURE_2D, None);
        tex
    };

    let texture_id = renderer.textures().insert(Texture {
        texture_id: texture,
        size: [TOTAL_OUTPUT_WIDTH as f32, TOTAL_OUTPUT_HEIGHT as f32],
    });

    let mut event_pump = sdl.event_pump().map_err(|err| err.to_string())?;
    let mut last_frame = Instant::now();
    let mut running = true;

    while running {
        while let Ok(message) = control_rx.try_recv() {
            if matches!(message, ControlMessage::Shutdown) {
                running = false;
            }
        }

        for event in event_pump.poll_iter() {
            handle_sdl_event(&mut imgui, &input_tx, &mut running, &event)?;
        }

        if !running {
            break;
        }

        let now = Instant::now();
        let delta = now - last_frame;
        last_frame = now;
        let io = imgui.io_mut();
        io.delta_time = delta.as_secs_f32().max(1.0 / 60.0);

        let window_size = window.size();
        io.display_size = [window_size.0 as f32, window_size.1 as f32];

        update_texture_if_needed(&gl, &frame_buffer, texture)?;

        unsafe {
            gl.viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
            gl.clear_color(0.05, 0.05, 0.05, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        let ui = imgui.frame();
        build_ui(&ui, texture_id);
        renderer
            .render(ui.render())
            .map_err(|err| err.to_string())?;

        window.gl_swap_window();
    }

    unsafe {
        let _ = renderer.textures().remove(texture_id);
        gl.delete_texture(texture);
    }

    Ok(())
}

fn handle_sdl_event(
    imgui: &mut ImguiContext,
    input_tx: &Sender<InputEvent>,
    running: &mut bool,
    event: &Event,
) -> Result<(), String> {
    match event {
        Event::Quit {
            ..
        } => {
            *running = false;
            let _ = input_tx.send(InputEvent::Quit);
        }
        Event::KeyDown {
            keycode: Some(Keycode::Escape),
            repeat: false,
            ..
        } => {
            *running = false;
            let _ = input_tx.send(InputEvent::Quit);
        }
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            repeat: false,
            ..
        } => {
            let _ = input_tx.send(InputEvent::IncPalette);
        }
        _ => {}
    }

    update_imgui_input(imgui, event);
    Ok(())
}

fn update_imgui_input(imgui: &mut ImguiContext, event: &Event) {
    let io = imgui.io_mut();
    match event {
        Event::MouseMotion {
            x,
            y,
            ..
        } => {
            io.add_mouse_pos_event(*x as f32, *y as f32);
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
            precise_x,
            precise_y,
            ..
        } => {
            io.add_mouse_wheel_event(*precise_x, *precise_y);
        }
        Event::KeyDown {
            keycode: Some(key),
            ..
        } => {
            if let Some(imgui_key) = map_keycode(*key) {
                io.add_key_event(imgui_key, true);
            }
        }
        Event::KeyUp {
            keycode: Some(key),
            ..
        } => {
            if let Some(imgui_key) = map_keycode(*key) {
                io.add_key_event(imgui_key, false);
            }
        }
        Event::TextInput {
            text, ..
        } => {
            for c in text.chars() {
                io.add_input_character(c);
            }
        }
        _ => {}
    }
}

fn map_keycode(key: Keycode) -> Option<imgui::Key> {
    use imgui::Key;

    Some(match key {
        Keycode::Tab => Key::Tab,
        Keycode::Left => Key::LeftArrow,
        Keycode::Right => Key::RightArrow,
        Keycode::Up => Key::UpArrow,
        Keycode::Down => Key::DownArrow,
        Keycode::PageUp => Key::PageUp,
        Keycode::PageDown => Key::PageDown,
        Keycode::Home => Key::Home,
        Keycode::End => Key::End,
        Keycode::Insert => Key::Insert,
        Keycode::Delete => Key::Delete,
        Keycode::Backspace => Key::Backspace,
        Keycode::Space => Key::Space,
        Keycode::Return => Key::Enter,
        Keycode::Escape => Key::Escape,
        Keycode::A => Key::A,
        Keycode::C => Key::C,
        Keycode::V => Key::V,
        Keycode::X => Key::X,
        Keycode::Y => Key::Y,
        Keycode::Z => Key::Z,
        _ => return None,
    })
}

fn map_mouse_button(button: MouseButton) -> Option<imgui::MouseButton> {
    Some(match button {
        MouseButton::Left => imgui::MouseButton::Left,
        MouseButton::Right => imgui::MouseButton::Right,
        MouseButton::Middle => imgui::MouseButton::Middle,
        MouseButton::X1 => imgui::MouseButton::Other(3),
        MouseButton::X2 => imgui::MouseButton::Other(4),
        _ => return None,
    })
}

fn update_texture_if_needed(
    gl: &glow::Context,
    frame_buffer: &Arc<Mutex<SharedFrameBuffer>>,
    texture: glow::NativeTexture,
) -> Result<(), String> {
    let mut guard = frame_buffer
        .lock()
        .map_err(|_| String::from("Failed to lock frame buffer"))?;
    if !guard.dirty {
        return Ok(());
    }

    let bytes: &[u8] = bytemuck::cast_slice(&guard.pixels);

    unsafe {
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_sub_image_2d(
            glow::TEXTURE_2D,
            0,
            0,
            0,
            TOTAL_OUTPUT_WIDTH as i32,
            TOTAL_OUTPUT_HEIGHT as i32,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(bytes),
        );
        gl.bind_texture(glow::TEXTURE_2D, None);
    }

    guard.dirty = false;
    Ok(())
}

fn build_ui(ui: &Ui, texture_id: TextureId) {
    Window::new("Emulator Output")
        .size([640.0, 480.0], Condition::FirstUseEver)
        .build(ui, || {
            imgui::Image::new(
                texture_id,
                [TOTAL_OUTPUT_WIDTH as f32, TOTAL_OUTPUT_HEIGHT as f32],
            )
            .build(ui);
        });

    Window::new("Pattern Tables")
        .size([400.0, 300.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text("Pattern table inspector coming soon");
        });

    Window::new("Nametables")
        .size([400.0, 300.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text("Nametable viewer placeholder");
        });
}
