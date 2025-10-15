use std::cell::Ref;

use sdl2::EventPump;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{ScaleMode, TextureCreator, UpdateTextureError, WindowCanvas};
use sdl2::video::WindowContext;

use crate::emulation::emu::{HEIGHT, InputEvent, WIDTH};
use crate::frontend::Frontend;

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
        let mut texture = self
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA8888, WIDTH, HEIGHT)
            .expect("Error creating Texture");
        texture.set_scale_mode(ScaleMode::Nearest);

        let bytes: &[u8] = bytemuck::cast_slice(&*pixel_buffer);
        texture
            .update(None, bytes, (WIDTH * 4) as usize)
            .map_err(|e: UpdateTextureError| e.to_string())?;

        // Render
        self.canvas.clear();
        self.canvas.copy(&texture, None, None)?;
        self.canvas.present();

        Ok(())
    }

    fn poll_input_events(&mut self) -> Result<Vec<InputEvent>, String> {
        let mut events = Vec::new();

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
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    events.push(InputEvent::IncPalette);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(..) | WindowEvent::SizeChanged(..),
                    ..
                } => self.canvas.set_logical_size(WIDTH, HEIGHT).expect(""),
                _ => {}
            }
        }

        Ok(events)
    }
}
