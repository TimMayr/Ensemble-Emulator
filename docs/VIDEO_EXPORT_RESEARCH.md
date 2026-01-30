# Video Export Research: Frame Buffer to MP4

## Executive Summary

This document evaluates approaches for exporting NES emulator frame buffer data (256×240 @ 60fps, ARGB u32) to MP4 video files.

**Recommended Approach:** **Self-contained encoding with `rav1e` + WebM for pure Rust, or bundled FFmpeg via `video-rs` for MP4 support.**

> **Design Decision:** Since this is user-facing software, we should NOT assume users have ffmpeg installed. The primary implementation should be self-contained with no external runtime dependencies. FFmpeg pipe mode can be offered as an advanced/optional feature for power users who want maximum control.

---

## Current Frame Buffer State

From `core/src/emulation/ppu.rs`:

```rust
pub pixel_buffer: Vec<u32>  // 256×240 = 61,440 pixels
```

- **Format:** ARGB 32-bit (`0xAARRGGBB`)
- **Resolution:** 256×240 pixels (standard NES)
- **Frame Rate:** ~60.0988 fps (NTSC timing)
- **Access:** `ppu.pixel_buffer` or `nes.get_pixel_buffer()`

---

## Approach Comparison

| Approach | Self-Contained | Pros | Cons | Recommendation |
|----------|----------------|------|------|----------------|
| **rav1e + WebM** | ✅ Yes | Pure Rust, no deps | Not MP4, slower encoding | ✅ **Default for WebM** |
| **video-rs (bundled)** | ✅ Yes | Full MP4 support, fast | Larger binary (~2MB) | ✅ **Default for MP4** |
| **PNG sequence** | ✅ Yes | Simple, lossless | Large files, post-process needed | ✅ **For lossless** |
| **GIF (gif crate)** | ✅ Yes | Universal support | 256 color limit | ✅ **For short clips** |
| **FFmpeg pipe** | ❌ No | All codecs, flexible | Requires ffmpeg installed | ⚠️ **Advanced/optional** |
| **rsmpeg** | ❌ No | Modern API | Requires ffmpeg libs | ⚠️ **Advanced/optional** |
| **ffmpeg-next** | ❌ No | Feature-complete | Maintenance mode, needs libs | ❌ Not recommended |

---

## Detailed Analysis

### Option 1: video-rs with Bundled FFmpeg (Recommended for MP4)

The `video-rs` crate provides a high-level Rust API that bundles FFmpeg statically, meaning **no external dependencies required at runtime**.

**Cargo.toml:**

```toml
[dependencies]
video-rs = { version = "0.10", features = ["ndarray"] }
ndarray = "0.16"
```

**Implementation:**

```rust
use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;
use ndarray::Array3;

pub struct VideoRsEncoder {
    encoder: Encoder,
    position: Time,
    duration: Time,
}

impl VideoRsEncoder {
    pub fn new(output_path: &str, width: usize, height: usize, fps: u32) -> Result<Self, video_rs::Error> {
        video_rs::init()?;
        
        let settings = Settings::preset_h264_yuv420p(width, height, false);
        let encoder = Encoder::new(std::path::Path::new(output_path), settings)?;
        
        let duration = Time::from_nth_of_a_second(fps);
        
        Ok(Self {
            encoder,
            position: Time::zero(),
            duration,
        })
    }
    
    /// Write a frame from NES pixel buffer (ARGB u32)
    pub fn write_frame(&mut self, pixel_buffer: &[u32], width: usize, height: usize) -> Result<(), video_rs::Error> {
        // Convert ARGB u32 to RGB u8 ndarray
        let frame = Array3::from_shape_fn((height, width, 3), |(y, x, c)| {
            let pixel = pixel_buffer[y * width + x];
            match c {
                0 => ((pixel >> 16) & 0xFF) as u8,  // R
                1 => ((pixel >> 8) & 0xFF) as u8,   // G
                2 => (pixel & 0xFF) as u8,          // B
                _ => unreachable!(),
            }
        });
        
        self.encoder.encode(&frame, self.position)?;
        self.position = self.position.aligned_with(self.duration).add();
        
        Ok(())
    }
    
    pub fn finish(self) -> Result<(), video_rs::Error> {
        self.encoder.finish()
    }
}
```

**Pros:**
- ✅ **Self-contained** - No runtime dependencies for end users
- ✅ Clean Rust API
- ✅ Full H.264/H.265 MP4 support
- ✅ Handles all encoding complexity internally
- ✅ Cross-platform (Windows, macOS, Linux)

