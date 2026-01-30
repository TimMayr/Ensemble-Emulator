//! Pseudo-bandlimited pixel art upscaling module.
//!
//! This module implements the pseudo-bandlimited pixel art filtering algorithm
//! described by Hans-Kristian Arntzen (themaister) for smooth upscaling of
//! pixel art without traditional bilinear blurring.
//!
//! # Performance
//!
//! This module uses rayon for parallel processing. Upscaling is performed
//! in parallel rows for optimal performance on multi-core systems.
//!
//! # Algorithm Overview
//!
//! The algorithm works by:
//! 1. Computing the fractional position within each source pixel
//! 2. Using a smoothstep function to create smooth transitions at pixel boundaries
//! 3. Blending between neighboring pixels based on the smoothed fractional position
//!
//! This produces crisp pixel art at integer scales while providing smooth
//! anti-aliased edges at non-integer scales.
//!
//! # NES Pixel Aspect Ratio (PAR)
//!
//! The NES outputs 256x240 pixels, but on CRT TVs these pixels weren't square.
//! The pixel aspect ratio (PAR) was approximately 8:7, meaning pixels were wider
//! than tall. This module applies PAR correction for non-integer scale outputs
//! to ensure the correct display aspect ratio.
//!
//! - Storage Aspect Ratio (SAR): 256:240 = 16:15
//! - Pixel Aspect Ratio (PAR): 8:7
//! - Display Aspect Ratio (DAR): ~4:3 (when PAR is applied)
//!
//! Integer scales (2x, 3x, 4x) do NOT apply PAR correction to preserve
//! pixel-perfect output for emulation purists.
//!
//! # Reference
//!
//! Based on: "Pseudo-bandlimited pixel art filtering in 3D – a mathematical derivation"
//! by Hans-Kristian Arntzen (themaister)

use rayon::prelude::*;

use crate::emulation::messages::RgbColor;

/// NES Pixel Aspect Ratio (PAR) numerator.
///
/// The NES outputs non-square pixels on CRT TVs. The pixel aspect ratio
/// is approximately 8:7, meaning pixels are wider than they are tall.
pub const NES_PAR_WIDTH: u32 = 8;

/// NES Pixel Aspect Ratio (PAR) denominator.
pub const NES_PAR_HEIGHT: u32 = 7;

/// NES PAR as a floating point ratio (8/7 ≈ 1.143).
pub const NES_PAR: f32 = NES_PAR_WIDTH as f32 / NES_PAR_HEIGHT as f32;

/// Smoothstep function for smooth interpolation.
/// Maps input from [0, 1] to [0, 1] with smooth derivatives at boundaries.
#[inline]
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Compute the blend weight using pseudo-bandlimited filtering.
///
/// This creates smooth transitions at pixel boundaries while keeping
/// pixels sharp in their interior. The filter width is inversely
/// proportional to the scale factor.
#[inline]
fn smoothstep_blend(frac: f32, scale: f32) -> f32 {
    // The filter width determines the transition zone size
    // At higher scales, smaller transition zone = sharper pixels
    // At lower scales, larger transition zone = smoother AA
    let filter_width = (1.0 / scale).min(0.5);

    // Use smoothstep to create smooth transition from 0 to 1
    // The transition zone is centered at 0.5
    smoothstep(filter_width, 1.0 - filter_width, frac)
}

