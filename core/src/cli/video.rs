//! Video export module for the NES emulator CLI.
//!
//! This module provides video encoding capabilities for exporting emulator
//! frame buffers to various video formats.
//!
//! # Supported Formats
//!
//! | Format | Implementation | Dependencies |
//! |--------|---------------|--------------|
//! | PNG sequence | Pure Rust (image crate) | None - self-contained |
//! | PPM sequence | Pure Rust | None - self-contained |
//! | MP4 | FFmpeg subprocess | FFmpeg installed |
//! | Raw | Writes raw BGRA bytes | None |
//!
//! # Usage Examples
//!
//! ```rust,ignore
//! use nes_core::cli::video::{VideoEncoder, VideoFormat, create_encoder};
//!
//! // Create an encoder for the specified format
//! let mut encoder = create_encoder(
//!     VideoFormat::Png,
//!     "frames/frame",
//!     256,
//!     240,
//!     60
//! )?;
//!
//! // Write frames
//! for frame in frames {
//!     encoder.write_frame(&frame)?;
//! }
//!
//! // Finish encoding
//! encoder.finish()?;
//! ```

use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use image::{ImageBuffer, Rgba, RgbaImage};

use super::args::VideoFormat;
use crate::emulation::messages::RgbColor;
use crate::frontend::util::FileType;

// =============================================================================
// Error Types
// =============================================================================

/// Video encoding error
#[derive(Debug)]
pub enum VideoError {
    /// FFmpeg is not installed or not found in PATH
    FfmpegNotFound,
    /// FFmpeg process failed
    FfmpegFailed(String),
    /// I/O error
    IoError(io::Error),
    /// Image encoding error
    ImageError(String),
    /// Invalid frame dimensions
    InvalidDimensions {
        expected: (u32, u32),
        got: (u32, u32),
    },
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoError::FfmpegNotFound => {
                write!(
                    f,
                    "FFmpeg not found. Please install FFmpeg for MP4 export, or use PNG/PPM format."
                )
            }
            VideoError::FfmpegFailed(msg) => write!(f, "FFmpeg encoding failed: {}", msg),
            VideoError::IoError(e) => write!(f, "I/O error: {}", e),
            VideoError::ImageError(e) => write!(f, "Image encoding error: {}", e),
            VideoError::InvalidDimensions {
                expected,
                got,
            } => {
                write!(
                    f,
                    "Invalid frame dimensions: expected {}x{}, got {}x{}",
                    expected.0, expected.1, got.0, got.1
                )
            }
        }
    }
}

impl std::error::Error for VideoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            VideoError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for VideoError {
    fn from(e: io::Error) -> Self {
        if e.kind() == io::ErrorKind::NotFound {
            VideoError::FfmpegNotFound
        } else {
            VideoError::IoError(e)
        }
    }
}

impl From<image::ImageError> for VideoError {
    fn from(e: image::ImageError) -> Self { VideoError::ImageError(e.to_string()) }
}

// =============================================================================
// Video Encoder Trait
// =============================================================================

/// Trait for video encoders.
///
/// Implement this trait to add support for new video formats.
pub trait VideoEncoder: Send {
    /// Write a frame to the video.
    ///
    /// The pixel buffer is in RGB format as (R, G, B) tuples,
    /// as a flat array of width × height values.
    fn write_frame(&mut self, pixel_buffer: &[RgbColor]) -> Result<(), VideoError>;
    
    /// Write a frame from raw BGRA bytes (4 bytes per pixel).
    ///
    /// This method is used when the GPU upscaler outputs BGRA directly.
    /// Default implementation converts to RGB and calls write_frame.
    fn write_frame_bgra_bytes(&mut self, bgra_bytes: &[u8]) -> Result<(), VideoError> {
        // Convert BGRA bytes to RgbColor array
        let rgb_pixels: Vec<RgbColor> = bgra_bytes
            .chunks_exact(4)
            .map(|chunk| (chunk[2], chunk[1], chunk[0]))  // BGRA -> RGB
            .collect();
        self.write_frame(&rgb_pixels)
    }

    /// Finish encoding and flush any remaining data.
    fn finish(&mut self) -> Result<(), VideoError>;

    /// Get the number of frames written so far.
    fn frames_written(&self) -> u64;
}

// =============================================================================
// Factory Function
// =============================================================================

