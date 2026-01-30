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
    /// The pixel buffer is in ARGB format (0xAARRGGBB) as a flat array
    /// of width × height u32 values.
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError>;

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
    fps: u32,
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
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        // Convert ARGB u32 to RGBA image
        let img: RgbaImage = ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let pixel = pixel_buffer[(y * self.width + x) as usize];
            Rgba([
                ((pixel >> 16) & 0xFF) as u8, // R
                ((pixel >> 8) & 0xFF) as u8,  // G
                (pixel & 0xFF) as u8,         // B
                ((pixel >> 24) & 0xFF) as u8, // A
            ])
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
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError> {
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
        for pixel in pixel_buffer {
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
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
    pub fn new(output_path: &Path, width: u32, height: u32, fps: u32) -> Result<Self, VideoError> {
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
                "-crf",
                "18", // Quality (0-51, lower = better)
                "-pix_fmt",
                "yuv420p", // Output pixel format
                "-movflags",
                "+faststart", // Enable streaming
                "-f",
                "mp4", // Explicitly specify MP4 format (in case path has no extension)
                output_path.to_str().unwrap_or("output.mp4"),
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
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        // Convert ARGB u32 to BGRA bytes for FFmpeg
        // Input: 0xAARRGGBB
        // Output: B, G, R, A (little-endian BGRA for FFmpeg)
        let bytes: Vec<u8> = pixel_buffer
            .iter()
            .flat_map(|&pixel| {
                [
                    (pixel & 0xFF) as u8,         // B
                    ((pixel >> 8) & 0xFF) as u8,  // G
                    ((pixel >> 16) & 0xFF) as u8, // R
                    ((pixel >> 24) & 0xFF) as u8, // A
                ]
            })
            .collect();

        if let Some(ref mut stdin) = self.stdin
            && let Err(e) = stdin.write_all(&bytes)
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
    fn write_frame(&mut self, pixel_buffer: &[u32]) -> Result<(), VideoError> {
        let expected_size = (self.width * self.height) as usize;
        if pixel_buffer.len() != expected_size {
            let got_height = pixel_buffer.len() / self.width as usize;
            return Err(VideoError::InvalidDimensions {
                expected: (self.width, self.height),
                got: (self.width, got_height as u32),
            });
        }

        // Convert ARGB u32 to BGRA bytes
        let bytes: Vec<u8> = pixel_buffer
            .iter()
            .flat_map(|&pixel| {
                [
                    (pixel & 0xFF) as u8,         // B
                    ((pixel >> 8) & 0xFF) as u8,  // G
                    ((pixel >> 16) & 0xFF) as u8, // R
                    ((pixel >> 24) & 0xFF) as u8, // A
                ]
            })
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
/// * `frames` - Vector of frame pixel buffers (ARGB u32)
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
    frames: &[Vec<u32>],
    format: VideoFormat,
    output_path: &Path,
    width: u32,
    height: u32,
    fps: u32,
) -> Result<u64, VideoError> {
    let mut encoder = create_encoder(format, output_path, width, height, fps)?;

    for frame in frames {
        encoder.write_frame(frame)?;
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
        let bad_frame = vec![0u32; 100]; // Much too small
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
