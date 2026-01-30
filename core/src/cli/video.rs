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
//! # Scaling
//!
//! Video scaling is handled natively by FFmpeg using the nearest neighbor
//! filter, which preserves sharp pixel edges for retro games.

use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use image::{ImageBuffer, Rgba, RgbaImage};

use super::args::VideoFormat;
use crate::emulation::messages::RgbColor;
use crate::frontend::util::FileType;

// =============================================================================
// Video Resolution
// =============================================================================

/// Target video resolution for output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoResolution {
    /// Native resolution (256x240 for NES)
    Native,
    /// Integer scale (2x, 3x, 4x, etc.)
    IntegerScale(u32),
    /// 720p (1280x720) - fit within bounds
    Hd720,
    /// 1080p (1920x1080) - fit within bounds
    Hd1080,
    /// 4K (3840x2160) - fit within bounds
    Uhd4k,
    /// Custom resolution
    Custom(u32, u32),
}

impl VideoResolution {
    /// Parse a video resolution string.
    ///
    /// Supported formats:
    /// - "native" - Native resolution
    /// - "2x", "3x", "4x" - Integer scales
    /// - "720p", "1080p", "4k" - Standard resolutions
    /// - "WxH" or "WIDTHxHEIGHT" - Custom resolution (e.g., "1920x1080")
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.to_lowercase();
        match s.as_str() {
            "native" | "1x" => Ok(VideoResolution::Native),
            "2x" => Ok(VideoResolution::IntegerScale(2)),
            "3x" => Ok(VideoResolution::IntegerScale(3)),
            "4x" => Ok(VideoResolution::IntegerScale(4)),
            "5x" => Ok(VideoResolution::IntegerScale(5)),
            "6x" => Ok(VideoResolution::IntegerScale(6)),
            "720p" | "hd" => Ok(VideoResolution::Hd720),
            "1080p" | "fullhd" | "fhd" => Ok(VideoResolution::Hd1080),
            "4k" | "uhd" | "2160p" => Ok(VideoResolution::Uhd4k),
            _ => {
                // Try to parse as WxH
                if let Some((w, h)) = s.split_once('x') {
                    let width = w
                        .trim()
                        .parse()
                        .map_err(|_| format!("Invalid width: {}", w))?;
                    let height = h
                        .trim()
                        .parse()
                        .map_err(|_| format!("Invalid height: {}", h))?;
                    Ok(VideoResolution::Custom(width, height))
                } else {
                    Err(format!(
                        "Unknown resolution: '{}'. Try: native, 2x, 3x, 4x, 720p, 1080p, 4k, or WxH",
                        s
                    ))
                }
            }
        }
    }

    /// Calculate the output dimensions for a given source size.
    ///
    /// For preset resolutions (720p, 1080p, 4k), the output is scaled to fit
    /// within the target while maintaining aspect ratio with the NES PAR (8:7).
    pub fn dimensions(&self, src_width: u32, src_height: u32) -> (u32, u32) {
        // NES pixel aspect ratio: 8:7 (pixels are slightly wider than tall)
        const NES_PAR: f64 = 8.0 / 7.0;

        match self {
            VideoResolution::Native => (src_width, src_height),
            VideoResolution::IntegerScale(scale) => (src_width * scale, src_height * scale),
            VideoResolution::Hd720 => {
                fit_to_bounds(src_width, src_height, 1280, 720, NES_PAR)
            }
            VideoResolution::Hd1080 => {
                fit_to_bounds(src_width, src_height, 1920, 1080, NES_PAR)
            }
            VideoResolution::Uhd4k => {
                fit_to_bounds(src_width, src_height, 3840, 2160, NES_PAR)
            }
            VideoResolution::Custom(w, h) => (*w, *h),
        }
    }
}