/// Bilinear interpolation between four colors.
#[inline]
fn bilinear_blend(
    c00: RgbColor,
    c10: RgbColor,
    c01: RgbColor,
    c11: RgbColor,
    wx: f32,
    wy: f32,
) -> RgbColor {
    // Interpolate horizontally first
    let r0 = c00.0 as f32 * (1.0 - wx) + c10.0 as f32 * wx;
    let g0 = c00.1 as f32 * (1.0 - wx) + c10.1 as f32 * wx;
    let b0 = c00.2 as f32 * (1.0 - wx) + c10.2 as f32 * wx;

    let r1 = c01.0 as f32 * (1.0 - wx) + c11.0 as f32 * wx;
    let g1 = c01.1 as f32 * (1.0 - wx) + c11.1 as f32 * wx;
    let b1 = c01.2 as f32 * (1.0 - wx) + c11.2 as f32 * wx;

    // Then interpolate vertically
    let r = r0 * (1.0 - wy) + r1 * wy;
    let g = g0 * (1.0 - wy) + g1 * wy;
    let b = b0 * (1.0 - wy) + b1 * wy;

    (
        r.clamp(0.0, 255.0) as u8,
        g.clamp(0.0, 255.0) as u8,
        b.clamp(0.0, 255.0) as u8,
    )
}

/// Pseudo-bandlimited pixel art upscaler.
///
/// This upscaler produces sharp pixels at integer scales while providing
/// smooth anti-aliased edges at non-integer scales.
pub struct PixelArtUpscaler {
    /// Source width
    pub src_width: u32,
    /// Source height
    pub src_height: u32,
    /// Destination width
    pub dst_width: u32,
    /// Destination height
    pub dst_height: u32,
    /// Horizontal scale factor
    scale_x: f32,
    /// Vertical scale factor
    scale_y: f32,
}

impl PixelArtUpscaler {
    /// Create a new upscaler with specified source and destination dimensions.
    pub fn new(src_width: u32, src_height: u32, dst_width: u32, dst_height: u32) -> Self {
        Self {
            src_width,
            src_height,
            dst_width,
            dst_height,
            scale_x: dst_width as f32 / src_width as f32,
            scale_y: dst_height as f32 / src_height as f32,
        }
    }

    /// Create an upscaler that maintains aspect ratio and fits within max dimensions.
    pub fn fit_aspect_ratio(
        src_width: u32,
        src_height: u32,
        max_width: u32,
        max_height: u32,
    ) -> Self {
        let scale_x = max_width as f32 / src_width as f32;
        let scale_y = max_height as f32 / src_height as f32;
        let scale = scale_x.min(scale_y);

        let dst_width = (src_width as f32 * scale).round() as u32;
        let dst_height = (src_height as f32 * scale).round() as u32;

        Self::new(src_width, src_height, dst_width, dst_height)
    }

    /// Create an upscaler with a specific integer scale factor.
    pub fn integer_scale(src_width: u32, src_height: u32, scale: u32) -> Self {
        Self::new(src_width, src_height, src_width * scale, src_height * scale)
    }

    /// Upscale a frame using pseudo-bandlimited filtering.
    ///
    /// Input is RgbColor pixel data as a flat array.
    /// Output is also RgbColor data with dimensions (dst_width, dst_height).
    ///
    /// This function uses parallel processing via rayon for optimal performance.
    /// The output buffer is pre-allocated and filled directly to minimize allocations.
    pub fn upscale_rgb(&self, src: &[RgbColor]) -> Vec<RgbColor> {
        let expected_size = (self.src_width * self.src_height) as usize;
        if src.len() != expected_size {
            panic!(
                "Invalid source buffer size: expected {}, got {}",
                expected_size,
                src.len()
            );
        }

        let dst_size = (self.dst_width * self.dst_height) as usize;
        
        // Pre-allocate output buffer with uninitialized memory for performance
        let mut result = vec![(0u8, 0u8, 0u8); dst_size];
        
        // Process rows in parallel using rayon
        // Each parallel task writes directly to its slice of the output buffer
        result
            .par_chunks_mut(self.dst_width as usize)
            .enumerate()
            .for_each(|(dst_y, row)| {
                for (dst_x, pixel) in row.iter_mut().enumerate() {
                    *pixel = self.sample_pixel(src, dst_x as u32, dst_y as u32);
                }
            });

        result
    }

