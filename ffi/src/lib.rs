#![allow(clippy::missing_safety_doc)]

use std::ffi::CStr;
use std::os::raw::{c_char, c_float, c_int, c_void};

use nes_core::emulation::emu::{Console, Consoles, HEIGHT, WIDTH};
use nes_core::emulation::nes::{ExecutionFinishedType, Nes, MASTER_CYCLES_PER_FRAME};
use nes_core::frontend::godot_frontend::GodotFrontend;
use nes_core::frontend::Frontends;
use nes_core::frontend::Frontends::Godot;

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
    paused: bool,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_create() -> *mut FfiWrapper {
    let emu = FfiWrapper {
        console: Consoles::Nes(Nes::default()),
        godot_frontend: Godot(GodotFrontend::new()),
        paused: true,
    };
    Box::into_raw(Box::new(emu))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_destroy(e: *mut FfiWrapper) {
    if !e.is_null() {
        unsafe {
            let emu = &mut *e;
            emu.console.flush_trace_log();

            drop(Box::from_raw(e));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_enable_trace_log(e: *mut FfiWrapper) {
    if e.is_null() {
        return;
    }

    let emu = unsafe { &mut *e };
    emu.console
        .set_trace_log_path(Some("./trace_log.log".into()));
    println!("[EmuBackend] Trace log has been enabled")
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

    if let Godot(ref mut godot_frontend) = emu.godot_frontend {
        godot_frontend.set_buffer(ptr, len);
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

    if emu.paused {
        return 0;
    }

    let res = emu.console.step_frame(&mut emu.godot_frontend);
    let Consoles::Nes(nes) = &emu.console;

    match res {
        Ok(res) => match res {
            ExecutionFinishedType::ReachedLastCycle => {
                println!(
                    "[EmuBackend] {:?} after {}",
                    res,
                    nes.cycles / MASTER_CYCLES_PER_FRAME as u128
                );
                0
            }
            ExecutionFinishedType::ReachedHlt => {
                println!("[EmuBackend] Encountered hlt");
                let mem = &nes.get_memory_debug(Some(0x6000..=0x6100))[0];

                for (i, n) in mem.iter().enumerate() {
                    if i % 32 == 0 {
                        if i > 0 {
                            println!();
                        }
                        print!("    ");
                    }
                    print!("{:02X}, ", n);
                }
                println!();
                -1
            }
            ExecutionFinishedType::CycleCompleted | ExecutionFinishedType::NotStepped => {
                unreachable!()
            }
        },
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_load_rom(e: *mut FfiWrapper, path: *const c_char) -> c_int {
    if e.is_null() || path.is_null() {
        return -1;
    }

    let emu = unsafe { &mut *e };
    let c_str = unsafe { CStr::from_ptr(path) };
    println!("[EmuBackend] Loading rom from: {:?}", c_str);
    let Ok(path_str) = c_str.to_str() else {
        return -1;
    };
    let path_string = path_str.to_owned();
    emu.console.load_rom(&path_string);

    let Consoles::Nes(nes) = &emu.console;

    println!("{:?}", nes.rom_file);
    println!("{:04X?}", nes.get_memory_debug(Some(0xFFFE..=0xFFFF))[0]);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_set_paused(e: *mut FfiWrapper, paused: c_int) -> c_int {
    if e.is_null() {
        return -1;
    }

    let emu = unsafe { &mut *e };
    emu.paused = paused != 0;
    println!("[EmuBackend] Emulator is paused: {}", emu.paused);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emu_reset(e: *mut FfiWrapper) {
    if e.is_null() {
        return;
    }

    let emu = unsafe { &mut *e };
    emu.console.reset();
    println!("[EmuBackend] Emulator has been reset")
}
