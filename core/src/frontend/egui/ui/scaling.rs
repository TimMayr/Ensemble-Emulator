//! Scaling utilities for graphics panes
//!
//! This module provides utilities to calculate integer scale factors for graphics
//! that snap to exact pixel multiples (1x, 2x, 3x, etc.) while ensuring the content
//! fits within the available space.

/// Calculate the maximum integer scale that fits within the available dimensions.
///
/// Returns the largest integer scale (1, 2, 3, ...) such that:
/// - `content_width * scale <= available_width`
/// - `content_height * scale <= available_height`
///
/// Always returns at least 1 to ensure content is visible.
///
/// # Arguments
/// * `content_width` - The native width of the content in pixels
/// * `content_height` - The native height of the content in pixels
/// * `available_width` - The maximum available width
/// * `available_height` - The maximum available height
///
/// # Returns
/// The integer scale factor (minimum 1)
pub fn calculate_integer_scale(
    content_width: f32,
    content_height: f32,
    available_width: f32,
    available_height: f32,
) -> u32 {
    // Calculate the maximum scale that fits in each dimension
    let max_scale_x = (available_width / content_width).floor();
    let max_scale_y = (available_height / content_height).floor();

    // Take the minimum of both to ensure it fits in both dimensions
    let scale = max_scale_x.min(max_scale_y) as u32;

    // Ensure minimum scale of 1
    scale.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_fit_1x() {
        // Exact 1x fit
        assert_eq!(calculate_integer_scale(256.0, 240.0, 256.0, 240.0), 1);
    }

    #[test]
    fn test_exact_fit_2x() {
        // Exact 2x fit
        assert_eq!(calculate_integer_scale(256.0, 240.0, 512.0, 480.0), 2);
    }

    #[test]
    fn test_slightly_larger_snaps_down() {
        // Slightly larger than 2x should still be 2x
        assert_eq!(calculate_integer_scale(256.0, 240.0, 520.0, 490.0), 2);
    }

    #[test]
    fn test_slightly_smaller_than_2x() {
        // Slightly smaller than 2x should snap to 1x
        assert_eq!(calculate_integer_scale(256.0, 240.0, 500.0, 470.0), 1);
    }

    #[test]
    fn test_minimum_scale_is_1() {
        // Even if space is too small, return 1
        assert_eq!(calculate_integer_scale(256.0, 240.0, 100.0, 100.0), 1);
    }

    #[test]
    fn test_width_limited() {
        // Width-limited case: height allows 3x, width only allows 2x
        assert_eq!(calculate_integer_scale(256.0, 240.0, 512.0, 800.0), 2);
    }

    #[test]
    fn test_height_limited() {
        // Height-limited case: width allows 3x, height only allows 2x
        assert_eq!(calculate_integer_scale(256.0, 240.0, 800.0, 480.0), 2);
    }

    #[test]
    fn test_large_scale() {
        // Large scale factor
        assert_eq!(calculate_integer_scale(128.0, 128.0, 1024.0, 1024.0), 8);
    }

    #[test]
    fn test_pattern_table_dimensions() {
        // Pattern table: 128x128, fits 3x in 400x400
        assert_eq!(calculate_integer_scale(128.0, 128.0, 400.0, 400.0), 3);
    }

    #[test]
    fn test_nametable_dimensions() {
        // Nametables: 512x480 (2x2 grid of 256x240)
        assert_eq!(calculate_integer_scale(512.0, 480.0, 1024.0, 960.0), 2);
    }
}