    /// Upscale a frame from u32 ARGB format.
    ///
    /// Input is ARGB pixel data (0xAARRGGBB) as a flat array.
    /// Output is RgbColor data with dimensions (dst_width, dst_height).
    pub fn upscale_argb_to_rgb(&self, src: &[u32]) -> Vec<RgbColor> {
        let rgb: Vec<RgbColor> = src
            .iter()
            .map(|&pixel| {
                (
                    ((pixel >> 16) & 0xFF) as u8,
                    ((pixel >> 8) & 0xFF) as u8,
                    (pixel & 0xFF) as u8,
                )
            })
            .collect();
        self.upscale_rgb(&rgb)
    }

    /// Upscale and return as u32 ARGB format.
    pub fn upscale_argb(&self, src: &[u32]) -> Vec<u32> {
        let rgb = self.upscale_argb_to_rgb(src);
        rgb.iter()
            .map(|&(r, g, b)| 0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
            .collect()
    }

    /// Sample a single pixel using pseudo-bandlimited filtering.
    ///
    /// This implements a simplified version of the pseudo-bandlimited algorithm:
    /// - At integer scales, pixels are sharp (nearest neighbor behavior)
    /// - At non-integer scales, smooth anti-aliased transitions occur at pixel boundaries
    fn sample_pixel(&self, src: &[RgbColor], dst_x: u32, dst_y: u32) -> RgbColor {
        // Map destination coordinates to source coordinates
        let src_x_f = (dst_x as f32 + 0.5) / self.scale_x - 0.5;
        let src_y_f = (dst_y as f32 + 0.5) / self.scale_y - 0.5;

        // Get integer source coordinates and fractional parts
        let src_x0 = src_x_f.floor() as i32;
        let src_y0 = src_y_f.floor() as i32;

        let frac_x = src_x_f - src_x0 as f32;
        let frac_y = src_y_f - src_y0 as f32;

        // For pseudo-bandlimited filtering:
        // The blend weight is modified based on the scale factor
        // At high scales (integer scales), blend sharply at pixel centers
        // At low scales (non-integer), blend smoothly across boundaries
        let blend_x = smoothstep_blend(frac_x, self.scale_x);
        let blend_y = smoothstep_blend(frac_y, self.scale_y);

        // Get the four neighboring pixels
        let c00 = self.get_pixel_clamped(src, src_x0, src_y0);
        let c10 = self.get_pixel_clamped(src, src_x0 + 1, src_y0);
        let c01 = self.get_pixel_clamped(src, src_x0, src_y0 + 1);
        let c11 = self.get_pixel_clamped(src, src_x0 + 1, src_y0 + 1);

        // Blend using the pseudo-bandlimited weights
        bilinear_blend(c00, c10, c01, c11, blend_x, blend_y)
    }

    /// Get a pixel from the source buffer with clamping.
    #[inline]
    fn get_pixel_clamped(&self, src: &[RgbColor], x: i32, y: i32) -> RgbColor {
        let x = x.clamp(0, self.src_width as i32 - 1) as u32;
        let y = y.clamp(0, self.src_height as i32 - 1) as u32;
        src[(y * self.src_width + x) as usize]
    }
}

/// Resolution presets for common video export sizes.
///
/// # Pixel Aspect Ratio (PAR) Correction
///
/// The NES outputs 256x240 pixels with a pixel aspect ratio (PAR) of 8:7.
/// On CRT TVs, pixels were wider than tall, resulting in a display aspect
/// ratio of approximately 4:3.
///
/// **Integer scales** (2x, 3x, 4x) do NOT apply PAR correction:
/// - These produce pixel-perfect output for emulation purists
/// - Pixels remain square in the output
///
/// **All other resolutions** apply PAR correction:
/// - The effective source width is treated as 256 × (8/7) ≈ 293 pixels
/// - This produces the correct display aspect ratio (~4:3)
/// - Useful for video export and sharing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoResolution {
    /// Native NES resolution (256x240) - no PAR correction (square pixels)
    Native,
    /// 2x scale (512x480) - exact integer scale, no PAR correction
    Scale2x,
    /// 3x scale (768x720) - exact integer scale, no PAR correction
    Scale3x,
    /// 4x scale (1024x960) - exact integer scale, no PAR correction
    Scale4x,
    /// 720p - fits within 1280x720 with PAR correction (~4:3 display)
    Hd720,
    /// 1080p - fits within 1920x1080 with PAR correction (~4:3 display)
    Hd1080,
    /// 4K - fits within 3840x2160 with PAR correction (~4:3 display)
    Uhd4k,
    /// Custom max dimensions - fits within WxH with PAR correction
    Custom(u32, u32),
}

