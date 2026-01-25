# CLI Interface Design Document

## Table of Contents

1. [Overview](#overview)
2. [Design Philosophy](#design-philosophy)
3. [Architecture](#architecture)
4. [Command Line Arguments](#command-line-arguments)
5. [ROM Loading](#rom-loading)
6. [Savestate Management](#savestate-management)
7. [Memory Operations](#memory-operations)
8. [Power Control](#power-control)
9. [Palette Configuration](#palette-configuration)
10. [Video Export](#video-export)
11. [Execution Control](#execution-control)
12. [Output Formats](#output-formats)
13. [Example Workflows](#example-workflows)
14. [Implementation Notes](#implementation-notes)

---

## Overview

This document describes a standardized CLI interface for the NES emulator that enables programmatic automation of emulation tasks. The interface is designed to support complex multi-step behaviors driven entirely by command-line arguments, making it suitable for:

- Automated testing and verification
- Tool-assisted speedrun (TAS) development
- Memory inspection and debugging
- Batch processing and screenshot/video generation
- Integration with external tools and scripts

The CLI interface builds upon the existing message-based architecture (`FrontendMessage`/`EmulatorMessage`) and extends the current headless mode with comprehensive control capabilities.

---

## Design Philosophy

### Principles

1. **Composability**: Individual operations should be combinable to create complex workflows
2. **Reproducibility**: Given the same inputs, the emulator should produce identical outputs
3. **Discoverability**: Options should be self-documenting with sensible defaults
4. **Safety**: Destructive operations should require explicit confirmation
5. **Integration**: Seamlessly integrate with the existing message-based architecture

### Consistency with Existing Code

The CLI interface should map directly to existing `FrontendMessage` variants where possible:

| CLI Operation | Existing Message |
|--------------|------------------|
| Load ROM | `FrontendMessage::LoadRom(PathBuf)` |
| Reset | `FrontendMessage::Reset` |
| Power On | `FrontendMessage::Power` |
| Power Off | `FrontendMessage::PowerOff` |
| Set Palette | `FrontendMessage::SetPalette(Box<RgbPalette>)` |
| Write CPU Memory | `FrontendMessage::WriteCpu(u16, u8)` |
| Write PPU Memory | `FrontendMessage::WritePpu(u16, u8)` |
| Load Savestate | `FrontendMessage::LoadSaveState(Box<SaveState>)` |
| Create Savestate | `FrontendMessage::CreateSaveState` |

---

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        CLI Entry Point                               │
│                   (core/src/bin/main.rs)                            │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     Argument Parser (clap)                          │
│                                                                      │
│  • Parses and validates all CLI arguments                           │
│  • Builds CliConfig struct with all options                         │
│  • Handles argument groups and conflicts                            │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      CLI Execution Engine                           │
│                                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │
│  │   Phase 1:   │  │   Phase 2:   │  │        Phase 3:          │  │
│  │    Setup     │─▶│  Initialize  │─▶│        Execute           │  │
│  │              │  │              │  │                          │  │
│  │ • Load ROM   │  │ • Init Memory│  │ • Run until condition    │  │
│  │ • Load State │  │ • Set Palette│  │ • Handle stop triggers   │  │
│  │ • Power On   │  │ • Init Regs  │  │ • Capture outputs        │  │
│  └──────────────┘  └──────────────┘  └──────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     Output Handler                                   │
│                                                                      │
│  • Memory dumps (hex/binary/JSON)                                   │
│  • Screenshots (PNG)                                                │
│  • Video files (using external encoder)                             │
│  • Savestates (rkyv serialized)                                     │
│  • Debug viewer exports                                             │
└─────────────────────────────────────────────────────────────────────┘
```

### Integration with Existing Code

The CLI engine should leverage the existing `Nes` struct and its methods:

```rust
// From core/src/emulation/nes.rs
impl Nes {
    pub fn power(&mut self);           // Power on the console
    pub fn power_off(&mut self);       // Power off the console
    pub fn reset(&mut self);           // Reset the console
    pub fn load_rom<T>(&mut self, rom_get: &T);  // Load a ROM
    pub fn save_state(&self) -> SaveState;       // Create savestate
    pub fn load_state(&mut self, state: SaveState);  // Load savestate
    pub fn run_until(&mut self, last_cycle: u128);   // Run until cycle
    pub fn step_frame(&mut self);      // Run one frame
    pub fn get_memory_debug(&self, range) -> Vec<Vec<u8>>;  // Memory dump
}
```

### Proposed New Module Structure

```
core/src/
├── bin/
│   └── main.rs              # Entry point (updated)
├── cli/
│   ├── mod.rs               # CLI module root
│   ├── args.rs              # Argument definitions (clap derive)
│   ├── config.rs            # CliConfig struct
│   ├── engine.rs            # CLI execution engine
│   ├── memory_ops.rs        # Memory read/write operations
│   ├── output.rs            # Output formatting/export
│   └── stop_conditions.rs   # Execution stop conditions
└── ...
```

---

## Command Line Arguments

### Argument Organization

Arguments are organized into logical groups:

#### Global Options

| Flag | Long Form | Description | Type | Default |
|------|-----------|-------------|------|---------|
| `-H` | `--headless` | Run without GUI | bool | false |
| `-q` | `--quiet` | Suppress non-error output | bool | false |
| `-v` | `--verbose` | Enable verbose output | bool | false |
| `--version` | | Print version and exit | | |
| `--help` | | Print help information | | |

#### ROM Loading

| Flag | Long Form | Description | Type |
|------|-----------|-------------|------|
| `-r` | `--rom` | Path to ROM file | PathBuf |
| | `--rom-info` | Print ROM information and exit | bool |

#### Savestate Operations

| Flag | Long Form | Description | Type |
|------|-----------|-------------|------|
| `-l` | `--load-state` | Load savestate from file | PathBuf |
| `-s` | `--save-state` | Save state to file on exit | PathBuf |
| | `--state-stdin` | Read savestate from stdin | bool |
| | `--state-stdout` | Write savestate to stdout on exit | bool |
| | `--save-state-on` | When to save state (see below) | String |

**`--save-state-on` Options:**
- `exit` - Save when emulator exits normally
- `stop` - Save when any stop condition is triggered
- `cycle:N` - Save at specific cycle N
- `pc:ADDR` - Save when PC reaches address
- `frame:N` - Save after N frames

#### Memory Operations

| Flag | Long Form | Description | Type |
|------|-----------|-------------|------|
| | `--read-cpu` | Read CPU memory range | String |
| | `--read-ppu` | Read PPU memory range | String |
| | `--read-oam` | Dump OAM (sprite) memory | bool |
| | `--read-palette` | Dump palette RAM | bool |
| | `--init-cpu` | Initialize CPU memory | String |
| | `--init-ppu` | Initialize PPU memory | String |
| | `--init-oam` | Initialize OAM from file | PathBuf |
| | `--init-file` | Load init values from file | PathBuf |

**Memory Range Format:** `START-END` or `START:LENGTH` (hex addresses)
- Examples: `0x0000-0x07FF`, `0x6000:0x2000`, `0x2000-0x3FFF`

**Memory Init Format:** `ADDR=VALUE` or `ADDR=VALUE1,VALUE2,...` (hex)
- Examples: `0x0000=0xFF`, `0x6000=0x01,0x02,0x03,0x04`

#### Power Control

| Flag | Long Form | Description | Type | Default |
|------|-----------|-------------|------|---------|
| | `--no-power` | Don't auto-power on after ROM load | bool | false |
| | `--reset` | Reset after loading | bool | false |
| | `--power-cycles` | Warm-up cycles before execution | u64 | 0 |

#### Palette Configuration

| Flag | Long Form | Description | Type |
|------|-----------|-------------|------|
| `-p` | `--palette` | Path to .pal RGB palette file | PathBuf |
| | `--palette-builtin` | Use built-in palette by name | String |

**Built-in Palettes:**
- `2C02G` (default) - Standard 2C02G palette
- `2C02G-smooth` - Smooth variant
- `composite` - NTSC composite simulation

#### Video/Screenshot Export

| Flag | Long Form | Description | Type |
|------|-----------|-------------|------|
| | `--screenshot` | Save screenshot on exit | PathBuf |
| | `--screenshot-on` | When to capture (same as save-state-on) | String |
| | `--video` | Record video to file | PathBuf |
| | `--video-format` | Video output format | String |
| | `--video-fps` | Video frame rate | u32 |
| | `--export-nametables` | Export nametable viewer | PathBuf |
| | `--export-pattern-tables` | Export pattern table viewer | PathBuf |
| | `--export-sprites` | Export sprite viewer | PathBuf |

**Video Formats:**
- `raw` - Raw RGBA frames (for piping to ffmpeg)
- `ppm` - PPM image sequence
- `png` - PNG image sequence

#### Execution Control

| Flag | Long Form | Description | Type |
|------|-----------|-------------|------|
| `-c` | `--cycles` | Run for N master cycles | u128 |
| `-f` | `--frames` | Run for N frames | u64 |
| | `--until-pc` | Run until PC reaches address | u16 |
| | `--until-opcode` | Run until specific opcode executes | u8 |
| | `--until-mem` | Run until memory condition | String |
| | `--until-hlt` | Run until HLT instruction | bool |
| | `--step` | Single-step mode | bool |
| | `--trace` | Enable instruction trace | PathBuf |
| | `--breakpoint` | Set breakpoint at address | Vec<u16> |

**Memory Condition Format:** `ADDR==VALUE`, `ADDR!=VALUE`, `ADDR&MASK==VALUE`
- Examples: `0x6000==0x80`, `0x2002&0x80!=0x00`

#### Output Control

| Flag | Long Form | Description | Type | Default |
|------|-----------|-------------|------|---------|
| `-o` | `--output` | Output file for memory dumps | PathBuf | stdout |
| | `--output-format` | Output format | String | hex |
| | `--json` | Output in JSON format | bool | false |
| | `--binary` | Output in binary format | bool | false |

---

## ROM Loading

### Basic ROM Loading

```bash
# Load and run ROM
nes_main --headless --rom game.nes --frames 100

# Load ROM without auto-power
nes_main --headless --rom game.nes --no-power

# Print ROM information only
nes_main --rom game.nes --rom-info
```

### ROM Information Output

The `--rom-info` flag should output:

```
ROM Information:
  File: game.nes
  Name: Super Mario Bros.
  Format: iNES 2.0
  Mapper: 0 (NROM)
  PRG ROM: 32 KB
  CHR ROM: 8 KB
  PRG RAM: 8 KB (battery-backed: no)
  Mirroring: Horizontal
  Console Type: NES/Famicom
  Checksum (SHA-256): abc123...
```

---

## Savestate Management

### Basic Savestate Operations

```bash
# Save state after running
nes_main -H --rom game.nes --frames 100 --save-state state.sav

# Load existing state
nes_main -H --rom game.nes --load-state state.sav --frames 100

# Chain operations: load state, run, save new state
nes_main -H --rom game.nes -l input.sav --frames 60 -s output.sav
```

### Pipe-Based Savestates (Streaming Workflows)

For multi-step automation pipelines, savestates can be read from stdin and written to stdout:

```bash
# Single step pipeline
nes_main -H --rom game.nes --frames 100 --state-stdout | \
nes_main -H --rom game.nes --state-stdin --frames 50 --state-stdout | \
nes_main -H --rom game.nes --state-stdin --frames 25 --save-state final.sav
```

### Savestate Format

Savestates use the existing `rkyv` serialization format from `savestate.rs`. The structure includes:

```rust
pub struct SaveState {
    pub cpu: CpuState,           // CPU registers and RAM
    pub ppu: PpuState,           // PPU state and VRAM
    pub rom_file: RomFile,       // ROM metadata (for verification)
    pub version: u16,            // Savestate format version
    pub total_cycles: u128,      // Total elapsed cycles
    pub cycle: u8,               // Current sub-cycle
    pub ppu_cycle_counter: u8,   // PPU cycle position
    pub cpu_cycle_counter: u8,   // CPU cycle position
}
```

### Conditional Savestates

```bash
# Save at specific cycle
nes_main -H --rom game.nes --save-state-on cycle:1000000 -s milestone.sav

# Save when PC reaches address (e.g., level complete routine)
nes_main -H --rom game.nes --save-state-on pc:0x8500 -s level_complete.sav

# Save on any stop condition
nes_main -H --rom game.nes --until-pc 0x8500 --save-state-on stop -s stopped.sav
```

---

## Memory Operations

### Reading Memory

#### CPU Memory (Addresses 0x0000-0xFFFF)

```bash
# Read zero page
nes_main -H --rom game.nes --frames 100 --read-cpu 0x0000-0x00FF

# Read RAM (with mirrors)
nes_main -H --rom game.nes --frames 100 --read-cpu 0x0000-0x07FF

# Read PRG RAM (save data area)
nes_main -H --rom game.nes --frames 100 --read-cpu 0x6000-0x7FFF

# Read specific range with length
nes_main -H --rom game.nes --frames 100 --read-cpu 0x6000:0x100

# Output to file
nes_main -H --rom game.nes --frames 100 --read-cpu 0x0000-0x07FF -o ram.bin --binary
```

#### PPU Memory (Addresses 0x0000-0x3FFF)

```bash
# Read pattern tables (CHR ROM/RAM)
nes_main -H --rom game.nes --frames 100 --read-ppu 0x0000-0x1FFF

# Read nametables
nes_main -H --rom game.nes --frames 100 --read-ppu 0x2000-0x2FFF

# Read palette RAM
nes_main -H --rom game.nes --frames 100 --read-palette
```

#### OAM (Sprite Memory)

```bash
# Dump full OAM (256 bytes, 64 sprites)
nes_main -H --rom game.nes --frames 100 --read-oam

# JSON format with sprite interpretation
nes_main -H --rom game.nes --frames 100 --read-oam --json
```

### Initializing Memory

#### CPU Memory Initialization

```bash
# Set single byte
nes_main -H --rom game.nes --init-cpu 0x0050=0xFF --frames 100

# Set multiple bytes
nes_main -H --rom game.nes --init-cpu 0x0050=0x01,0x02,0x03,0x04 --frames 100

# Multiple init operations
nes_main -H --rom game.nes \
  --init-cpu 0x0050=0xFF \
  --init-cpu 0x0060=0x01,0x02 \
  --frames 100
```

#### PPU Memory Initialization

```bash
# Initialize VRAM
nes_main -H --rom game.nes --init-ppu 0x2000=0x20,0x20,0x20 --frames 100

# Initialize palette RAM
nes_main -H --rom game.nes --init-ppu 0x3F00=0x0F,0x00,0x10,0x20 --frames 100
```

#### Initialization from File

```bash
# Init file format (JSON):
# {
#   "cpu": {"0x0050": [1, 2, 3, 4], "0x0060": [255]},
#   "ppu": {"0x3F00": [15, 0, 16, 32]},
#   "oam": "base64encodeddata..."
# }
nes_main -H --rom game.nes --init-file init.json --frames 100
```

### Memory Access Timing

Memory initialization happens:
1. **After ROM loading** - ROM is loaded and mapped
2. **After power-on** - CPU/PPU are in initialized state
3. **After savestate load** - If loading a savestate
4. **Before execution** - Just before running cycles/frames

This ensures that initialized values are present when execution begins.

---

## Power Control

### Power Sequence

```bash
# Normal: Load ROM → Power On → Execute
nes_main -H --rom game.nes --frames 100

# Manual power control
nes_main -H --rom game.nes --no-power  # ROM loaded but not powered
```

### Reset Operations

```bash
# Power on then immediately reset (mimics physical reset)
nes_main -H --rom game.nes --reset --frames 100

# Multiple resets (for testing reset behavior)
# First run to state, then reset
nes_main -H --rom game.nes --frames 100 -s pre_reset.sav
nes_main -H --rom game.nes -l pre_reset.sav --reset --frames 100 -s post_reset.sav
```

### Power Cycles (Warm-up)

Some tests require the emulator to run for a specific number of cycles before the main execution begins:

```bash
# Run 1000 power-up cycles before main execution
nes_main -H --rom game.nes --power-cycles 1000 --frames 100
```

---

## Palette Configuration

### Loading Custom Palettes

```bash
# Load custom .pal file (192-byte or 1536-byte format)
nes_main -H --rom game.nes --palette custom.pal --frames 100

# Use built-in palette
nes_main -H --rom game.nes --palette-builtin 2C02G --frames 100
```

### Palette File Format

The emulator supports the standard NES palette format:

**192-byte format (single palette):**
- 64 colors × 3 bytes (RGB) = 192 bytes
- Colors are in NES palette order (0x00-0x3F)

**1536-byte format (8 emphasis variants):**
- 8 emphasis modes × 64 colors × 3 bytes = 1536 bytes
- Emphasis modes: Normal, R, G, RG, B, RB, GB, RGB

### Palette Emphasis

When using the 1536-byte format, the correct emphasis palette is automatically selected based on the PPU mask register bits.

---

## Video Export

### Screenshots

```bash
# Screenshot on exit
nes_main -H --rom game.nes --frames 100 --screenshot frame100.png

# Screenshot at specific frame
nes_main -H --rom game.nes --screenshot-on frame:100 --screenshot shot.png

# Screenshot when reaching specific address
nes_main -H --rom game.nes --screenshot-on pc:0x8500 --screenshot level_end.png
```

### Video Recording

```bash
# Record raw frames (pipe to ffmpeg)
nes_main -H --rom game.nes --frames 600 --video-format raw --video - | \
  ffmpeg -f rawvideo -pixel_format rgba -video_size 256x240 \
         -framerate 60 -i - output.mp4

# Record PNG sequence
nes_main -H --rom game.nes --frames 600 --video frame_%04d.png --video-format png

# Record PPM sequence (faster, larger files)
nes_main -H --rom game.nes --frames 600 --video frame_%04d.ppm --video-format ppm
```

### Debug Viewer Exports

```bash
# Export pattern tables
nes_main -H --rom game.nes --frames 100 --export-pattern-tables chr.png

# Export nametables
nes_main -H --rom game.nes --frames 100 --export-nametables nametables.png

# Export sprite viewer
nes_main -H --rom game.nes --frames 100 --export-sprites sprites.png
```

---

## Execution Control

### Cycle-Based Execution

```bash
# Run for exact number of master cycles
nes_main -H --rom game.nes --cycles 1000000

# Run for exact number of frames
nes_main -H --rom game.nes --frames 60
```

### Conditional Stop

```bash
# Stop when PC reaches address
nes_main -H --rom game.nes --until-pc 0x8500

# Stop when specific opcode executes (0x02 is KIL, an illegal "halt" opcode)
nes_main -H --rom game.nes --until-opcode 0x02  # Stop on KIL (illegal halt)

# Stop when memory condition is met
nes_main -H --rom game.nes --until-mem "0x6000==0x80"

# Combined conditions (stops on first match)
nes_main -H --rom game.nes --frames 3600 --until-pc 0x8500 --until-mem "0x6000==0x80"
```

### Breakpoints and Single-Step

```bash
# Set breakpoints (execution pauses at each)
nes_main -H --rom game.nes --breakpoint 0x8000 --breakpoint 0x8500 --trace trace.log

# Single-step mode (outputs state after each instruction)
nes_main -H --rom game.nes --step --cycles 100
```

### Instruction Tracing

The trace format should be compatible with existing trace systems:

```bash
# Enable tracing to file
nes_main -H --rom game.nes --trace execution.log --frames 10
```

**Trace Format:**
```
C000  78        SEI            A:00 X:00 Y:00 P:04 SP:FD CYC:7
C001  D8        CLD            A:00 X:00 Y:00 P:04 SP:FD CYC:9
C002  A9 10     LDA #$10       A:00 X:00 Y:00 P:04 SP:FD CYC:11
...
```

---

## Output Formats

### Hexadecimal (Default)

```
0000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
...
```

### Binary

Raw binary data written to file or stdout.

### JSON

```json
{
  "memory_dump": {
    "type": "cpu",
    "start": "0x0000",
    "end": "0x07FF",
    "data": "AAAAAAAAAAAAAAAA..."
  },
  "registers": {
    "pc": "0xC000",
    "a": "0x00",
    "x": "0x00",
    "y": "0x00",
    "sp": "0xFD",
    "p": "0x04"
  },
  "cycles": 1000000,
  "frames": 16
}
```

### OAM JSON Format

```json
{
  "sprites": [
    {
      "index": 0,
      "y": 64,
      "tile": 1,
      "attributes": {
        "palette": 0,
        "priority": false,
        "flip_h": false,
        "flip_v": false
      },
      "x": 128
    },
    ...
  ],
  "raw": "base64..."
}
```

---

## Example Workflows

### Workflow 1: Automated ROM Testing

```bash
#!/bin/bash
# Test ROM runs without crashing for 1 minute of game time

nes_main --headless \
  --rom "$1" \
  --frames 3600 \
  --quiet

if [ $? -eq 0 ]; then
  echo "PASS: $1"
else
  echo "FAIL: $1"
fi
```

### Workflow 2: Memory Comparison Tool

```bash
#!/bin/bash
# Compare RAM state before and after running

nes_main -H --rom game.nes --frames 100 --read-cpu 0x0000-0x07FF -o before.bin --binary
nes_main -H --rom game.nes --frames 200 --read-cpu 0x0000-0x07FF -o after.bin --binary
diff before.bin after.bin && echo "No changes" || echo "Memory changed"
```

### Workflow 3: TAS Input Testing

```bash
#!/bin/bash
# Test a sequence of inputs using savestate chains

# Frame 0: Start state
nes_main -H --rom game.nes --frames 1 --state-stdout > state0.sav

# Apply input A, run 10 frames
cat state0.sav | nes_main -H --rom game.nes --state-stdin \
  --init-cpu 0x4016=0x01 --frames 10 --state-stdout > state1.sav

# Apply input B, run 10 frames  
cat state1.sav | nes_main -H --rom game.nes --state-stdin \
  --init-cpu 0x4016=0x02 --frames 10 --state-stdout > state2.sav

# Check final state
cat state2.sav | nes_main -H --rom game.nes --state-stdin \
  --read-cpu 0x0050-0x0060 --json
```

### Workflow 4: Sprite Extraction

```bash
#!/bin/bash
# Extract sprite data at specific game moment

nes_main -H --rom game.nes \
  --until-pc 0x8500 \
  --read-oam --json \
  --export-sprites sprites.png
```

### Workflow 5: Screenshot Generation Pipeline

```bash
#!/bin/bash
# Generate screenshots at regular intervals

for frame in 100 200 300 400 500; do
  nes_main -H --rom game.nes \
    --frames $frame \
    --screenshot "frame_$frame.png"
done
```

### Workflow 6: Conditional Execution with Fallback

```bash
#!/bin/bash
# Run until condition, but timeout after 1 hour of game time

nes_main -H --rom game.nes \
  --frames 216000 \
  --until-mem "0x6000==0x80" \
  --save-state-on stop \
  --save-state result.sav

if [ $? -eq 0 ]; then
  echo "Condition met, state saved"
else
  echo "Timeout reached"
fi
```

---

## Implementation Notes

### Integration with Existing Architecture

#### Leveraging ChannelEmulator

The CLI can optionally use the `ChannelEmulator` infrastructure for consistency:

```rust
// Option 1: Direct Nes manipulation (current headless approach)
let mut emu = Nes::default();
emu.load_rom(&rom_path);
emu.power();
emu.run_until(target_cycles);

// Option 2: Message-based approach (more consistent with GUI)
let (mut channel_emu, tx, rx) = ChannelEmulator::new(Nes::default());
tx.send(FrontendMessage::LoadRom(rom_path));
tx.send(FrontendMessage::Power);
// Process messages in a loop
```

For the CLI, **Option 1 (direct manipulation)** is recommended for:
- Lower overhead
- Simpler control flow
- Easier cycle-accurate timing

However, option 2 should be used when features require it (e.g., debug data that goes through the message system).

### Memory Init Implementation

Memory initialization should map to existing memory write operations. The method naming
uses `init` to indicate this happens before execution begins, distinguishing it from
runtime memory writes:

```rust
// In the CLI engine
fn apply_memory_init(nes: &mut Nes, cpu_inits: &[(u16, Vec<u8>)], ppu_inits: &[(u16, Vec<u8>)]) {
    for (addr, bytes) in cpu_inits {
        for (i, byte) in bytes.iter().enumerate() {
            // Uses the existing init method which bypasses normal bus behavior
            nes.cpu.memory.init(*addr + i as u16, *byte);
        }
    }
    
    for (addr, bytes) in ppu_inits {
        for (i, byte) in bytes.iter().enumerate() {
            // Uses mem_init for direct PPU memory initialization
            nes.ppu.borrow_mut().mem_init(*addr + i as u16, *byte);
        }
    }
}
```

### Stop Condition Implementation

```rust
pub enum StopCondition {
    Cycles(u128),
    Frames(u64),
    ProgramCounter(u16),
    Opcode(u8),
    MemoryCondition {
        address: u16,
        mask: u8,
        operation: CompareOp,
        value: u8,
    },
    Halt,
}

pub enum CompareOp {
    Equal,
    NotEqual,
}

impl StopCondition {
    fn is_met(&self, nes: &Nes, current_cycles: u128, current_frames: u64) -> bool {
        match self {
            StopCondition::Cycles(target) => current_cycles >= *target,
            StopCondition::Frames(target) => current_frames >= *target,
            StopCondition::ProgramCounter(addr) => nes.cpu.program_counter == *addr,
            StopCondition::Opcode(op) => {
                nes.cpu.current_opcode.map(|o| o.opcode == *op).unwrap_or(false)
            }
            StopCondition::MemoryCondition { address, mask, operation, value } => {
                let mem_val = nes.cpu.memory.mem_read(*address) & mask;
                match operation {
                    CompareOp::Equal => mem_val == *value,
                    CompareOp::NotEqual => mem_val != *value,
                }
            }
            StopCondition::Halt => nes.cpu.is_halted,
        }
    }
}
```

### Video Export Implementation

For raw frame export:

```rust
fn export_frame(frame: &[u32], output: &mut impl Write, format: VideoFormat) -> io::Result<()> {
    match format {
        VideoFormat::Raw => {
            // RGBA format, 4 bytes per pixel
            for pixel in frame {
                output.write_all(&pixel.to_le_bytes())?;
            }
        }
        VideoFormat::Ppm => {
            writeln!(output, "P6")?;
            writeln!(output, "256 240")?;
            writeln!(output, "255")?;
            for pixel in frame {
                // Extract RGB, ignore alpha
                output.write_all(&[
                    ((pixel >> 16) & 0xFF) as u8,
                    ((pixel >> 8) & 0xFF) as u8,
                    (pixel & 0xFF) as u8,
                ])?;
            }
        }
        // PNG would use image crate
    }
    Ok(())
}
```

### Pipe-Based Savestate Serialization

```rust
// Write savestate to stdout
fn write_state_stdout(state: &SaveState) -> io::Result<()> {
    let bytes = rkyv::to_bytes::<rkyv::rancor::BoxedError>(state)
        .map_err(|e| io::Error::other(e))?;
    io::stdout().write_all(&bytes)
}

// Read savestate from stdin  
fn read_state_stdin() -> io::Result<SaveState> {
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes)?;
    rkyv::from_bytes::<SaveState, rkyv::rancor::BoxedError>(&bytes)
        .map_err(|e| io::Error::other(e))
}
```

### Error Handling

The CLI should use standard exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | ROM load failed |
| 4 | Savestate load failed |
| 5 | I/O error |
| 6 | Timeout/stop condition not met |

### Configuration File Support (Future)

Consider supporting a configuration file for complex setups:

```toml
# nes_cli.toml
[rom]
path = "game.nes"

[savestate]
load = "initial.sav"
save = "final.sav"
save_on = "stop"

[memory.init.cpu]
"0x0050" = [0xFF, 0x00, 0x10]
"0x0060" = [0x01]

[execution]
stop_conditions = ["pc:0x8500", "frames:3600"]

[output]
format = "json"
screenshot = "final.png"
```

---

## Summary

This CLI interface design provides:

1. **Complete control** over the emulator via command-line arguments
2. **Composable operations** that can be chained together
3. **Reproducible results** with deterministic execution
4. **Integration** with the existing message-based architecture
5. **Extensibility** for future features

The design prioritizes:
- Minimal changes to existing code
- Reuse of existing infrastructure (messages, savestate format, etc.)
- Clear separation between CLI parsing, execution, and output
- Compatibility with shell scripting and pipeline workflows

Implementation should proceed in phases:
1. Basic ROM loading and cycle-count execution
2. Savestate load/save with pipe support
3. Memory read/write operations
4. Stop conditions and breakpoints
5. Video/screenshot export
6. Debug viewer exports