/// Fit source dimensions to target bounds while maintaining aspect ratio.
fn fit_to_bounds(
    src_width: u32,
    src_height: u32,
    max_width: u32,
    max_height: u32,
    par: f64,
) -> (u32, u32) {
    // Calculate the maximum integer scale that fits within bounds
    let scale_x = max_width as f64 / (src_width as f64 * par);
    let scale_y = max_height as f64 / src_height as f64;
    let scale = scale_x.min(scale_y);

    // Use integer scale for clean pixel scaling
    let int_scale = scale.floor() as u32;
    let int_scale = int_scale.max(1); // At least 1x

    // Calculate output dimensions
    let out_width = (src_width as f64 * par * int_scale as f64).round() as u32;
    let out_height = src_height * int_scale;

    // Ensure dimensions are even (required for many video codecs)
    let out_width = (out_width + 1) & !1;
    let out_height = (out_height + 1) & !1;

    (out_width, out_height)
}

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
            VideoError::InvalidDimensions { expected, got } => {
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
    fn from(e: image::ImageError) -> Self {
        VideoError::ImageError(e.to_string())
    }
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
/// For MP4 format with scaling, use `create_encoder_with_scale` instead.
pub fn create_encoder(
    format: VideoFormat,
    output_path: &Path,
    width: u32,
    height: u32,
    fps: f64,
) -> Result<Box<dyn VideoEncoder>, VideoError> {
    match format {
        VideoFormat::Png => Ok(Box::new(PngSequenceEncoder::new(
            output_path, width, height,
        )?)),
        VideoFormat::Ppm => Ok(Box::new(PpmSequenceEncoder::new(
            output_path, width, height,
        )?)),
        VideoFormat::Mp4 => Ok(Box::new(FfmpegMp4Encoder::new(
            output_path, width, height, fps, None,
        )?)),
        VideoFormat::Raw => Ok(Box::new(RawEncoder::new(width, height)?)),
    }
}

/// Create an MP4 encoder with FFmpeg native nearest-neighbor scaling.
///
/// This passes scaling to FFmpeg using `-vf scale=W:H:flags=neighbor`,
/// which is efficient and produces sharp pixel edges.
pub fn create_encoder_with_scale(
    output_path: &Path,
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
    fps: f64,
) -> Result<Box<dyn VideoEncoder>, VideoError> {
    Ok(Box::new(FfmpegMp4Encoder::new(
        output_path,
        src_width,
        src_height,
        fps,
        Some((dst_width, dst_height)),
    )?))
}

// =============================================================================
// PNG Sequence Encoder
// =============================================================================

/// Encoder that outputs a sequence of PNG images.
pub struct PngSequenceEncoder {
    base_path: PathBuf,
    width: u32,
    height: u32,
    frame_count: u64,
}

impl PngSequenceEncoder {
    /// Create a new PNG sequence encoder.
    pub fn new(output_path: &Path, width: u32, height: u32) -> Result<Self, VideoError> {
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                fs::create_dir_all(parent)?;
            }
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

        let img: RgbaImage = ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let (r, g, b) = pixel_buffer[(y * self.width + x) as usize];
            Rgba([r, g, b, 255])
        });

        let path = self.frame_path(self.frame_count);
        img.save(&path)?;

        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        Ok(())
    }

    fn frames_written(&self) -> u64 {
        self.frame_count
    }
}

// =============================================================================
// PPM Sequence Encoder
// =============================================================================

/// Encoder that outputs a sequence of PPM images.
pub struct PpmSequenceEncoder {
    base_path: PathBuf,
    width: u32,
    height: u32,
    frame_count: u64,
}

impl PpmSequenceEncoder {
    /// Create a new PPM sequence encoder.
    pub fn new(output_path: &Path, width: u32, height: u32) -> Result<Self, VideoError> {
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                fs::create_dir_all(parent)?;
            }
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

        writeln!(writer, "P6")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for &(r, g, b) in pixel_buffer {
            writer.write_all(&[r, g, b])?;
        }

        writer.flush()?;
        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        Ok(())
    }

    fn frames_written(&self) -> u64 {
        self.frame_count
    }
}

// =============================================================================
// FFmpeg MP4 Encoder
// =============================================================================