impl VideoResolution {
    /// Get the target dimensions for the given source size.
    ///
    /// # PAR Correction
    ///
    /// - **Integer scales** (Native, 2x, 3x, 4x): No PAR correction, square pixels
    /// - **All other resolutions**: Applies 8:7 PAR correction for correct display AR
    ///
    /// # Returns
    ///
    /// The output dimensions (width, height) that will fit within the target
    /// resolution while preserving the correct display aspect ratio.
    pub fn dimensions(&self, src_width: u32, src_height: u32) -> (u32, u32) {
        match self {
            // Integer scales: no PAR correction (pixel-perfect)
            VideoResolution::Native => (src_width, src_height),
            VideoResolution::Scale2x => (src_width * 2, src_height * 2),
            VideoResolution::Scale3x => (src_width * 3, src_height * 3),
            VideoResolution::Scale4x => (src_width * 4, src_height * 4),
            // All other resolutions: apply PAR correction
            VideoResolution::Hd720 => {
                fit_with_par_correction(src_width, src_height, 1280, 720)
            }
            VideoResolution::Hd1080 => {
                fit_with_par_correction(src_width, src_height, 1920, 1080)
            }
            VideoResolution::Uhd4k => {
                fit_with_par_correction(src_width, src_height, 3840, 2160)
            }
            VideoResolution::Custom(w, h) => {
                fit_with_par_correction(src_width, src_height, *w, *h)
            }
        }
    }

    /// Parse from string (e.g., "1080p", "4x", "1920x1080")
    ///
    /// Supported formats:
    /// - `native`, `1x` - native NES resolution (256x240)
    /// - `2x`, `3x`, `4x` - integer scale factors
    /// - `720p`, `hd` - fit within 1280x720
    /// - `1080p`, `fhd`, `fullhd` - fit within 1920x1080
    /// - `4k`, `2160p`, `uhd` - fit within 3840x2160
    /// - `WIDTHxHEIGHT` - fit within custom dimensions (e.g., "1920x1080")
    ///
    /// All resolutions preserve the source aspect ratio (NES: 16:15).
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.to_lowercase();

        // Check for presets
        match s.as_str() {
            "native" | "1x" => return Ok(VideoResolution::Native),
            "2x" => return Ok(VideoResolution::Scale2x),
            "3x" => return Ok(VideoResolution::Scale3x),
            "4x" => return Ok(VideoResolution::Scale4x),
            "720p" | "hd" => return Ok(VideoResolution::Hd720),
            "1080p" | "fhd" | "fullhd" => return Ok(VideoResolution::Hd1080),
            "4k" | "2160p" | "uhd" => return Ok(VideoResolution::Uhd4k),
            _ => {}
        }

        // Check for WIDTHxHEIGHT format
        if let Some((w, h)) = s.split_once('x') {
            let width = w
                .trim()
                .parse::<u32>()
                .map_err(|_| format!("Invalid width: {}", w))?;
            let height = h
                .trim()
                .parse::<u32>()
                .map_err(|_| format!("Invalid height: {}", h))?;
            return Ok(VideoResolution::Custom(width, height));
        }

        Err(format!(
            "Unknown resolution: '{}'. Use native, 2x, 3x, 4x, 720p, 1080p, 4k, or WIDTHxHEIGHT",
            s
        ))
    }
}

