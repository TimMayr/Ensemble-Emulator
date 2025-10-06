use std::os::raw::{c_float, c_int, c_void};

use crate::emulation::emu::{Console, Consoles, HEIGHT, WIDTH};
use crate::emulation::nes::Nes;
use crate::frontend::godot_frontend::GodotFrontend;
use crate::frontend::Frontends;
use crate::frontend::Frontends::Godot;

#[repr(C)]
pub struct EmuVidSpecs {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub format: c_int,
    pub pixel_aspect_x: c_int,
    pub pixel_aspect_y: c_int,
}

#[repr(C)]
pub struct EmuAudioSpecs {
    pub sample_rate: c_int,
    pub channels: c_int,
    pub suggested_frames: c_int,
}

#[repr(C)]
pub struct EmuInput {
    pub buttons: u32,
    pub analog0_x: c_float, // optional
    pub analog0_y: c_float,
}

#[repr(C)]
pub struct FfiWrapper {
    console: Consoles,
    godot_frontend: Frontends,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_create() -> *mut FfiWrapper {
    let emu = FfiWrapper {
        console: Consoles::Nes(Nes::default()),
        godot_frontend: Frontends::Godot(GodotFrontend::new()),
    };
    Box::into_raw(Box::new(emu))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_destroy(e: *mut FfiWrapper) {
    if !e.is_null() {
        unsafe {
            drop(Box::from_raw(e));
        }
    }
}

// Specs negotiated here; can be called any time after ROM load.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_get_video_spec(e: *mut FfiWrapper, out: *mut EmuVidSpecs) -> c_int {
    if e.is_null() || out.is_null() {
        return -1;
    }

    let _emu = unsafe { &mut *e };
    let spec = EmuVidSpecs {
        width: WIDTH as c_int,
        height: HEIGHT as c_int,
        stride: (WIDTH * 4u32) as c_int,
        format: 0 as c_int,
        pixel_aspect_x: 1 as c_int,
        pixel_aspect_y: 1 as c_int,
    };

    unsafe {
        *out = spec;
    }
    0
}

// Provide a pointer to a C#-pinned buffer that Rust writes into each frame.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_set_video_buffer(
    e: *mut FfiWrapper,
    ptr: *mut c_void,
    len: usize,
) -> c_int {
    if e.is_null() || ptr.is_null() || len == 0 {
        return -1;
    }

    let emu = unsafe { &mut *e };

    if let Godot(ref mut godotFrontend) = emu.godot_frontend {
        godotFrontend.set_buffer(ptr, len);
    }

    0
}

// Godot thread drives the emu: run for one frame worth of emu time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_step_frame(e: *mut FfiWrapper) -> c_int {
    if e.is_null() {
        return -1;
    }

    let emu = unsafe { &mut *e };
    match emu.console.step(&mut emu.godot_frontend) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