**Cons:**
- Increases binary size by ~2-5MB (bundled ffmpeg libs)
- Requires ffmpeg libraries at compile time (but NOT at runtime)
- Longer compile times

---

### Option 2: Pure Rust AV1/WebM (rav1e + webm crate)

For a completely pure Rust solution with no C dependencies at all.

**Cargo.toml:**

```toml
[dependencies]
rav1e = "0.7"
webm = "1.0"
```

**Implementation:**

```rust
use rav1e::prelude::*;

pub struct Rav1eEncoder {
    ctx: Context<u8>,
    // ...
}

impl Rav1eEncoder {
    pub fn new(output_path: &str, width: usize, height: usize, fps: u32) -> Result<Self, rav1e::EncoderStatus> {
        let enc = EncoderConfig {
            width,
            height,
            bit_depth: 8,
            time_base: Rational::new(1, fps as u64),
            ..Default::default()
        };
        
        let cfg = Config::new().with_encoder_config(enc);
        let ctx = cfg.new_context()?;
        
        Ok(Self { ctx })
    }
    
    pub fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), rav1e::EncoderStatus> {
        // Convert ARGB to YUV planes
        let mut frame = self.ctx.new_frame();
        // ... conversion logic ...
        self.ctx.send_frame(frame)?;
        Ok(())
    }
}
```

**Pros:**
- ✅ **100% Pure Rust** - No C dependencies whatsoever
- ✅ Modern AV1 codec (excellent quality)
- ✅ Smaller binary than bundled ffmpeg

**Cons:**
- Output is WebM/AV1, not MP4 (though widely supported)
- Slower encoding than hardware-accelerated codecs
- More complex implementation

---

### Option 3: FFmpeg Pipe (Advanced/Optional Feature)

For power users who have ffmpeg installed and want maximum control.

> **Note:** This should NOT be the default. It's an advanced option for users who explicitly want custom ffmpeg pipelines.

**Implementation:**

```rust
use std::io::{self, Write, BufWriter};
use std::process::{Command, Stdio};

pub struct FfmpegPipeEncoder {
    child: std::process::Child,
    stdin: BufWriter<std::process::ChildStdin>,
}

impl FfmpegPipeEncoder {
    pub fn new(output_path: &str, width: u32, height: u32, fps: f64) -> io::Result<Self> {
        let child = Command::new("ffmpeg")
            .args([
                "-y",                              // Overwrite output
                "-f", "rawvideo",                  // Input format
                "-pixel_format", "bgra",           // ARGB as little-endian = BGRA
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &format!("{}", fps),
                "-i", "-",                         // Read from stdin
                "-c:v", "libx264",                 // H.264 codec
                "-preset", "fast",                 // Encoding speed
                "-crf", "18",                      // Quality (0-51, lower = better)
                "-pix_fmt", "yuv420p",             // Output pixel format
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        
        let stdin = BufWriter::new(child.stdin.take().unwrap());
        Ok(Self { child, stdin })
    }
    
    /// Write a frame from NES pixel buffer (ARGB u32)
    pub fn write_frame(&mut self, pixel_buffer: &[u32]) -> io::Result<()> {
        // Convert ARGB u32 to BGRA bytes (for little-endian systems)
        let bytes: Vec<u8> = pixel_buffer.iter()
            .flat_map(|&pixel| {
                [
                    (pixel & 0xFF) as u8,          // B
                    ((pixel >> 8) & 0xFF) as u8,   // G
                    ((pixel >> 16) & 0xFF) as u8,  // R
                    ((pixel >> 24) & 0xFF) as u8,  // A
                ]
            })
            .collect();
        self.stdin.write_all(&bytes)
    }
    
    pub fn finish(mut self) -> io::Result<()> {
        self.stdin.flush()?;
        drop(self.stdin);
        self.child.wait()?;
        Ok(())
    }
}
```

**Usage:**

```rust
let mut encoder = FfmpegPipeEncoder::new("output.mp4", 256, 240, 60.0)?;

for _ in 0..frames {
    nes.step_frame();
    encoder.write_frame(&nes.ppu.pixel_buffer)?;
}

encoder.finish()?;
```

**Pros:**
- Maximum flexibility for custom encoding
- Access to all ffmpeg features
- No binary size increase

**Cons:**
- ❌ **Requires ffmpeg installed** - Not suitable as default for user-facing software
- External process management
- Poor user experience if ffmpeg not found

---

### Option 4: PNG Sequence (Universal Fallback)

Pure Rust, always works, for users who want to post-process themselves.

**Implementation:**

