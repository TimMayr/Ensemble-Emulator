// ffi/build.rs
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=src/");

    let out_dir = PathBuf::from("../godot_frontend/bin");
    fs::create_dir_all(&out_dir).unwrap();

    let lib_name = if cfg!(target_os = "windows") {
        "nes_ffi.dll"
    } else if cfg!(target_os = "macos") {
        "libnes_ffi.dylib"
    } else {
        "libnes_ffi.so"
    };

    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .ancestors()
        .nth(3)
        .unwrap()
        .join(lib_name);

    println!("Copying from {target_dir:?} to {out_dir:?}");
    match fs::copy(&target_dir, out_dir.join(lib_name)) {
        Ok(_) => println!("cargo:warning=Library copied to Godot project"),
        Err(e) => println!("cargo:warning=Failed to copy library: {e}"),
    }
}