impl std::str::FromStr for VideoResolution {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> { VideoResolution::parse(s) }
}

/// Fit source dimensions to target dimensions with PAR correction.
///
/// Applies the 8:7 NES pixel aspect ratio correction to produce the correct
/// display aspect ratio (~4:3) in the output.
///
/// # Arguments
///
/// * `src_w` - Source width (e.g., 256 for NES)
/// * `src_h` - Source height (e.g., 240 for NES)
/// * `max_w` - Maximum output width
/// * `max_h` - Maximum output height
///
/// # Returns
///
/// Output dimensions that fit within max_w x max_h while preserving the
/// PAR-corrected display aspect ratio.
fn fit_with_par_correction(src_w: u32, src_h: u32, max_w: u32, max_h: u32) -> (u32, u32) {
    // Apply PAR to get the effective display width
    // For NES: 256 pixels × (8/7) PAR = ~293 effective display pixels wide
    let display_w = src_w as f32 * NES_PAR;
    let display_h = src_h as f32;

    // Fit within max dimensions while preserving display aspect ratio (DAR)
    // DAR for NES: 293/240 ≈ 1.22 (close to 4:3 when overscan is considered)
    let scale_by_width = max_w as f32 / display_w;
    let scale_by_height = max_h as f32 / display_h;
    let scale = scale_by_width.min(scale_by_height);

    // Calculate final dimensions
    let dst_w = (display_w * scale).round() as u32;
    let dst_h = (display_h * scale).round() as u32;

    // Round to nearest even dimensions for video codec compatibility
    ((dst_w + 1) & !1, (dst_h + 1) & !1)
}

