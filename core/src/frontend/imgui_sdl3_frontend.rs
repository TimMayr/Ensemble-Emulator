use std::cell::Ref;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError, bounded, unbounded};
use glow::HasContext as _;
use imgui::{Condition, TextureId};
use imgui_glow_renderer::AutoRenderer;
use log::error;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::video::GLProfile;

use crate::emulation::emu::{InputEvent, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::Frontend;

const FRAME_CHANNEL_CAPACITY: usize = 2;

pub struct ImguiSdl3Frontend {
    command_tx: Sender<UiCommand>,
    input_rx: Receiver<InputEvent>,
    thread_handle: Option<JoinHandle<()>>,
    error_rx: Receiver<String>,
}

impl Default for ImguiSdl3Frontend {
    fn default() -> Self { Self::new().expect("Failed to initialize imgui+sdl3 frontend") }
}

impl ImguiSdl3Frontend {
    pub fn new() -> Result<Self, String> {
        let (command_tx, command_rx) = bounded(FRAME_CHANNEL_CAPACITY);
        let (input_tx, input_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);

        let thread_handle = thread::Builder::new()
            .name("imgui_sdl3_frontend".into())
            .spawn(move || {
                if let Err(err) = ui_thread(command_rx, input_tx) {
                    let _ = error_tx.send(err);
                }
            })
            .map_err(|err| err.to_string())?;

        Ok(Self {
            command_tx,
            input_rx,
            thread_handle: Some(thread_handle),
            error_rx,
        })
    }

    fn check_thread_error(&self) -> Result<(), String> {
        match self.error_rx.try_recv() {
            Ok(err) => Err(err),
            Err(TryRecvError::Empty) => Ok(()),
            Err(TryRecvError::Disconnected) => Ok(()),
        }
    }
}

impl Drop for ImguiSdl3Frontend {
    fn drop(&mut self) {
        let _ = self.command_tx.send(UiCommand::Shutdown);
        if let Some(handle) = self.thread_handle.take() {
            if let Err(err) = handle.join() {
                error!("Failed to join frontend thread: {err:?}");
            }
        }
    }
}

impl Frontend for ImguiSdl3Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT) as usize]>,
    ) -> Result<(), String> {
        self.check_thread_error()?;

        let bytes: &[u8] = bytemuck::cast_slice(&*pixel_buffer);
        let frame = FrameData {
            pixels: bytes.to_vec(),
            width: TOTAL_OUTPUT_WIDTH,
            height: TOTAL_OUTPUT_HEIGHT,
        };

        match self.command_tx.try_send(UiCommand::Frame(frame)) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(_)) => Ok(()),
            Err(TrySendError::Disconnected(_)) => Err("Frontend thread disconnected".into()),
        }
    }

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        self.check_thread_error()?;

        let mut events = Vec::new();
        loop {
            match self.input_rx.try_recv() {
                Ok(event) => events.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    return Err("Frontend input channel disconnected".into());
                }
            }
        }

        Ok(events)
    }
}

enum UiCommand {
    Frame(FrameData),
    Shutdown,
}

struct FrameData {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

struct FrameTexture {
    texture: glow::NativeTexture,
    texture_id: TextureId,
    width: u32,
    height: u32,
}

impl FrameTexture {
    unsafe fn new(
        gl: &glow::Context,
        renderer: &mut AutoRenderer,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let texture = gl.create_texture().map_err(|err| err.to_string())?;
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
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            None,
        );
        gl.bind_texture(glow::TEXTURE_2D, None);

        let texture_id = renderer.textures().insert(imgui_glow_renderer::Texture {
            texture,
            target: glow::TEXTURE_2D,
        });

        Ok(Self {
            texture,
            texture_id,
            width,
            height,
        })
    }

    unsafe fn upload(&self, gl: &glow::Context, data: &[u8]) {
        gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        gl.tex_sub_image_2d(
            glow::TEXTURE_2D,
            0,
            0,
            0,
            self.width as i32,
            self.height as i32,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(data),
        );
        gl.bind_texture(glow::TEXTURE_2D, None);
    }
}

