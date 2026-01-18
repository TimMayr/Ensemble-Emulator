//! Pane size snapping utilities for graphics panes
//!
//! This module provides utilities to snap pane sizes to integer scale factors
//! when resizing. This ensures pixel-perfect rendering by having panes snap
//! to sizes that result in clean integer scales (1x, 2x, 3x, etc.) when the
//! user is within a small threshold of those sizes.

use egui_tiles::{Container, Tile, TileId, Tree};

use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::egui::tiles::Pane;

/// Threshold for snapping - if within this fraction of an integer scale, snap to it
const SNAP_THRESHOLD: f32 = 0.1;

/// Native dimensions for each graphics pane type
#[derive(Clone, Copy)]
struct PaneDimensions {
    width: f32,
    height: f32,
}

impl PaneDimensions {
    fn for_pane(pane: &Pane) -> Option<Self> {
        match pane {
            Pane::EmulatorOutput => Some(Self {
                width: TOTAL_OUTPUT_WIDTH as f32,
                height: TOTAL_OUTPUT_HEIGHT as f32,
            }),
            Pane::PatternTables => Some(Self {
                // Two 128x128 pattern tables side by side plus spacing
                width: 128.0 * 2.0,
                height: 128.0,
            }),
            Pane::Nametables => Some(Self {
                // 2x2 grid of 256x240 nametables
                width: 256.0 * 2.0,
                height: 240.0 * 2.0,
            }),
            Pane::Options => None, // Options pane doesn't need snapping
        }
    }
}

/// Calculate the current scale factor for a pane given its available size
fn calculate_scale(available_width: f32, available_height: f32, dims: PaneDimensions) -> f32 {
    (available_width / dims.width).min(available_height / dims.height)
}

/// Check if a scale is close to an integer and return the snapped scale if so
fn snap_scale_if_close(scale: f32) -> Option<f32> {
    let rounded = scale.round();
    if rounded >= 1.0 && (scale - rounded).abs() < SNAP_THRESHOLD {
        Some(rounded)
    } else {
        None
    }
}

/// Snap the shares of graphics panes in a tree to achieve integer scale factors
/// when the current size is within SNAP_THRESHOLD of an integer scale.
///
/// This function modifies the tree's shares in-place.
pub fn snap_graphics_pane_sizes(tree: &mut Tree<Pane>) {
    let Some(root_id) = tree.root else {
        return;
    };

    // First, collect information about which panes need snapping
    let snap_info = collect_snap_info(tree, root_id);

    // Then apply the snapping by adjusting shares
    for (tile_id, target_scale, dims, direction) in snap_info {
        apply_snap(tree, tile_id, target_scale, dims, direction);
    }
}

/// Direction of the linear container containing the pane
#[derive(Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}

/// Collect information about panes that need snapping
fn collect_snap_info(
    tree: &Tree<Pane>,
    _root_id: TileId,
) -> Vec<(TileId, f32, PaneDimensions, Direction)> {
    let mut result = Vec::new();

    for (tile_id, tile) in tree.tiles.iter() {
        if let Tile::Pane(pane) = tile {
            if let Some(dims) = PaneDimensions::for_pane(pane) {
                if let Some(rect) = tree.tiles.rect(*tile_id) {
                    // Account for UI chrome (label, padding, etc.)
                    // The actual available size for the image is less than the pane rect
                    let available_width = rect.width();
                    let available_height = rect.height() - 20.0; // Approximate label height

                    let current_scale = calculate_scale(available_width, available_height, dims);

                    if let Some(target_scale) = snap_scale_if_close(current_scale) {
                        // Find the parent linear container and its direction
                        if let Some((_parent_id, direction)) =
                            find_parent_linear_container(tree, *tile_id)
                        {
                            result.push((*tile_id, target_scale, dims, direction));
                        }
                    }
                }
            }
        }
    }

    result
}

/// Find the parent linear container of a tile and its direction
fn find_parent_linear_container(tree: &Tree<Pane>, tile_id: TileId) -> Option<(TileId, Direction)> {
    for (container_id, tile) in tree.tiles.iter() {
        if let Tile::Container(container) = tile {
            if let Container::Linear(linear) = container {
                if linear.children.contains(&tile_id) {
                    let direction = match linear.dir {
                        egui_tiles::LinearDir::Horizontal => Direction::Horizontal,
                        egui_tiles::LinearDir::Vertical => Direction::Vertical,
                    };
                    return Some((*container_id, direction));
                }
            }
        }
    }
    None
}

/// Apply snapping by adjusting shares to achieve the target scale
fn apply_snap(
    tree: &mut Tree<Pane>,
    tile_id: TileId,
    target_scale: f32,
    dims: PaneDimensions,
    direction: Direction,
) {
    // Get the current pane rect
    let Some(current_rect) = tree.tiles.rect(tile_id) else {
        return;
    };

    // Calculate the target size based on the target scale
    let (current_size, target_size) = match direction {
        Direction::Horizontal => {
            let target_width = dims.width * target_scale;
            (current_rect.width(), target_width)
        }
        Direction::Vertical => {
            // Add back the chrome height for vertical snapping
            let target_height = dims.height * target_scale + 20.0;
            (current_rect.height(), target_height)
        }
    };

    // Only snap if we're actually close to the target
    let size_diff = (current_size - target_size).abs();
    let threshold_in_pixels = SNAP_THRESHOLD * match direction {
        Direction::Horizontal => dims.width,
        Direction::Vertical => dims.height,
    };

    if size_diff > threshold_in_pixels {
        return;
    }

    // Find the parent container and adjust shares
    if let Some((parent_id, _)) = find_parent_linear_container(tree, tile_id) {
        if let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get_mut(parent_id) {
            // Calculate the ratio to adjust the share
            if current_size > 0.0 {
                let ratio = target_size / current_size;
                let current_share = linear.shares[tile_id];
                linear.shares.set_share(tile_id, current_share * ratio);
            }
        }
    }
}
