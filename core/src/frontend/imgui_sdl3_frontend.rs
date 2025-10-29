use std::cell::Ref;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError, bounded, unbounded};
use glow::HasContext;
use imgui::{Condition, Context, TextureId};
use imgui_opengl_renderer::{Renderer as ImguiRenderer, Texture as ImguiTexture};
use imgui_sdl3::ImguiSdl3;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Scancode;
use sdl3::video::{GLProfile, SwapInterval};

use crate::emulation::emu::{InputEvent, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::Frontend;

const DEFAULT_WINDOW_WIDTH: u32 = 1280;
const DEFAULT_WINDOW_HEIGHT: u32 = 720;

pub struct ImguiSdl3Frontend {
    frame_sender: Sender<FrameMessage>,
    input_receiver: Receiver<InputEvent>,
    control_sender: Sender<UiThreadCommand>,
    join_handle: Option<JoinHandle<()>>,
}

impl Default for ImguiSdl3Frontend {
    fn default() -> Self { Self::new().expect("Failed to start ImGui SDL3 frontend") }
}

impl ImguiSdl3Frontend {
    pub fn new() -> Result<Self, String> {
        let (frame_sender, frame_receiver) = bounded::<FrameMessage>(3);
        let (input_sender, input_receiver) = unbounded::<InputEvent>();
        let (control_sender, control_receiver) = bounded::<UiThreadCommand>(1);

        let join_handle = thread::Builder::new()
            .name("imgui-sdl3-frontend".into())
            .spawn(move || {
                if let Err(err) = run_ui(frame_receiver, input_sender, control_receiver) {
                    log::error!("Imgui SDL3 frontend crashed: {err}");
                }
            })
            .map_err(|err| err.to_string())?;

        Ok(Self {
            frame_sender,
            input_receiver,
            control_sender,
            join_handle: Some(join_handle),
        })
    }
}

impl Frontend for ImguiSdl3Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
    ) -> Result<(), String> {
        let mut owned_frame = Vec::with_capacity(pixel_buffer.len());
        owned_frame.extend(pixel_buffer.iter().copied());

        match self.frame_sender.try_send(FrameMessage::Frame(owned_frame)) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Ok(()),
            Err(TrySendError::Disconnected(_)) => Err("Frontend UI thread disconnected".into()),
        }
    }

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        let mut events = Vec::new();
        loop {
            match self.input_receiver.try_recv() {
                Ok(event) => events.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return Err("UI thread disconnected".into()),
            }
        }
        Ok(events)
    }
}

impl Drop for ImguiSdl3Frontend {
    fn drop(&mut self) {
        let _ = self.control_sender.send(UiThreadCommand::Shutdown);
        if let Some(handle) = self.join_handle.take() {
            let _ = handle.join();
        }
    }
}

enum FrameMessage {
    Frame(Vec<u32>),
}

enum UiThreadCommand {
    Shutdown,
}

fn run_ui(
    frame_receiver: Receiver<FrameMessage>,
    input_sender: Sender<InputEvent>,
    control_receiver: Receiver<UiThreadCommand>,
) -> Result<(), String> {
    let sdl = sdl3::init().map_err(stringify_err)?;
    let video = sdl.video().map_err(stringify_err)?;

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);

    let window = video
        .window("NES Emulator", DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .map_err(stringify_err)?;

    let gl_context = window.gl_create_context().map_err(stringify_err)?;
    window.gl_make_current(&gl_context).map_err(stringify_err)?;
    video
        .gl_set_swap_interval(SwapInterval::Immediate)
        .map_err(stringify_err)?;

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    let mut platform = ImguiSdl3::new(&mut imgui, &window);
    let gl = Arc::new(unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    });

    let mut renderer = ImguiRenderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _)
        .map_err(|err| format!("Failed to initialize ImGui renderer: {err}"))?;

    let emulator_texture = unsafe { create_emulator_texture(gl.clone())? };
    let emulator_texture_id = renderer.textures().insert(ImguiTexture {
        texture_id: emulator_texture,
        target: glow::TEXTURE_2D,
    });

    let mut last_frame = Vec::<u32>::new();
    let mut event_pump = sdl.event_pump().map_err(stringify_err)?;

    'running: loop {
        match control_receiver.try_recv() {
            Ok(UiThreadCommand::Shutdown) | Err(TryRecvError::Disconnected) => break 'running,
            Err(TryRecvError::Empty) => {}
        }

        // Poll SDL events and forward to ImGui and emulator
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);
            match translate_event(&event) {
                Some(frontend_event) => {
                    let _ = input_sender.send(frontend_event);
                }
                None => {}
            }
        }

        // Drain latest frame
        let mut new_frame = None;
        loop {
            match frame_receiver.try_recv() {
                Ok(FrameMessage::Frame(frame)) => new_frame = Some(frame),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break 'running,
            }
        }

        if let Some(frame) = new_frame {
            last_frame = frame;
            unsafe {
                update_emulator_texture(gl.clone(), emulator_texture, &last_frame)?;
            }
        }

        platform
            .prepare_frame(&mut imgui, &window, &event_pump)
            .map_err(|err| format!("Failed to prepare ImGui frame: {err}"))?;
        let ui = imgui.frame();

        draw_main_windows(&ui, emulator_texture_id);

        platform.prepare_render(&ui, &window);
        renderer
            .render(ui)
            .map_err(|err| format!("Failed to render ImGui: {err}"))?;
        window.gl_swap_window();
    }

    if let Some(texture) = renderer.textures().remove(emulator_texture_id) {
        unsafe {
            gl.delete_texture(texture.texture_id);
        }
    }

    Ok(())
}

fn draw_main_windows(ui: &imgui::Ui, emulator_texture: TextureId) {
    imgui::Window::new("NES Output")
        .size(
            [
                TOTAL_OUTPUT_WIDTH as f32 * 2.0,
                TOTAL_OUTPUT_HEIGHT as f32 * 2.0,
            ],
            Condition::FirstUseEver,
        )
        .build(ui, |ui| {
            let size = [TOTAL_OUTPUT_WIDTH as f32, TOTAL_OUTPUT_HEIGHT as f32];
            imgui::Image::new(emulator_texture, size).build(ui);
        });

    imgui::Window::new("Pattern Tables")
        .size([400.0, 400.0], Condition::FirstUseEver)
        .build(ui, |ui| {
            ui.text("Pattern table viewer not yet implemented");
        });

    imgui::Window::new("Nametable")
        .size([400.0, 400.0], Condition::FirstUseEver)
        .build(ui, |ui| {
            ui.text("Nametable viewer not yet implemented");
        });
}

fn translate_event(event: &Event) -> Option<InputEvent> {
    match event {
        Event::Quit {
            ..
        } => Some(InputEvent::Quit),
        Event::Window {
            win_event: WindowEvent::Close,
            ..
        } => Some(InputEvent::Quit),
        Event::KeyDown {
            scancode: Some(Scancode::Space),
            ..
        } => Some(InputEvent::IncPalette),
        _ => None,
    }
}

unsafe fn create_emulator_texture(gl: Arc<glow::Context>) -> Result<glow::NativeTexture, String> {
    let texture = gl.create_texture().map_err(stringify_err)?;
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::LINEAR as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::LINEAR as i32,
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
    Ok(texture)
}

unsafe fn update_emulator_texture(
    gl: Arc<glow::Context>,
    texture: glow::NativeTexture,
    frame: &[u32],
) -> Result<(), String> {
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    let bytes: &[u8] = bytemuck::cast_slice(frame);
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
    Ok(())
}

fn stringify_err<E: std::fmt::Display>(err: E) -> String { err.to_string() }