/// Create a video encoder for the specified format.
///
/// # Arguments
///
/// * `format` - The video format to use
/// * `output_path` - Path to output file/directory
/// * `width` - Frame width in pixels
/// * `height` - Frame height in pixels
/// * `fps` - Frames per second
///
/// # Returns
///
/// A boxed video encoder, or an error if the encoder cannot be created.
pub fn create_encoder(
    format: VideoFormat,
    output_path: &Path,
    width: u32,
    height: u32,
    fps: f64,
) -> Result<Box<dyn VideoEncoder>, VideoError> {
    match format {
        VideoFormat::Png => Ok(Box::new(PngSequenceEncoder::new(
            output_path,
            width,
            height,
        )?)),
        VideoFormat::Ppm => Ok(Box::new(PpmSequenceEncoder::new(
            output_path,
            width,
            height,
        )?)),
        VideoFormat::Mp4 => Ok(Box::new(FfmpegMp4Encoder::new(
            output_path,
            width,
            height,
            fps,
        )?)),
        VideoFormat::Raw => Ok(Box::new(RawEncoder::new(width, height)?)),
    }
}

// =============================================================================
// PNG Sequence Encoder
// =============================================================================

/// Encoder that outputs a sequence of PNG images.
///
/// Output files are named: `{base_path}_{frame:06}.png`
///
/// This is a pure Rust implementation with no external dependencies.
pub struct PngSequenceEncoder {
    base_path: PathBuf,
    width: u32,
    height: u32,
    frame_count: u64,
}

impl PngSequenceEncoder {
    /// Create a new PNG sequence encoder.
    /// Creates the output directory if it doesn't exist.
    pub fn new(output_path: &Path, width: u32, height: u32) -> Result<Self, VideoError> {
        // Create output directory if needed
        if let Some(parent) = output_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }

        Ok(Self {
            base_path: output_path.to_path_buf(),
            width,
            height,
            frame_count: 0,
        })
    }

    /// Get the path for a specific frame number
    fn frame_path(&self, frame: u64) -> PathBuf {
        let stem = self
            .base_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "frame".to_string());
        let dir = self.base_path.parent().unwrap_or(Path::new("."));
        dir.join(format!("{}_{:06}.png", stem, frame))
    }
}

impl VideoEncoder for PngSequenceEncoder {
    fn write_frame(&mut self, pixel_buffer: &[RgbColor]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        // Convert RgbColor to RGBA image
        let img: RgbaImage = ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let (r, g, b) = pixel_buffer[(y * self.width + x) as usize];
            Rgba([r, g, b, 255])
        });

        // Save the frame
        let path = self.frame_path(self.frame_count);
        img.save(&path)?;

        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        // PNG sequence doesn't need finalization
        Ok(())
    }

    fn frames_written(&self) -> u64 { self.frame_count }
}

// =============================================================================
// PPM Sequence Encoder
// =============================================================================

/// Encoder that outputs a sequence of PPM images.
///
/// PPM is a simple format that's easy to convert with external tools.
/// This is a pure Rust implementation with no external dependencies.
pub struct PpmSequenceEncoder {
    base_path: PathBuf,
    width: u32,
    height: u32,
    frame_count: u64,
}

impl PpmSequenceEncoder {
    /// Create a new PPM sequence encoder.
    pub fn new(output_path: &Path, width: u32, height: u32) -> Result<Self, VideoError> {
        if let Some(parent) = output_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }

        Ok(Self {
            base_path: output_path.to_path_buf(),
            width,
            height,
            frame_count: 0,
        })
    }

    fn frame_path(&self, frame: u64) -> PathBuf {
        let stem = self
            .base_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "frame".to_string());
        let dir = self.base_path.parent().unwrap_or(Path::new("."));
        dir.join(format!("{}_{:06}.ppm", stem, frame))
    }
}

impl VideoEncoder for PpmSequenceEncoder {
    fn write_frame(&mut self, pixel_buffer: &[RgbColor]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        let path = self.frame_path(self.frame_count);
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);

        // Write PPM header
        writeln!(writer, "P6")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        // Write RGB data
        for &(r, g, b) in pixel_buffer {
            writer.write_all(&[r, g, b])?;
        }

        writer.flush()?;
        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> { Ok(()) }

    fn frames_written(&self) -> u64 { self.frame_count }
}

// =============================================================================
// FFmpeg MP4 Encoder
// =============================================================================