impl Drop for FrameTexture {
    fn drop(&mut self) {
        // Actual OpenGL resource cleanup is performed by the UI thread after the renderer is dropped.
    }
}

fn ui_thread(command_rx: Receiver<UiCommand>, input_tx: Sender<InputEvent>) -> Result<(), String> {
    let sdl = sdl3::init().map_err(|err| err.to_string())?;
    let video = sdl.video().map_err(|err| err.to_string())?;

    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_double_buffer(true);
    }

    let window = video
        .window("NES Emulator", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .map_err(|err| err.to_string())?;

    let gl_context = window.gl_create_context().map_err(|err| err.to_string())?;
    window
        .gl_make_current(&gl_context)
        .map_err(|err| err.to_string())?;

    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut renderer = AutoRenderer::initialize(gl.clone(), &mut imgui)
        .map_err(|err| format!("Failed to initialize imgui renderer: {err}"))?;

    let mut event_pump = sdl.event_pump().map_err(|err| err.to_string())?;

    let mut last_frame_time = Instant::now();
    let mut latest_frame: Option<FrameData> = None;
    let mut frame_texture: Option<FrameTexture> = None;

    'running: loop {
        loop {
            match command_rx.try_recv() {
                Ok(UiCommand::Frame(frame)) => {
                    latest_frame = Some(frame);
                }
                Ok(UiCommand::Shutdown) => break 'running,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break 'running,
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {
                    ..
                } => {
                    let _ = input_tx.send(InputEvent::Quit);
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    let _ = input_tx.send(InputEvent::Quit);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let _ = input_tx.send(InputEvent::IncPalette);
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let delta = now - last_frame_time;
        last_frame_time = now;
        imgui.io_mut().delta_time = delta.as_secs_f32().max(1.0 / 60.0);

        let window_size = window.size();
        imgui.io_mut().display_size = [window_size.0 as f32, window_size.1 as f32];

        if let Some(frame) = latest_frame.take() {
            unsafe {
                if frame_texture.is_none()
                    || frame_texture
                        .as_ref()
                        .map(|tex| tex.width != frame.width || tex.height != frame.height)
                        .unwrap_or(false)
                {
                    frame_texture = Some(FrameTexture::new(
                        &gl,
                        &mut renderer,
                        frame.width,
                        frame.height,
                    )?);
                }

                if let Some(texture) = &frame_texture {
                    texture.upload(&gl, &frame.pixels);
                }
            }
            latest_frame = Some(frame);
        }

        let ui = imgui.frame();

        if let Some(texture) = &frame_texture {
            imgui::Window::new("Emulator Output")
                .size(
                    [
                        (texture.width as f32).max(256.0),
                        (texture.height as f32).max(240.0),
                    ],
                    Condition::FirstUseEver,
                )
                .build(&ui, || {
                    let avail = ui.content_region_avail();
                    let aspect = texture.width as f32 / texture.height as f32;
                    let mut size = [avail[0], avail[0] / aspect];
                    if size[1] > avail[1] {
                        size[1] = avail[1];
                        size[0] = size[1] * aspect;
                    }
                    ui.image(texture.texture_id, size).build();
                });
        } else {
            imgui::Window::new("Emulator Output")
                .size([512.0, 480.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text("Waiting for first frame...");
                });
        }

        imgui::Window::new("Pattern Tables")
            .size([300.0, 300.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text("Pattern table visualization coming soon");
            });

        imgui::Window::new("Nametables")
            .size([300.0, 300.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text("Nametable visualization coming soon");
            });

        let draw_data = ui.render();
        unsafe {
            gl.viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        renderer
            .render(draw_data)
            .map_err(|err| format!("Failed to render imgui frame: {err}"))?;

        window.gl_swap_window();

        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