```rust
use image::{RgbaImage, Rgba};

pub fn save_frame_png(pixel_buffer: &[u32], path: &str) -> Result<(), image::ImageError> {
    let mut img = RgbaImage::new(256, 240);
    
    for (i, &pixel) in pixel_buffer.iter().enumerate() {
        let x = (i % 256) as u32;
        let y = (i / 256) as u32;
        
        img.put_pixel(x, y, Rgba([
            ((pixel >> 16) & 0xFF) as u8,  // R
            ((pixel >> 8) & 0xFF) as u8,   // G
            (pixel & 0xFF) as u8,          // B
            ((pixel >> 24) & 0xFF) as u8,  // A
        ]));
    }
    
    img.save(path)
}
```

**Usage:**

```bash
# Export frames
nes_main -H --rom game.nes --frames 600 --video-format png --video "frames/frame_%04d.png"

# Convert with ffmpeg
ffmpeg -framerate 60 -i frames/frame_%04d.png -c:v libx264 -crf 18 -pix_fmt yuv420p output.mp4
```

**Pros:**
- Pure Rust, no ffmpeg build dependency
- Lossless intermediate
- Flexible post-processing

**Cons:**
- Large disk usage (~100KB per frame × 3600 frames/minute = 360MB/minute)
- Two-step process
- Slower than direct encoding

---

### Option 5: GIF Output (for simple sharing)

Useful for short clips, not recommended for long recordings.

**Implementation:**

```rust
use gif::{Encoder, Frame, Repeat};
use std::fs::File;

pub struct GifEncoder {
    encoder: Encoder<File>,
    width: u16,
    height: u16,
}

impl GifEncoder {
    pub fn new(path: &str, width: u16, height: u16) -> std::io::Result<Self> {
        let file = File::create(path)?;
        let mut encoder = Encoder::new(file, width, height, &[])?;
        encoder.set_repeat(Repeat::Infinite)?;
        
        Ok(Self { encoder, width, height })
    }
    
    pub fn write_frame(&mut self, pixel_buffer: &[u32]) -> std::io::Result<()> {
        // Convert ARGB to RGB
        let mut rgb: Vec<u8> = pixel_buffer.iter()
            .flat_map(|&p| [
                ((p >> 16) & 0xFF) as u8,
                ((p >> 8) & 0xFF) as u8,
                (p & 0xFF) as u8,
            ])
            .collect();
        
        let frame = Frame::from_rgb(self.width, self.height, &mut rgb);
        self.encoder.write_frame(&frame)
    }
}
```

**Pros:**
- Pure Rust
- Universal support
- Small file size for simple graphics

**Cons:**
- 256 color limit (dithering required)
- Not MP4
- Large file size for complex frames
- Not suitable for long recordings

---

## Recommended Implementation Strategy

### Design Principles

1. **Self-contained by default** - Users should NOT need to install external tools
2. **Multiple output options** - Different formats for different use cases
3. **Advanced features optional** - FFmpeg pipe for power users only

### Phase 1: Self-Contained Implementation (Primary)

| Format | Implementation | User Experience |
|--------|---------------|-----------------|
| **MP4** | `video-rs` (bundled ffmpeg) | ✅ Just works - no install needed |
| **WebM** | `rav1e` (pure Rust) | ✅ Just works - no install needed |
| **PNG** | `image` crate | ✅ Just works - lossless frames |
| **GIF** | `gif` crate | ✅ Just works - short clips |

### Phase 2: Advanced Features (Optional)

| Format | Implementation | User Experience |
|--------|---------------|-----------------|
| **Raw pipe** | FFmpeg pipe | ⚠️ Requires ffmpeg - for power users |
| **Custom** | FFmpeg pipe | ⚠️ Requires ffmpeg - advanced control |

### Proposed Module Structure