/// Encoder that pipes frames to FFmpeg for MP4 encoding.
///
/// Requires FFmpeg to be installed and available in PATH.
///
/// Uses H.264 encoding with yuv420p pixel format for broad compatibility.
pub struct FfmpegMp4Encoder {
    child: Option<Child>,
    stdin: Option<BufWriter<std::process::ChildStdin>>,
    stderr_path: Option<PathBuf>,
    width: u32,
    height: u32,
    frame_count: u64,
}

impl FfmpegMp4Encoder {
    /// Create a new FFmpeg MP4 encoder.
    ///
    /// # Errors
    ///
    /// Returns `VideoError::FfmpegNotFound` if FFmpeg is not installed.
    pub fn new(output_path: &Path, width: u32, height: u32, fps: f64) -> Result<Self, VideoError> {
        // Check if ffmpeg exists
        let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();

        match ffmpeg_check {
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Err(VideoError::FfmpegNotFound);
            }
            Err(e) => return Err(VideoError::IoError(e)),
            Ok(_) => {}
        }

        // Create output directory if needed
        if let Some(parent) = output_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }

        // Create a temporary file for ffmpeg stderr so we can read errors
        let stderr_path =
            std::env::temp_dir().join(format!("nes_ffmpeg_stderr_{}.log", std::process::id()));
        let stderr_file = File::create(&stderr_path)?;

        let path = output_path.with_extension(FileType::Mp4.get_default_extension());
        // Start ffmpeg process
        let mut child = Command::new("ffmpeg")
            .args([
                "-y", // Overwrite output
                "-f",
                "rawvideo", // Input format
                "-pixel_format",
                "bgra", // Input pixel format
                "-video_size",
                &format!("{}x{}", width, height), // Frame size
                "-framerate",
                &format!("{}", fps), // Frame rate
                "-i",
                "-", // Read from stdin
                "-c:v",
                "libx264", // H.264 codec
                "-preset",
                "fast", // Encoding speed
                "-vsync",
                "cfr",
                "-video_track_timescale",
                "39375000",
                "-crf",
                "16", // Quality (0-51, lower = better)
                "-pix_fmt",
                "yuv420p", // Output pixel format
                "-movflags",
                "+faststart", // Enable streaming
                "-f",
                "mp4", // Explicitly specify MP4 format (in case path has no extension)
                path.to_str().unwrap_or("output.mp4"),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::from(stderr_file))
            .spawn()
            .map_err(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    VideoError::FfmpegNotFound
                } else {
                    VideoError::IoError(e)
                }
            })?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| VideoError::FfmpegFailed("Failed to open FFmpeg stdin".to_string()))?;

        Ok(Self {
            child: Some(child),
            stdin: Some(BufWriter::new(stdin)),
            stderr_path: Some(stderr_path),
            width,
            height,
            frame_count: 0,
        })
    }
}

impl VideoEncoder for FfmpegMp4Encoder {
    fn write_frame(&mut self, pixel_buffer: &[RgbColor]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        // Convert RgbColor to BGRA bytes for FFmpeg
        // Input: (R, G, B)
        // Output: B, G, R, A (little-endian BGRA for FFmpeg)
        let bytes: Vec<u8> = pixel_buffer
            .iter()
            .flat_map(|&(r, g, b)| [b, g, r, 255u8])
            .collect();

        self.write_bgra_bytes_internal(&bytes)
    }
    
    fn write_frame_bgra_bytes(&mut self, bgra_bytes: &[u8]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height * 4) as usize;
        if bgra_bytes.len() != expected_size {
            return Err(VideoError::FfmpegFailed(format!(
                "Invalid BGRA byte count: expected {}, got {}",
                expected_size, bgra_bytes.len()
            )));
        }
        
        // BGRA bytes can be written directly to FFmpeg
        self.write_bgra_bytes_internal(bgra_bytes)
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        // Flush and drop stdin to signal EOF to ffmpeg
        if let Some(mut stdin) = self.stdin.take() {
            let _ = stdin.flush(); // Ignore flush errors, we're closing anyway
        }

        // Wait for FFmpeg to finish
        if let Some(mut child) = self.child.take() {
            let status = child.wait()?;
            if !status.success() {
                let ffmpeg_error = self.read_stderr_error();
                // Clean up stderr file
                if let Some(ref path) = self.stderr_path {
                    let _ = fs::remove_file(path);
                }
                return Err(VideoError::FfmpegFailed(format!(
                    "FFmpeg exited with status: {}. Error: {}",
                    status,
                    ffmpeg_error.unwrap_or_else(|| "Unknown error".to_string())
                )));
            }
        }

