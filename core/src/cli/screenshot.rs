//! Screenshot export functions for the CLI.
//!
//! This module provides functions to save screenshots from the emulator.

use crate::cli::CliArgs;
use crate::emulation::messages::RgbColor;

// =============================================================================
// NES Constants
// =============================================================================

/// NES native width in pixels
const NES_WIDTH: u32 = 256;
/// NES native height in pixels  
const NES_HEIGHT: u32 = 240;

// =============================================================================
// Screenshot Export
// =============================================================================

/// Save screenshot to file from a collection of frames.
///
/// Uses the last frame in the collection.
pub fn save_screenshot(frames: &[Vec<RgbColor>], args: &CliArgs) -> Result<(), String> {
    if let Some(ref screenshot_path) = args.video.screenshot {
        if frames.is_empty() {
            eprintln!("Warning: No frames to screenshot");
            return Ok(());
        }

        // Use the last frame for screenshot
        let frame = frames.last().unwrap();

        if !args.quiet {
            eprintln!("Saving screenshot to {}...", screenshot_path.display());
        }

        // Convert RgbColor to RGB bytes for PNG
        let rgb_data: Vec<u8> = frame.iter().flat_map(|&(r, g, b)| [r, g, b]).collect();

        // Create PNG using image crate
        let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(NES_WIDTH, NES_HEIGHT, rgb_data)
                .ok_or_else(|| "Failed to create image buffer".to_string())?;

        img.save(screenshot_path)
            .map_err(|e| format!("Failed to save screenshot: {}", e))?;

        if !args.quiet {
            eprintln!("Screenshot saved successfully");
        }
    }

    Ok(())
}

/// Save a single screenshot (used in streaming mode).
pub fn save_single_screenshot(frame: &[RgbColor], args: &CliArgs) -> Result<(), String> {
    if let Some(ref screenshot_path) = args.video.screenshot {
        let img: image::RgbaImage = image::ImageBuffer::from_fn(NES_WIDTH, NES_HEIGHT, |x, y| {
            let (r, g, b) = frame[(y * NES_WIDTH + x) as usize];
            image::Rgba([r, g, b, 255])
        });

        img.save(screenshot_path)
            .map_err(|e| format!("Failed to save screenshot: {}", e))?;

        if !args.quiet {
            eprintln!("Screenshot saved to {}", screenshot_path.display());
        }
    }
    Ok(())
}