```rust
// core/src/cli/video.rs

pub enum VideoFormat {
    Mp4,      // Self-contained via video-rs (default)
    WebM,     // Pure Rust via rav1e
    Png,      // PNG sequence via image crate
    Gif,      // Animated GIF via gif crate
    Raw,      // Raw BGRA bytes (for advanced ffmpeg piping)
}

pub trait VideoEncoder {
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError>;
    fn finish(self) -> Result<(), VideoError>;
}

// Self-contained encoders (no external deps at runtime)
pub struct Mp4Encoder { /* video-rs */ }
pub struct WebMEncoder { /* rav1e */ }
pub struct PngSequenceEncoder { /* image crate */ }
pub struct GifEncoder { /* gif crate */ }

// Advanced encoder (requires ffmpeg installed)
pub struct RawEncoder { /* writes to stdout for ffmpeg piping */ }

impl VideoFormat {
    pub fn encoder(&self, path: &str, width: u32, height: u32, fps: f64) -> Result<Box<dyn VideoEncoder>, VideoError> {
        match self {
            VideoFormat::Mp4 => Ok(Box::new(Mp4Encoder::new(path, width, height, fps)?)),
            VideoFormat::WebM => Ok(Box::new(WebMEncoder::new(path, width, height, fps)?)),
            VideoFormat::Png => Ok(Box::new(PngSequenceEncoder::new(path)?)),
            VideoFormat::Gif => Ok(Box::new(GifEncoder::new(path, width, height)?)),
            VideoFormat::Raw => Ok(Box::new(RawEncoder::new())),
        }
    }
    
    /// Returns true if this format requires external tools (ffmpeg)
    pub fn requires_external_tools(&self) -> bool {
        matches!(self, VideoFormat::Raw)
    }
}
```

---

## Dependencies to Add

```toml
[dependencies]
# For self-contained MP4 export (bundles ffmpeg codecs)
video-rs = { version = "0.10", features = ["ndarray"] }
ndarray = "0.16"

# For PNG sequence export
image = { version = "0.25", default-features = false, features = ["png"] }

# For GIF export  
gif = "0.13"

# For pure Rust WebM/AV1 (optional, no C deps)
# rav1e = { version = "0.7", optional = true }
# webm = { version = "1.0", optional = true }

[features]
# Enable pure Rust AV1/WebM encoding (slower but zero C dependencies)
# pure-rust-video = ["rav1e", "webm"]
```

---

## CLI Usage Examples

```bash
# Record 10 seconds of gameplay to MP4 (self-contained, just works!)
nes_main -H --rom game.nes --frames 600 --video gameplay.mp4

# Record to WebM (pure Rust, no external deps)
nes_main -H --rom game.nes --frames 600 --video gameplay.webm --video-format webm

# Record with custom quality
nes_main -H --rom game.nes --frames 600 --video gameplay.mp4 --video-quality high

# Record PNG sequence (lossless)
nes_main -H --rom game.nes --frames 600 --video-format png --video "frames/frame_%04d.png"

# Record animated GIF (short clips only)
nes_main -H --rom game.nes --frames 60 --video clip.gif --video-format gif

# ADVANCED: Raw output for custom ffmpeg pipeline (requires ffmpeg installed)
nes_main -H --rom game.nes --frames 600 --video-format raw | \
  ffmpeg -f rawvideo -pixel_format bgra -video_size 256x240 \
         -framerate 60 -i - -c:v libx265 -crf 20 output.mp4

# Record debug views
nes_main -H --rom game.nes --frames 600 --export-nametables-video nametables.mp4
nes_main -H --rom game.nes --frames 600 --export-pattern-tables-video chr.mp4
```

---

## Performance Considerations

| Format | Write Speed | File Size | Quality |
|--------|-------------|-----------|---------|
| Raw (pipe) | ~1000+ fps | N/A (piped) | Lossless |
| MP4 (ffmpeg) | ~200-500 fps | ~1MB/min | Excellent |
| PNG sequence | ~100-200 fps | ~360MB/min | Lossless |
| GIF | ~50-100 fps | ~10-50MB/min | Limited |

Note: Emulation at 60fps is the limiting factor, not encoding.

---

## Error Handling

```rust
#[derive(Debug)]
pub enum VideoError {
    FfmpegNotFound,
    FfmpegFailed(String),
    IoError(std::io::Error),
    EncodingError(String),
}

impl From<std::io::Error> for VideoError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::NotFound {
            VideoError::FfmpegNotFound
        } else {
            VideoError::IoError(e)
        }
    }
}
```

---

## Conclusion

**Self-contained `video-rs`** is the recommended primary approach because:

1. **Just works for users** - No need to install ffmpeg or any external tools
2. **Full MP4/H.264 support** - Industry standard format that plays everywhere
3. **Cross-platform** - Works on Windows, macOS, and Linux out of the box
4. **Clean Rust API** - Easy to integrate and maintain
5. **Reasonable binary size** - Only adds ~2-5MB to the executable

For users who need additional options:
- **WebM/AV1** via `rav1e` for pure Rust (optional feature)
- **PNG sequence** for lossless frames
- **GIF** for short shareable clips
- **Raw pipe** for advanced users with ffmpeg installed

> **Key Principle:** User-facing software should work out of the box. External dependencies like ffmpeg should only be required for optional advanced features, never for basic functionality.