        // Clean up stderr file
        if let Some(ref path) = self.stderr_path {
            let _ = fs::remove_file(path);
        }

        Ok(())
    }

    fn frames_written(&self) -> u64 { self.frame_count }
}

impl FfmpegMp4Encoder {
    /// Internal helper to write BGRA bytes to FFmpeg stdin.
    fn write_bgra_bytes_internal(&mut self, bytes: &[u8]) -> Result<(), VideoError> {
        if let Some(ref mut stdin) = self.stdin
            && let Err(e) = stdin.write_all(bytes)
        {
            // If we get a broken pipe, FFmpeg may have exited early
            // Try to get the error message from stderr
            if e.kind() == io::ErrorKind::BrokenPipe {
                let ffmpeg_error = self.read_stderr_error();
                return Err(VideoError::FfmpegFailed(format!(
                    "FFmpeg pipe closed unexpectedly after {} frames. FFmpeg error: {}",
                    self.frame_count,
                    ffmpeg_error.unwrap_or_else(|| "Unknown error".to_string())
                )));
            }
            return Err(VideoError::IoError(e));
        }
        self.frame_count += 1;
        Ok(())
    }
    
    /// Read the FFmpeg stderr log file to get error details.
    fn read_stderr_error(&self) -> Option<String> {
        if let Some(ref path) = self.stderr_path
            && let Ok(content) = fs::read_to_string(path)
        {
            // Return the last few lines which usually contain the error
            let lines: Vec<&str> = content.lines().collect();
            let relevant = if lines.len() > 10 {
                lines[lines.len() - 10..].join("\n")
            } else {
                lines.join("\n")
            };
            if !relevant.is_empty() {
                return Some(relevant);
            }
        }
        None
    }
}

// =============================================================================
// Raw Encoder
// =============================================================================

/// Encoder that writes raw BGRA frames to stdout.
///
/// Useful for piping to external tools like FFmpeg.
pub struct RawEncoder {
    width: u32,
    height: u32,
    frame_count: u64,
    writer: BufWriter<io::Stdout>,
}

impl RawEncoder {
    /// Create a new raw encoder that writes to stdout.
    pub fn new(width: u32, height: u32) -> Result<Self, VideoError> {
        Ok(Self {
            width,
            height,
            frame_count: 0,
            writer: BufWriter::new(io::stdout()),
        })
    }
}

impl VideoEncoder for RawEncoder {
    fn write_frame(&mut self, pixel_buffer: &[RgbColor]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        // Convert RgbColor to BGRA bytes
        let bytes: Vec<u8> = pixel_buffer
            .iter()
            .flat_map(|&(r, g, b)| [b, g, r, 255u8])
            .collect();

        self.writer.write_all(&bytes)?;
        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        self.writer.flush()?;
        Ok(())
    }

