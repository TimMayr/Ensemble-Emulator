use std::cell::Ref;
use std::fmt::{Debug, Formatter};
use std::mem;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, UpdateTextureError, WindowCanvas};

use crate::emulation::emu::InputEvent;
use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::Frontend;

pub struct SdlFrontend {
    texture: Texture<'static>,
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
            .window("NES Emulator", TOTAL_OUTPUT_WIDTH as u32, TOTAL_OUTPUT_HEIGHT as u32)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .expect("Error initializing windows");

        // Create canvas
        let mut canvas = window
            .into_canvas()
            .build()
            .expect("Error initializing canvas");
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .set_logical_size(TOTAL_OUTPUT_WIDTH as u32, TOTAL_OUTPUT_HEIGHT as u32)
            .expect("Error setting canvas size");
        canvas
            .set_integer_scale(true)
            .expect("Error forcing integer scaling");

        // Create texture creator
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                TOTAL_OUTPUT_WIDTH as u32,
                TOTAL_OUTPUT_HEIGHT as u32,
            )
            .expect("Error creating Texture");

        // SAFETY: The texture only lives as long as the frontend instance and is dropped
        // before the stored texture creator. Extending the lifetime to 'static is safe
        // because both values share the same actual lifetime and are dropped together.
        let texture: Texture<'static> = unsafe { mem::transmute(texture) };

        let event_pump = context.event_pump().expect("Error creating event pump");

        Self {
            texture,
            event_pump,
            canvas,
        }
    }
}

impl Frontend for SdlFrontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; TOTAL_OUTPUT_WIDTH * TOTAL_OUTPUT_HEIGHT]>,
    ) -> Result<(), String> {
        let bytes: &[u8] = bytemuck::cast_slice(&*pixel_buffer);
        self.texture
            .update(None, bytes, TOTAL_OUTPUT_WIDTH * 4)
            .map_err(|e: UpdateTextureError| e.to_string())?;

        // Render
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None)?;
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
                } => events.push(InputEvent::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    events.push(InputEvent::IncPalette);
                }
                _ => {}
            }
        }

        Ok(events)
    }
}

impl Debug for SdlFrontend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("f") }
}
