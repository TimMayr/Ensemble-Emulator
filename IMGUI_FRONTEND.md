# ImGui-SDL3 Frontend

This document describes the ImGui-based frontend for the NES emulator.

## Architecture

The frontend follows a Model-View-Controller (MVC) pattern with message-based communication:

- **Model**: The emulator core (`Consoles::Nes`)
- **View**: ImGui UI windows and rendering
- **Controller**: `ChannelEmulator` wrapper that mediates between model and view

### Components

1. **ChannelEmulator** (`core/src/emulation/channel_emu.rs`)
   - Wraps the emulator console
   - Provides channel-based communication
   - Handles message passing between frontend and emulator
   - Manages emulation state (paused, running, etc.)

2. **ImGuiFrontend** (`core/src/frontend/imgui_frontend.rs`)
   - Main UI implementation using ImGui and SDL3
   - Manages multiple windows/views
   - Handles user input and events
   - Displays emulator output and debug information

3. **Messages** (`core/src/emulation/messages.rs`)
   - `FrontendMessage`: Commands from frontend to emulator (Quit, Pause, Resume, etc.)
   - `EmulatorMessage`: Notifications from emulator to frontend (FrameReady, Stopped)
   - `ControllerEvent`: Input events (controller buttons, etc.)

### Message Flow

```
User Input → ImGuiFrontend → FrontendMessage → ChannelEmulator → Emulator
                    ↑                                ↓
                    └────── EmulatorMessage ─────────┘
```

## Features

### Current Features

- **Emulator Output Window**: Displays the NES screen output
- **Pattern Table Viewer**: Placeholder window for viewing CHR ROM patterns (TODO: implement visualization)
- **Nametable Viewer**: Placeholder window for viewing nametable data (TODO: implement visualization)
- **FPS Counter**: Real-time frame rate display
- **Status Bar**: Shows emulator status and statistics
- **Menu System**: View menu for toggling debug windows

### Planned Features

- Full Pattern Table visualization
- Complete Nametable viewer with scrolling
- Memory viewer
- CPU state viewer
- APU state viewer
- Sprite viewer
- Palette viewer
- Breakpoints and debugging
- Save states
- Input remapping

## Threading Model

### Current Implementation

The current implementation uses a **channel-based architecture** but runs in a **single thread**. The emulator and frontend communicate via crossbeam channels, but both execute in the main thread.

**Advantages:**
- Clean separation of concerns
- Easy to understand and debug
- No synchronization issues
- Predictable frame pacing

**Limitations:**
- UI responsiveness tied to emulation speed
- Cannot speed up/slow down emulation independently
- If emulation lags, UI becomes unresponsive

### Future: True Multi-Threading

To implement true multi-threading, the following changes are needed:

1. **Replace `Rc<RefCell<T>>` with `Arc<Mutex<T>>`** in the emulator core
   - Current: `Rc<RefCell<Ppu>>` in `Nes` struct
   - Required: `Arc<Mutex<Ppu>>` for thread-safe sharing
   
2. **Use `ThreadedEmulator` wrapper** (see `core/src/emulation/threaded.rs`)
   - Already implemented but disabled due to thread safety issues
   - Will allow emulator to run in a separate thread
   - UI remains responsive even if emulation is slow
   - Enables speed control independent of UI

## Building and Running

### Prerequisites

- Rust nightly toolchain
- SDL3 development libraries
- X11 or Wayland development libraries (Linux)
- Vulkan or SPIR-V support for GPU rendering

### Build

```bash
cargo build --features imgui-frontend --release
```

### Run

```bash
cargo run --features imgui-frontend --bin nes_main
```

## Code Structure

```
core/
├── src/
│   ├── app/
│   │   └── mod.rs              # Main entry point for imgui-frontend
│   ├── emulation/
│   │   ├── channel_emu.rs      # Channel-based emulator wrapper
│   │   ├── messages.rs         # Message types for communication
│   │   └── threaded.rs         # Future: threaded emulator (disabled)
│   └── frontend/
│       └── imgui_frontend.rs   # ImGui UI implementation
```

## Extending the Frontend

### Adding a New Window

1. Add a boolean field to `ImGuiFrontend` for window visibility
2. Add a menu item to toggle the window
3. Implement the window rendering in a separate function
4. Call the function from `render_ui_static` when the boolean is true

Example:
```rust
// In ImGuiFrontend struct
show_my_window: bool,

// In render_ui_static
if *show_my_window {
    ui.window("My Window")
        .size([400.0, 300.0], imgui::Condition::FirstUseEver)
        .opened(show_my_window)
        .build(|| {
            ui.text("My window content");
        });
}
```

### Adding New Messages

1. Add the message variant to `FrontendMessage` or `EmulatorMessage` in `messages.rs`
2. Handle the message in `ChannelEmulator::step_frame()` or `ImGuiFrontend::run()`
3. Send the message from the appropriate component

### Getting Emulator Data

To display emulator internal state in the UI:

1. Add a new `EmulatorMessage` variant with the data
2. Send the data from `ChannelEmulator` when available
3. Store the data in `ImGuiFrontend` when received
4. Display it in the appropriate window

## Performance Considerations

- The current single-threaded implementation aims for 60 FPS (NES native frame rate)
- Frame pacing is controlled by the emulator's `step_frame()` method
- ImGui rendering is efficient but can slow down with many windows open
- Consider using textures for large pixel buffers instead of drawing individual pixels

## Known Issues

1. Threading not yet implemented (see Threading Model section)
2. Pattern Table and Nametable viewers are placeholders
3. No texture rendering for emulator output yet (currently shows placeholder)
4. No controller input handling yet (only keyboard)
5. Frame buffer display needs to be implemented using SDL3 textures

## Future Improvements

1. Implement GPU texture rendering for emulator output
2. Add keyboard shortcut system
3. Add file dialog for ROM loading
4. Implement all debug viewers (Pattern Table, Nametable, etc.)
5. Add save state management
6. Add input configuration UI
7. Add audio visualization
8. Convert to multi-threaded architecture