    fn frames_written(&self) -> u64 { self.frame_count }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Encode collected frames to video.
///
/// This is the main entry point for video export from the CLI.
///
/// # Arguments
///
/// * `frames` - Vector of frame pixel buffers (RgbColor tuples)
/// * `format` - Output format
/// * `output_path` - Path to output file
/// * `width` - Frame width (default: 256 for NES)
/// * `height` - Frame height (default: 240 for NES)
/// * `fps` - Frame rate (default: 60 for NES)
///
/// # Returns
///
/// Number of frames written, or an error.
pub fn encode_frames(
    frames: &[Vec<RgbColor>],
    format: VideoFormat,
    output_path: &Path,
    width: u32,
    height: u32,
    fps: f64,
) -> Result<u64, VideoError> {
    let mut encoder = create_encoder(format, output_path, width, height, fps)?;

    for frame in frames {
        encoder.write_frame(frame)?;
    }

    encoder.finish()?;
    Ok(encoder.frames_written())
}

/// Encode collected frames to video with optional upscaling.
///
/// This version supports pseudo-bandlimited pixel art upscaling before encoding.
///
/// # Arguments
///
/// * `frames` - Vector of frame pixel buffers (RgbColor tuples)
/// * `format` - Output format
/// * `output_path` - Path to output file
/// * `src_width` - Source frame width (default: 256 for NES)
/// * `src_height` - Source frame height (default: 240 for NES)
/// * `resolution` - Target output resolution
/// * `fps` - Frame rate (default: 60 for NES)
///
/// # Returns
///
/// Number of frames written, or an error.
pub fn encode_frames_with_upscale(
    frames: &[Vec<RgbColor>],
    format: VideoFormat,
    output_path: &Path,
    src_width: u32,
    src_height: u32,
    resolution: &super::upscale::VideoResolution,
    fps: f64,
) -> Result<u64, VideoError> {
    use super::upscale::{PixelArtUpscaler, VideoResolution};

    let (dst_width, dst_height) = resolution.dimensions(src_width, src_height);

    // If native resolution, don't upscale
    if *resolution == VideoResolution::Native
        || (dst_width == src_width && dst_height == src_height)
    {
        return encode_frames(frames, format, output_path, src_width, src_height, fps);
    }

    // Create upscaler and encoder
    let upscaler = PixelArtUpscaler::new(src_width, src_height, dst_width, dst_height);
    let mut encoder = create_encoder(format, output_path, dst_width, dst_height, fps)?;

    // Upscale and encode each frame
    for frame in frames {
        let upscaled = upscaler.upscale_rgb(frame);
        encoder.write_frame(&upscaled)?;
    }

    encoder.finish()?;
    Ok(encoder.frames_written())
}

/// Check if FFmpeg is available for MP4 encoding.
pub fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// =============================================================================
// Streaming Video Encoder
// =============================================================================

/// A streaming video encoder that handles upscaling and encoding in one step.
///
/// This encoder is designed for use during emulation - frames are written
/// immediately as they are generated, without buffering all frames in memory.
///
/// # Performance
///
/// - Upscaling uses rayon parallel processing (~10-15ms per 1080p frame)
/// - Frames are streamed directly to the encoder
/// - Memory usage is O(1) per frame instead of O(n) for all frames
/// Upscaling backend
enum UpscaleBackend {
    /// No upscaling needed
    None,
    /// CPU-based upscaling with rayon parallelism
    Cpu(super::upscale::PixelArtUpscaler),
    /// GPU-based upscaling with wgpu compute shaders
    Gpu(super::gpu_upscale::GpuUpscaler),
}

/// Combined video encoder with optional upscaling.
///
/// This encoder handles both upscaling and encoding in a streaming fashion,
/// writing frames directly as they are received rather than buffering them.
pub struct StreamingVideoEncoder {
    encoder: Box<dyn VideoEncoder>,
    upscale_backend: UpscaleBackend,
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
}

impl StreamingVideoEncoder {
    /// Create a new streaming encoder.
    ///
    /// # Arguments
    ///
    /// * `format` - Output video format
    /// * `output_path` - Path to output file
    /// * `src_width` - Source frame width (NES: 256)
    /// * `src_height` - Source frame height (NES: 240)
    /// * `resolution` - Target output resolution
    /// * `fps` - Frame rate
    /// * `no_gpu` - If true, disable GPU acceleration (use CPU)
    pub fn new(
        format: VideoFormat,
        output_path: &Path,
        src_width: u32,
        src_height: u32,
        resolution: &super::upscale::VideoResolution,
        fps: f64,
        no_gpu: bool,
    ) -> Result<Self, VideoError> {
        use super::gpu_upscale::GpuUpscaler;
        use super::upscale::{PixelArtUpscaler, VideoResolution};

        let (dst_width, dst_height) = resolution.dimensions(src_width, src_height);
        
        // Get effective source dimensions for PAR correction
        let (effective_width, effective_height) = resolution.effective_source_dimensions(src_width, src_height);

        // Determine upscaling backend
        let upscale_backend = if *resolution == VideoResolution::Native
            || (dst_width == src_width && dst_height == src_height)
        {
            UpscaleBackend::None
        } else if no_gpu {
            // User explicitly requested CPU
            eprintln!("Using CPU upscaling (rayon parallel) {}x{} -> {}x{}", 
                     src_width, src_height, dst_width, dst_height);
            UpscaleBackend::Cpu(PixelArtUpscaler::new(
                src_width, src_height, dst_width, dst_height,
            ))
        } else {
            // Try GPU first, fall back to CPU
            match GpuUpscaler::try_new(
                src_width, src_height, dst_width, dst_height,
                effective_width as f32, effective_height as f32,
            ) {
                Some(gpu_upscaler) => {
                    eprintln!("Using GPU upscaling (wgpu compute) {}x{} -> {}x{}", 
                             src_width, src_height, dst_width, dst_height);
                    UpscaleBackend::Gpu(gpu_upscaler)
                }
                None => {
                    eprintln!("GPU not available, falling back to CPU upscaling {}x{} -> {}x{}", 
                             src_width, src_height, dst_width, dst_height);
                    UpscaleBackend::Cpu(PixelArtUpscaler::new(
                        src_width, src_height, dst_width, dst_height,
                    ))
                }
            }
        };

        let encoder = create_encoder(format, output_path, dst_width, dst_height, fps)?;

        Ok(Self {
            encoder,
            upscale_backend,
            src_width,
            src_height,
            dst_width,
            dst_height,
        })
    }

