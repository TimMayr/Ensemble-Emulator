use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("../godot_frontend/bin");
    fs::create_dir_all(&out_dir).unwrap();

    let lib_name = if cfg!(target_os = "windows") {
        "nes_ffi.dll"
    } else if cfg!(target_os = "macos") {
        "libnes_ffi.dylib"
    } else {
        "libnes_ffi.so"
    };

    println!("cargo:rerun-if-changed=src/");
    let target_dir = std::env::var("OUT_DIR").unwrap();
    let built_lib = PathBuf::from(&target_dir)
        .ancestors()
        .nth(3) // up to target/release
        .unwrap()
        .join(lib_name);

    if let Ok(_) = fs::copy(&built_lib, out_dir.join(lib_name)) {
        println!("Library copied to Godot project");
    }
}
