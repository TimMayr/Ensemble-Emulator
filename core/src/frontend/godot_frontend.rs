use std::cell::Ref;
use std::ffi::c_void;

use crate::emulation::emu::{HEIGHT, WIDTH};
use crate::frontend::Frontend;

pub struct GodotFrontend {
    pub video_ptr: *mut c_void,
    pub video_len: usize,
}

impl Default for GodotFrontend {
    fn default() -> Self { Self::new() }
}

impl GodotFrontend {
    pub fn new() -> Self {
        Self {
            video_ptr: std::ptr::null_mut(),
            video_len: 0,
        }
    }

    pub fn set_buffer(&mut self, ptr: *mut c_void, len: usize) {
        self.video_ptr = ptr;
        self.video_len = len;
    }
}

impl Frontend for GodotFrontend {
    fn show_frame(
        &mut self,
        pixel_buffer: Ref<'_, [u32; (WIDTH * HEIGHT) as usize]>,
    ) -> Result<(), String> {
        if self.video_ptr.is_null() || self.video_len == 0 {
            return Ok(());
        }

        let src_bytes: &[u8] = bytemuck::cast_slice(&*pixel_buffer);
        let expected_len = src_bytes.len();

        if expected_len > self.video_len {
            return Err("Godot video buffer too small".into());
        }

        unsafe {
            let dst = std::slice::from_raw_parts_mut(self.video_ptr as *mut u8, self.video_len);

            for (i, &pix) in pixel_buffer.iter().enumerate() {
                // Extract color components from the u32 (assuming 0xAARRGGBB)
                let r = ((pix >> 24) & 0xFF) as u8;
                let g = ((pix >> 16) & 0xFF) as u8;
                let b = ((pix >> 8) & 0xFF) as u8;
                let a = (pix & 0xFF) as u8;

                let base = i * 4;
                if base + 3 >= dst.len() {
                    break;
                }

                dst[base] = r;
                dst[base + 1] = g;
                dst[base + 2] = b;
                dst[base + 3] = a;
            }
        }

        Ok(())
    }
}
