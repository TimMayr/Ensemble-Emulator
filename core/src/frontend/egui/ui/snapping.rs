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

/// Minimum pixel delta to trigger snapping - prevents oscillation when already snapped
const MIN_SNAP_DELTA_PIXELS: f32 = 0.5;

/// Minimum share value to ensure tiles remain visible after snapping
const MIN_TILE_SHARE: f32 = 0.1;

/// Approximate height of UI chrome (labels, padding) subtracted from pane height
/// when calculating available space for graphics
const UI_CHROME_HEIGHT: f32 = 20.0;

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
    let Some(_root_id) = tree.root else {
        return;
    };

    // Collect all panes that need snapping and their parent containers
    // (tile_in_linear, linear_parent_id, size_delta)
    let mut snap_adjustments: Vec<(TileId, TileId, f32)> = Vec::new();

    for (tile_id, tile) in tree.tiles.iter() {
        if let Tile::Pane(pane) = tile {
            if let Some(dims) = PaneDimensions::for_pane(pane) {
                if let Some(rect) = tree.tiles.rect(*tile_id) {
                    // Account for UI chrome (label, padding, etc.)
                    let available_width = rect.width();
                    let available_height = rect.height() - UI_CHROME_HEIGHT;

                    let current_scale = calculate_scale(available_width, available_height, dims);

                    if let Some(target_scale) = snap_scale_if_close(current_scale) {
                        // Find the parent linear container and the tile that's directly in it
                        if let Some((linear_id, tile_in_linear, direction)) =
                            find_parent_linear_and_child(tree, *tile_id)
                        {
                            // Calculate the size difference we need to achieve
                            let (current_size, target_size) = match direction {
                                Direction::Horizontal => {
                                    (rect.width(), dims.width * target_scale)
                                }
                                Direction::Vertical => {
                                    (rect.height(), dims.height * target_scale + UI_CHROME_HEIGHT)
                                }
                            };

                            let size_delta = target_size - current_size;
                            
                            // Only snap if the delta is small enough (within threshold)
                            let threshold_pixels = SNAP_THRESHOLD * match direction {
                                Direction::Horizontal => dims.width,
                                Direction::Vertical => dims.height,
                            };
                            
                            if size_delta.abs() <= threshold_pixels && size_delta.abs() > MIN_SNAP_DELTA_PIXELS {
                                snap_adjustments.push((tile_in_linear, linear_id, size_delta));
                            }
                        }
                    }
                }
            }
        }
    }

    // Apply the snap adjustments
    for (tile_in_linear, linear_id, size_delta) in snap_adjustments {
        apply_snap_adjustment(tree, tile_in_linear, linear_id, size_delta);
    }
}

/// Direction of the linear container containing the pane
#[derive(Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}

/// Find the parent linear container of a tile and its direction, along with the
/// child tile that's directly in the Linear container.
/// This handles the case where panes are wrapped in Tabs containers.
/// Returns: (linear_id, tile_in_linear, direction)
fn find_parent_linear_and_child(tree: &Tree<Pane>, tile_id: TileId) -> Option<(TileId, TileId, Direction)> {
    // First, find the immediate parent of the tile
    let parent_id = find_immediate_parent(tree, tile_id)?;
    
    // If the immediate parent is a Linear, the tile itself is the child
    if let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get(parent_id) {
        let direction = match linear.dir {
            egui_tiles::LinearDir::Horizontal => Direction::Horizontal,
            egui_tiles::LinearDir::Vertical => Direction::Vertical,
        };
        return Some((parent_id, tile_id, direction));
    }
    
    // If the immediate parent is a Tabs container, look for its Linear parent
    if let Some(Tile::Container(Container::Tabs(_))) = tree.tiles.get(parent_id) {
        // Find the Linear parent of the Tabs container
        if let Some(grandparent_id) = find_immediate_parent(tree, parent_id) {
            if let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get(grandparent_id) {
                let direction = match linear.dir {
                    egui_tiles::LinearDir::Horizontal => Direction::Horizontal,
                    egui_tiles::LinearDir::Vertical => Direction::Vertical,
                };
                // The Tabs container is the child that's directly in the Linear
                return Some((grandparent_id, parent_id, direction));
            }
        }
    }
    
    None
}

/// Find the immediate parent container of a tile
fn find_immediate_parent(tree: &Tree<Pane>, tile_id: TileId) -> Option<TileId> {
    for (container_id, tile) in tree.tiles.iter() {
        if let Tile::Container(container) = tile {
            let contains = match container {
                Container::Linear(linear) => linear.children.contains(&tile_id),
                Container::Tabs(tabs) => tabs.children.contains(&tile_id),
                Container::Grid(grid) => grid.children().any(|id| *id == tile_id),
            };
            if contains {
                return Some(*container_id);
            }
        }
    }
    None
}

/// Apply a snap adjustment by modifying shares to achieve the target size delta
fn apply_snap_adjustment(
    tree: &mut Tree<Pane>,
    tile_id: TileId,
    parent_id: TileId,
    size_delta: f32,
) {
    // Get the parent rect to calculate total available size
    let Some(parent_rect) = tree.tiles.rect(parent_id) else {
        return;
    };

    // Get the parent container
    let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get(parent_id) else {
        return;
    };

    // Calculate the total shares and the share-to-pixel ratio
    let total_size = match linear.dir {
        egui_tiles::LinearDir::Horizontal => parent_rect.width(),
        egui_tiles::LinearDir::Vertical => parent_rect.height(),
    };

    // Calculate total shares for visible children
    let mut total_shares = 0.0;
    for child_id in &linear.children {
        if tree.tiles.is_visible(*child_id) {
            total_shares += linear.shares[*child_id];
        }
    }

    if total_shares <= 0.0 || total_size <= 0.0 {
        return;
    }

    // Calculate shares per pixel
    let shares_per_pixel = total_shares / total_size;
    
    // Calculate the share delta needed
    let share_delta = size_delta * shares_per_pixel;

    // Get mutable reference and apply the change
    let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get_mut(parent_id) else {
        return;
    };

    // Adjust the share for this tile
    let current_share = linear.shares[tile_id];
    let new_share = (current_share + share_delta).max(MIN_TILE_SHARE);
    linear.shares.set_share(tile_id, new_share);
}
