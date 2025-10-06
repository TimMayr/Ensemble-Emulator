#[allow(clippy::missing_safety_doc)]
pub mod ffi_wrapper;
pub mod godot_frontend;

use std::cell::Ref;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ScaleMode, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::EventPump;

use crate::emulation::emu::{HEIGHT, WIDTH};
use crate::frontend::godot_frontend::GodotFrontend;

pub enum Frontends {
    Sdl2(SdlFrontend),
    Godot(GodotFrontend),
    None(),
}

impl Default for Frontends {
    fn default() -> Self { Frontends::None() }
}

impl Frontend for Frontends {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>,
    ) -> Result<(), String> {
        match self {
            Frontends::Sdl2(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::Godot(frontend) => frontend.show_frame(pixel_buffer),
            Frontends::None() => Ok(()),
        }
    }
}

pub trait Frontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>,
    ) -> Result<(), String>;
}

pub struct SdlFrontend {
    texture_creator: TextureCreator<WindowContext>,
    event_pump: EventPump,
    canvas: WindowCanvas,
}

impl Default for SdlFrontend {
    fn default() -> Self {
        // Initialize SDL2
        let context = sdl2::init().expect("Error initializing SDL");
        let video_subsystem = context.video().expect("Error initializing Video System");

        // Create window
        let window = video_subsystem
            .window("NES Emulator", WIDTH * 3, HEIGHT * 3) // scale x3
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .expect("Error initializing windows");

        // Create canvas
        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Error initializing canvas");

        // Create texture creator
        let texture_creator = canvas.texture_creator();

        let event_pump = context.event_pump().expect("Error creating event pump");

        Self {
            texture_creator,
            event_pump,
            canvas,
        }
    }
}

impl Frontend for SdlFrontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>,
    ) -> Result<(), String> {
        // Handle events
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Err(String::from("Quit Program"));
                }
                Event::Window {
                    win_event: WindowEvent::Resized(..) | WindowEvent::SizeChanged(..),
                    ..
                } => self.canvas.set_logical_size(WIDTH, HEIGHT).expect(""),
                _ => {}
            }
        }

        let mut texture = self
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA8888, WIDTH, HEIGHT)
            .expect("Error creating Texture");
        texture.set_scale_mode(ScaleMode::Nearest);

        let bytes: &[u8] = bytemuck::cast_slice(&*pixel_buffer);
        texture
            .update(None, bytes, (WIDTH * 4) as usize)
            .map_err(|e| e.to_string())?;

        // Render
        self.canvas.clear();
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }
}
