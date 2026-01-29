# Video Export Research: Frame Buffer to MP4

## Executive Summary

This document evaluates approaches for exporting NES emulator frame buffer data (256×240 @ 60fps, ARGB u32) to MP4 video files.

**Recommended Approach:** **FFmpeg piping with `video-rs` as fallback for self-contained builds.**

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

| Approach | Pros | Cons | Complexity | Dependencies |
|----------|------|------|------------|--------------|
| **FFmpeg pipe** | Mature, efficient, all codecs | Requires ffmpeg installed | Low | External binary |
| **video-rs** | Rust API, ffmpeg features | Requires ffmpeg libs | Medium | ffmpeg libs |
| **rsmpeg** | Modern API, well-maintained | Requires ffmpeg libs | Medium | ffmpeg libs |
| **ffmpeg-next** | Feature-complete | Maintenance mode, complex | High | ffmpeg libs |
| **GIF (gif crate)** | Pure Rust, no deps | Not MP4, 256 color limit | Low | None |
| **PNG sequence** | Simple, lossless | Large files, needs ffmpeg to stitch | Very Low | None |
| **WebM (rav1e)** | Pure Rust AV1 | Slow encoding, not MP4 | High | None |

---

## Detailed Analysis

### Option 1: FFmpeg Pipe (Recommended for CLI)

The simplest and most robust approach. Output raw frames to stdout, pipe to external ffmpeg.

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
- No build-time dependencies
- Full codec support (H.264, H.265, VP9, AV1, etc.)
- Battle-tested, industry standard
- Easy to customize (bitrate, quality, filters)

**Cons:**
- Requires ffmpeg installed on system
- External process management

**CLI Integration:**

```bash
# User-friendly command
nes_main -H --rom game.nes --frames 600 --video output.mp4

# Advanced: pipe raw for custom encoding
nes_main -H --rom game.nes --frames 600 --video-format raw --video - | \
  ffmpeg -f rawvideo -pixel_format bgra -video_size 256x240 \
         -framerate 60 -i - -c:v libx264 -preset slow -crf 15 output.mp4
```

---

### Option 2: video-rs Crate (Recommended for Embedded)

High-level Rust wrapper around ffmpeg. Good balance of simplicity and features.

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
- Clean Rust API
- Handles ffmpeg complexities internally
- No shell process spawning

**Cons:**
- Requires ffmpeg libraries at build time
- Platform-specific build setup
- Additional 2MB+ binary size

---

### Option 3: PNG Sequence + Post-Processing

For maximum simplicity and flexibility, export PNG sequence then convert.

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

### Option 4: GIF Output (for simple sharing)

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

### Phase 1: FFmpeg Pipe (Immediate)

1. Add `--video <path>` and `--video-format` arguments (already in args.rs)
2. Implement `FfmpegPipeEncoder` in `cli/video.rs`
3. Support formats: `raw`, `mp4`, `png`, `gif`
4. For `mp4`: spawn ffmpeg, pipe raw frames
5. For `raw`: write to stdout (for custom ffmpeg commands)
6. For `png`: use `image` crate sequence
7. For `gif`: use `gif` crate

### Phase 2: Self-Contained (Optional)

1. Add `video-rs` as optional dependency behind feature flag
2. Use for embedded builds where ffmpeg install isn't guaranteed
3. Feature: `video-builtin`

### Proposed Module Structure

```rust
// core/src/cli/video.rs

pub enum VideoFormat {
    Raw,      // Raw BGRA bytes to stdout
    Mp4,      // FFmpeg pipe to MP4
    Png,      // PNG sequence
    Gif,      // Animated GIF
}

pub trait VideoEncoder {
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError>;
    fn finish(self) -> Result<(), VideoError>;
}

pub struct FfmpegEncoder { /* ... */ }
pub struct PngSequenceEncoder { /* ... */ }
pub struct GifEncoder { /* ... */ }
pub struct RawEncoder { /* ... */ }  // Just writes bytes to stdout

impl VideoFormat {
    pub fn encoder(&self, path: &str, width: u32, height: u32, fps: f64) -> Box<dyn VideoEncoder> {
        match self {
            VideoFormat::Raw => Box::new(RawEncoder::new()),
            VideoFormat::Mp4 => Box::new(FfmpegEncoder::new(path, width, height, fps)),
            VideoFormat::Png => Box::new(PngSequenceEncoder::new(path)),
            VideoFormat::Gif => Box::new(GifEncoder::new(path, width, height)),
        }
    }
}
```

---

## Dependencies to Add

```toml
[dependencies]
# For PNG sequence export
image = { version = "0.25", default-features = false, features = ["png"] }

# For GIF export  
gif = "0.13"

# Optional: For self-contained MP4 (requires ffmpeg libs)
# video-rs = { version = "0.10", features = ["ndarray"], optional = true }
# ndarray = { version = "0.16", optional = true }

[features]
# video-builtin = ["video-rs", "ndarray"]
```

---

## CLI Usage Examples

```bash
# Record 10 seconds of gameplay to MP4 (requires ffmpeg)
nes_main -H --rom game.nes --frames 600 --video gameplay.mp4

# Record with custom quality
nes_main -H --rom game.nes --frames 600 --video gameplay.mp4 --video-quality high

# Record raw for custom ffmpeg pipeline
nes_main -H --rom game.nes --frames 600 --video-format raw | \
  ffmpeg -f rawvideo -pixel_format bgra -video_size 256x240 \
         -framerate 60 -i - -c:v libx265 -crf 20 output.mp4

# Record PNG sequence
nes_main -H --rom game.nes --frames 600 --video-format png --video "frames/frame_%04d.png"

# Record animated GIF (short clips only)
nes_main -H --rom game.nes --frames 60 --video clip.gif --video-format gif

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

**FFmpeg pipe** is the recommended approach because:

1. **No build complexity** - No need to compile ffmpeg or link libraries
2. **Full codec support** - Any codec ffmpeg supports
3. **Industry standard** - Well-tested, documented
4. **Flexible** - Users can customize encoding
5. **Minimal code** - ~50 lines of Rust

The only requirement is that users have ffmpeg installed, which is standard for video work and available on all platforms via package managers.

For users who need a fully self-contained binary, `video-rs` can be added as an optional feature in the future.