/// Encoder that pipes frames to FFmpeg for MP4 encoding.
///
/// Supports native nearest-neighbor scaling via FFmpeg's scale filter.
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
    /// If `scale_to` is provided, FFmpeg will scale to that resolution using
    /// nearest-neighbor interpolation (sharp pixel edges).
    pub fn new(
        output_path: &Path,
        width: u32,
        height: u32,
        fps: f64,
        scale_to: Option<(u32, u32)>,
    ) -> Result<Self, VideoError> {
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
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let stderr_path =
            std::env::temp_dir().join(format!("nes_ffmpeg_stderr_{}.log", std::process::id()));
        let stderr_file = File::create(&stderr_path)?;

        let path = output_path.with_extension(FileType::Mp4.get_default_extension());

        // Build FFmpeg arguments
        let mut args = vec![
            "-y".to_string(),
            "-f".to_string(),
            "rawvideo".to_string(),
            "-pixel_format".to_string(),
            "bgra".to_string(),
            "-video_size".to_string(),
            format!("{}x{}", width, height),
            "-framerate".to_string(),
            format!("{}", fps),
            "-i".to_string(),
            "-".to_string(),
        ];

        // Add scaling filter if requested (nearest neighbor for sharp pixels)
        if let Some((dst_w, dst_h)) = scale_to {
            if dst_w != width || dst_h != height {
                eprintln!(
                    "FFmpeg scaling {}x{} -> {}x{} (nearest neighbor)",
                    width, height, dst_w, dst_h
                );
                args.extend([
                    "-vf".to_string(),
                    format!("scale={}:{}:flags=neighbor", dst_w, dst_h),
                ]);
            }
        }

        // Encoder settings
        args.extend([
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "fast".to_string(),
            "-crf".to_string(),
            "16".to_string(),
            "-vsync".to_string(),
            "cfr".to_string(),
            "-video_track_timescale".to_string(),
            "39375000".to_string(),
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            "-movflags".to_string(),
            "+faststart".to_string(),
            "-f".to_string(),
            "mp4".to_string(),
            path.to_str().unwrap_or("output.mp4").to_string(),
        ]);

        let mut child = Command::new("ffmpeg")
            .args(&args)
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

        let stdin = child.stdin.take().map(BufWriter::new);

        Ok(Self {
            child: Some(child),
            stdin,
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

        // Convert RGB to BGRA
        let mut bgra_buffer = Vec::with_capacity(pixel_buffer.len() * 4);
        for &(r, g, b) in pixel_buffer {
            bgra_buffer.extend_from_slice(&[b, g, r, 255]);
        }

        if let Some(ref mut stdin) = self.stdin {
            stdin.write_all(&bgra_buffer)?;
        }

        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        // Close stdin to signal EOF to ffmpeg
        self.stdin.take();

        // Wait for ffmpeg to finish
        if let Some(mut child) = self.child.take() {
            let status = child.wait()?;
            if !status.success() {
                let stderr_content = if let Some(ref path) = self.stderr_path {
                    fs::read_to_string(path).unwrap_or_default()
                } else {
                    String::new()
                };
                return Err(VideoError::FfmpegFailed(format!(
                    "FFmpeg exited with status {}: {}",
                    status,
                    stderr_content.lines().take(10).collect::<Vec<_>>().join("\n")
                )));
            }
        }

        // Clean up stderr file
        if let Some(ref path) = self.stderr_path {
            let _ = fs::remove_file(path);
        }

        Ok(())
    }

    fn frames_written(&self) -> u64 {
        self.frame_count
    }
}

impl Drop for FfmpegMp4Encoder {
    fn drop(&mut self) {
        // Ensure stdin is closed
        self.stdin.take();

        // Try to wait for child process
        if let Some(mut child) = self.child.take() {
            let _ = child.wait();
        }

        // Clean up stderr file
        if let Some(ref path) = self.stderr_path {
            let _ = fs::remove_file(path);
        }
    }
}

// =============================================================================
// Raw Encoder
// =============================================================================

/// Encoder that outputs raw BGRA frames.
pub struct RawEncoder {
    width: u32,
    height: u32,
    frame_count: u64,
}

impl RawEncoder {
    /// Create a new raw encoder.
    pub fn new(width: u32, height: u32) -> Result<Self, VideoError> {
        Ok(Self {
            width,
            height,
            frame_count: 0,
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

        // Convert RGB to BGRA and write to stdout
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for &(r, g, b) in pixel_buffer {
            handle.write_all(&[b, g, r, 255])?;
        }

        self.frame_count += 1;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), VideoError> {
        io::stdout().flush()?;
        Ok(())
    }

    fn frames_written(&self) -> u64 {
        self.frame_count
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Encode collected frames to video.
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

/// A streaming video encoder that handles scaling via FFmpeg.
///
/// This encoder is designed for use during emulation - frames are written
/// immediately as they are generated, without buffering all frames in memory.
///
/// Scaling is handled natively by FFmpeg using nearest-neighbor interpolation,
/// which is efficient and produces sharp pixel edges.
pub struct StreamingVideoEncoder {
    encoder: Box<dyn VideoEncoder>,
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
}

impl StreamingVideoEncoder {
    /// Create a new streaming encoder.
    ///
    /// If the resolution specifies scaling, FFmpeg will handle it natively
    /// using nearest-neighbor interpolation.
    pub fn new(
        format: VideoFormat,
        output_path: &Path,
        src_width: u32,
        src_height: u32,
        resolution: &VideoResolution,
        fps: f64,
    ) -> Result<Self, VideoError> {
        let (dst_width, dst_height) = resolution.dimensions(src_width, src_height);

        let encoder: Box<dyn VideoEncoder> = match format {
            VideoFormat::Mp4 => {
                if dst_width != src_width || dst_height != src_height {
                    // Use FFmpeg native scaling
                    Box::new(FfmpegMp4Encoder::new(
                        output_path,
                        src_width,
                        src_height,
                        fps,
                        Some((dst_width, dst_height)),
                    )?)
                } else {
                    Box::new(FfmpegMp4Encoder::new(
                        output_path,
                        src_width,
                        src_height,
                        fps,
                        None,
                    )?)
                }
            }
            _ => {
                // For non-MP4 formats, no scaling (use native resolution)
                if dst_width != src_width || dst_height != src_height {
                    eprintln!(
                        "Warning: Scaling only supported for MP4 format. Using native resolution."
                    );
                }
                create_encoder(format, output_path, src_width, src_height, fps)?
            }
        };

        Ok(Self {
            encoder,
            src_width,
            src_height,
            dst_width,
            dst_height,
        })
    }

    /// Write a single frame.
    pub fn write_frame(&mut self, frame: &[RgbColor]) -> Result<(), VideoError> {
        self.encoder.write_frame(frame)
    }

    /// Finish encoding and finalize the output file.
    pub fn finish(&mut self) -> Result<(), VideoError> {
        self.encoder.finish()
    }

    /// Get the number of frames written so far.
    pub fn frames_written(&self) -> u64 {
        self.encoder.frames_written()
    }

    /// Get the source dimensions.
    pub fn source_dimensions(&self) -> (u32, u32) {
        (self.src_width, self.src_height)
    }

    /// Get the output dimensions (after scaling).
    pub fn output_dimensions(&self) -> (u32, u32) {
        (self.dst_width, self.dst_height)
    }

    /// Check if scaling is enabled.
    pub fn is_scaling(&self) -> bool {
        self.dst_width != self.src_width || self.dst_height != self.src_height
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_resolution_parse() {
        assert_eq!(VideoResolution::parse("native").unwrap(), VideoResolution::Native);
        assert_eq!(VideoResolution::parse("1x").unwrap(), VideoResolution::Native);
        assert_eq!(VideoResolution::parse("2x").unwrap(), VideoResolution::IntegerScale(2));
        assert_eq!(VideoResolution::parse("4x").unwrap(), VideoResolution::IntegerScale(4));
        assert_eq!(VideoResolution::parse("720p").unwrap(), VideoResolution::Hd720);
        assert_eq!(VideoResolution::parse("1080p").unwrap(), VideoResolution::Hd1080);
        assert_eq!(VideoResolution::parse("4k").unwrap(), VideoResolution::Uhd4k);
        assert_eq!(
            VideoResolution::parse("1920x1080").unwrap(),
            VideoResolution::Custom(1920, 1080)
        );
    }

    #[test]
    fn test_video_resolution_dimensions() {
        // Native
        assert_eq!(VideoResolution::Native.dimensions(256, 240), (256, 240));

        // Integer scale
        assert_eq!(VideoResolution::IntegerScale(2).dimensions(256, 240), (512, 480));
        assert_eq!(VideoResolution::IntegerScale(4).dimensions(256, 240), (1024, 960));

        // 1080p should fit within bounds
        let (w, h) = VideoResolution::Hd1080.dimensions(256, 240);
        assert!(w <= 1920);
        assert!(h <= 1080);
    }

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
        let _available = is_ffmpeg_available();
    }

    #[test]
    fn test_invalid_frame_dimensions() {
        let mut encoder =
            PpmSequenceEncoder::new(Path::new("/tmp/test_invalid"), 256, 240).unwrap();

        let bad_frame: Vec<RgbColor> = vec![(0, 0, 0); 100];
        let result = encoder.write_frame(&bad_frame);

        assert!(result.is_err());
        if let Err(VideoError::InvalidDimensions { expected, .. }) = result {
            assert_eq!(expected, (256, 240));
        } else {
            panic!("Expected InvalidDimensions error");
        }
    }
}
