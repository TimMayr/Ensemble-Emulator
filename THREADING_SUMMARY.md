# Threading Implementation Summary

## Problem

A previous attempt to split emulation and UI onto separate threads failed because requiring `Arc<Mutex<Ppu>>` (to make `Nes` `Send`) added significant performance overhead compared to the gains from multi-threading.

## Solution

Create the `Nes` emulator **on the background thread** rather than moving it there, which avoids the `Send` requirement entirely. This allows us to keep the fast `Rc<RefCell<Ppu>>` pattern while still achieving true multi-threaded execution.

## Key Insight

**The `Send` trait is only required when MOVING a value from one thread to another.**

```rust
// ❌ WRONG: This requires Nes to be Send
let nes = Nes::default();           // Created on main thread
std::thread::spawn(move || {
    // nes moved here - REQUIRES Send!
});

// ✅ CORRECT: This does NOT require Nes to be Send
std::thread::spawn(|| {
    let nes = Nes::default();       // Created ON background thread
    // nes never crosses thread boundary!
});
```

## Implementation

### ThreadedEmulator (`frontend/src/threaded_emu.rs`)

- **Public API**: `ThreadedEmulator` holds thread handle and message sender
- **Private Worker**: `EmulatorWorker` runs on background thread, owns `Nes`
- **Message Protocol**: Same as `ChannelEmulator` for easy drop-in replacement
- **Clean Shutdown**: Drop handler sends Quit and joins thread

### Architecture

```text
UI Thread                          Background Thread
─────────                          ─────────────────
  │                                      │
  ├─ Send FrontendMessage ─────────────>│
  │   (StepFrame, LoadRom, etc.)        │
  │                                      ├─ Nes::new() created here
  │                                      ├─ process_messages()
  │                                      ├─ nes.step_frame()
  │                                      │
  │<──── Send EmulatorMessage ──────────┤
       (FrameReady, DebugData, etc.)    │
```

## Verification

The `threaded_emulator` example demonstrates:

```
$ cargo run --example threaded_emulator

=== ThreadedEmulator Demo ===
✓ Created ThreadedEmulator (Nes created on background thread)
✓ Completed 5 frames in 88.25ms (56.7 FPS equivalent)
✓ Main thread did 1815 work iterations while emulator ran
✓ Received 10 frames in 289.57ms
✓ Main thread remained responsive throughout!
```

## Performance Characteristics

### Rc<RefCell<Ppu>> (Current - Used by ThreadedEmulator)
- ✅ Reference counting without atomic operations
- ✅ Runtime borrow checking is fast
- ✅ No lock contention
- ✅ No thread synchronization overhead
- ❌ Not Send (cannot move between threads)

### Arc<Mutex<Ppu>> (Previous Failed Attempt)
- ✅ Send + Sync (can be shared between threads)
- ❌ Atomic reference counting (compare-and-swap overhead)
- ❌ Lock acquisition on every access
- ❌ Potential lock contention
- ❌ Thread synchronization overhead

## Benefits

1. **No Arc/Mutex overhead**: Keeps fast `Rc<RefCell<>>` performance
2. **Responsive UI**: Emulation runs in parallel with UI thread
3. **Same API**: Drop-in replacement for `ChannelEmulator`
4. **Clean separation**: Emulation and UI are truly independent

## Usage

```rust
use monsoon_frontend::threaded_emu::ThreadedEmulator;
use monsoon_frontend::messages::{FrontendMessage, EmulatorMessage};

// Create threaded emulator
let (emu, rx_from_emu) = ThreadedEmulator::new();

// Send messages to emulator
emu.send(FrontendMessage::Reset).unwrap();
emu.send(FrontendMessage::Power).unwrap();
emu.send(FrontendMessage::StepFrame).unwrap();

// Receive messages from emulator
if let Ok(EmulatorMessage::FrameReady(frame)) = rx_from_emu.recv() {
    // Render frame on UI thread
}

// Clean shutdown happens automatically when emu is dropped
```

## Next Steps (Optional)

1. **Add feature flag**: Allow choosing threaded vs non-threaded at compile time
2. **Runtime config**: Allow choosing at runtime via config file
3. **Performance benchmarks**: Measure actual performance improvements
4. **Integration**: Update `EguiApp` to optionally use `ThreadedEmulator`

## Files

- `frontend/src/threaded_emu.rs` - Implementation
- `frontend/examples/threaded_emulator.rs` - Working example
- `THREADING_ANALYSIS.md` - Detailed analysis
- `THREADING_SUMMARY.md` - This file

## Conclusion

By understanding that `Send` is only required when **moving** values between threads, we can create the emulator on the background thread and avoid the requirement entirely. This solves the previous threading failure by eliminating the Arc/Mutex overhead while still achieving true multi-threaded execution.