    /// Write a single frame, with upscaling if configured.
    ///
    /// This method handles upscaling and immediately writes to the underlying encoder.
    pub fn write_frame(&mut self, frame: &[RgbColor]) -> Result<(), VideoError> {
        match &self.upscale_backend {
            UpscaleBackend::None => {
                // No upscaling needed
                self.encoder.write_frame(frame)
            }
            UpscaleBackend::Cpu(upscaler) => {
                // CPU upscaling via rayon
                let upscaled = upscaler.upscale_rgb(frame);
                self.encoder.write_frame(&upscaled)
            }
            UpscaleBackend::Gpu(gpu_upscaler) => {
                // GPU upscaling - returns BGRA bytes directly
                let bgra_bytes = gpu_upscaler.upscale(frame);
                self.encoder.write_frame_bgra_bytes(&bgra_bytes)
            }
        }
    }

    /// Finish encoding and finalize the output file.
    pub fn finish(&mut self) -> Result<(), VideoError> { self.encoder.finish() }

    /// Get the number of frames written so far.
    pub fn frames_written(&self) -> u64 { self.encoder.frames_written() }

    /// Get the source dimensions.
    pub fn source_dimensions(&self) -> (u32, u32) { (self.src_width, self.src_height) }

    /// Get the output dimensions (after upscaling).
    pub fn output_dimensions(&self) -> (u32, u32) {
        (self.dst_width, self.dst_height)
    }

    /// Check if upscaling is enabled.
    pub fn is_upscaling(&self) -> bool { !matches!(self.upscale_backend, UpscaleBackend::None) }
    
    /// Check if GPU upscaling is being used.
    pub fn is_using_gpu(&self) -> bool { matches!(self.upscale_backend, UpscaleBackend::Gpu(_)) }
}
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_error_display() {
        let err = VideoError::FfmpegNotFound;
        assert!(err.to_string().contains("FFmpeg not found"));

        let err = VideoError::InvalidDimensions {
            expected: (256, 240),
            got: (128, 120),
        };
        assert!(err.to_string().contains("256x240"));
        assert!(err.to_string().contains("128x120"));
    }

    #[test]
    fn test_png_encoder_frame_path() {
        let encoder = PngSequenceEncoder::new(Path::new("/tmp/test/frames"), 256, 240).unwrap();
        let path = encoder.frame_path(0);
        assert!(path.to_string_lossy().contains("frames_000000.png"));

        let path = encoder.frame_path(42);
        assert!(path.to_string_lossy().contains("frames_000042.png"));
    }

    #[test]
    fn test_ppm_encoder_frame_path() {
        let encoder = PpmSequenceEncoder::new(Path::new("/tmp/test/output"), 256, 240).unwrap();
        let path = encoder.frame_path(123);
        assert!(path.to_string_lossy().contains("output_000123.ppm"));
    }

    #[test]
    fn test_ffmpeg_availability_check() {
        // This test just checks that the function doesn't panic
        let _available = is_ffmpeg_available();
    }

    #[test]
    fn test_invalid_frame_dimensions() {
        let mut encoder =
            PpmSequenceEncoder::new(Path::new("/tmp/test_invalid"), 256, 240).unwrap();

        // Wrong size frame
        let bad_frame: Vec<RgbColor> = vec![(0, 0, 0); 100]; // Much too small
        let result = encoder.write_frame(&bad_frame);

        assert!(result.is_err());
        if let Err(VideoError::InvalidDimensions {
            expected, ..
        }) = result
        {
            assert_eq!(expected, (256, 240));
        } else {
            panic!("Expected InvalidDimensions error");
        }
    }
}