/// Fit source dimensions to target dimensions without PAR correction.
///
/// Preserves the storage aspect ratio (pixel grid ratio) without applying
/// any pixel aspect ratio correction.
#[allow(dead_code)]
fn fit_with_aspect_ratio(src_w: u32, src_h: u32, max_w: u32, max_h: u32) -> (u32, u32) {
    let scale_x = max_w as f32 / src_w as f32;
    let scale_y = max_h as f32 / src_h as f32;
    let scale = scale_x.min(scale_y);

    // Round to nearest even dimensions for video codec compatibility
    let dst_w = ((src_w as f32 * scale).round() as u32) & !1;
    let dst_h = ((src_h as f32 * scale).round() as u32) & !1;

    (dst_w, dst_h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upscaler_integer_scale() {
        let upscaler = PixelArtUpscaler::integer_scale(4, 4, 2);
        assert_eq!(upscaler.dst_width, 8);
        assert_eq!(upscaler.dst_height, 8);

        // Create a simple 4x4 image with distinct colors
        let src = vec![
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (0, 255, 255),
            (255, 0, 255),
            (128, 128, 128),
            (0, 0, 0),
            (255, 128, 0),
            (128, 0, 255),
            (0, 128, 255),
            (255, 255, 255),
            (64, 64, 64),
            (192, 192, 192),
            (128, 0, 0),
            (0, 128, 0),
        ];

        let dst = upscaler.upscale_rgb(&src);
        assert_eq!(dst.len(), 64); // 8x8
    }

    #[test]
    fn test_video_resolution_parse() {
        assert_eq!(
            VideoResolution::parse("native").unwrap(),
            VideoResolution::Native
        );
        assert_eq!(
            VideoResolution::parse("2x").unwrap(),
            VideoResolution::Scale2x
        );
        assert_eq!(
            VideoResolution::parse("1080p").unwrap(),
            VideoResolution::Hd1080
        );
        assert_eq!(
            VideoResolution::parse("1920x1080").unwrap(),
            VideoResolution::Custom(1920, 1080)
        );
    }

    #[test]
    fn test_resolution_dimensions() {
        let src_w = 256;
        let src_h = 240;

        // Integer scales: no PAR correction (square pixels)
        assert_eq!(VideoResolution::Native.dimensions(src_w, src_h), (256, 240));
        assert_eq!(
            VideoResolution::Scale2x.dimensions(src_w, src_h),
            (512, 480)
        );
        assert_eq!(
            VideoResolution::Scale4x.dimensions(src_w, src_h),
            (1024, 960)
        );

        // 1080p should fit with PAR correction (8:7)
        let (w, h) = VideoResolution::Hd1080.dimensions(src_w, src_h);
        // Check that it fits within 1920x1080
        assert!(w <= 1920);
        assert!(h <= 1080);
        // Output aspect ratio should match PAR-corrected display AR
        // Expected DAR = (256 * 8/7) / 240 ≈ 1.219
        let expected_dar = (src_w as f32 * NES_PAR) / src_h as f32;
        let actual_dar = w as f32 / h as f32;
        assert!(
            (expected_dar - actual_dar).abs() < 0.02,
            "1080p should have PAR-corrected aspect ratio: expected={:.3}, got={:.3}",
            expected_dar,
            actual_dar
        );
    }

    #[test]
    fn test_par_correction_applied() {
        let src_w = 256;
        let src_h = 240;

        // Expected PAR-corrected display aspect ratio
        let expected_dar = (src_w as f32 * NES_PAR) / src_h as f32;

        // All non-integer resolutions should have PAR correction
        for resolution in [
            VideoResolution::Hd720,
            VideoResolution::Hd1080,
            VideoResolution::Uhd4k,
            VideoResolution::Custom(1920, 1080),
            VideoResolution::Custom(800, 600),
        ] {
            let (w, h) = resolution.dimensions(src_w, src_h);
            let actual_dar = w as f32 / h as f32;
            assert!(
                (expected_dar - actual_dar).abs() < 0.02,
                "{:?}: expected DAR={:.3}, got DAR={:.3} (w={}, h={})",
                resolution,
                expected_dar,
                actual_dar,
                w,
                h
            );
        }
    }

    #[test]
    fn test_integer_scales_no_par() {
        let src_w = 256;
        let src_h = 240;

        // Integer scales should NOT apply PAR correction
        let storage_ar = src_w as f32 / src_h as f32;

        for (resolution, expected_w, expected_h) in [
            (VideoResolution::Native, 256, 240),
            (VideoResolution::Scale2x, 512, 480),
            (VideoResolution::Scale3x, 768, 720),
            (VideoResolution::Scale4x, 1024, 960),
        ] {
            let (w, h) = resolution.dimensions(src_w, src_h);
            assert_eq!(
                (w, h),
                (expected_w, expected_h),
                "{:?} should be exact integer scale",
                resolution
            );
            let actual_ar = w as f32 / h as f32;
            assert!(
                (storage_ar - actual_ar).abs() < 0.001,
                "{:?} should preserve storage AR (no PAR): expected={:.3}, got={:.3}",
                resolution,
                storage_ar,
                actual_ar
            );
        }
    }

    #[test]
    fn test_custom_resolution_fits_within_bounds() {
        let src_w = 256;
        let src_h = 240;

        // Custom resolutions should fit within specified bounds
        for (max_w, max_h) in [(1920, 1080), (1280, 720), (800, 600), (640, 480)] {
            let (w, h) = VideoResolution::Custom(max_w, max_h).dimensions(src_w, src_h);
            assert!(
                w <= max_w,
                "Width {} should fit within max {}",
                w,
                max_w
            );
            assert!(
                h <= max_h,
                "Height {} should fit within max {}",
                h,
                max_h
            );
        }
    }

    #[test]
    fn test_smoothstep() {
        assert!((smoothstep(0.0, 1.0, 0.0) - 0.0).abs() < 0.001);
        assert!((smoothstep(0.0, 1.0, 0.5) - 0.5).abs() < 0.001);
        assert!((smoothstep(0.0, 1.0, 1.0) - 1.0).abs() < 0.001);
    }
}
