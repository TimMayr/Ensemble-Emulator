//! Pane size snapping utilities for graphics panes
//!
//! This module provides utilities to snap pane sizes to integer scale factors
//! when resizing. This ensures pixel-perfect rendering by having panes snap
//! to sizes that result in clean integer scales (1x, 2x, 3x, etc.) when the
//! user is within a small threshold of those sizes.
//!
//! The snapping behavior is "sticky": once snapped, the pane stays at the integer
//! scale until the user drags far enough to exit the threshold zone.

use std::collections::HashMap;

use egui_tiles::{Container, Tile, TileId, Tree};

use crate::emulation::messages::{TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH};
use crate::frontend::egui::tiles::Pane;

/// Threshold for snapping - if within this fraction of an integer scale, snap to it
const SNAP_THRESHOLD: f32 = 0.1;

/// Minimum share value to ensure tiles remain visible after snapping
const MIN_TILE_SHARE: f32 = 0.1;

/// Approximate height of UI chrome (labels, padding) subtracted from pane height
/// when calculating available space for graphics
const UI_CHROME_HEIGHT: f32 = 20.0;

/// State for tracking snapped panes
#[derive(Default)]
pub struct SnapState {
    /// Map from pane tile_id to the integer scale it's snapped to (if any)
    snapped_scales: HashMap<TileId, f32>,
}

impl SnapState {
    pub fn new() -> Self {
        Self::default()
    }
}

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
fn get_nearest_integer_scale(scale: f32) -> f32 {
    scale.round().max(1.0)
}

/// Check if a scale is within the snap threshold of an integer
fn is_within_snap_threshold(scale: f32, target_integer: f32) -> bool {
    (scale - target_integer).abs() < SNAP_THRESHOLD
}

/// Snap the shares of graphics panes in a tree to achieve integer scale factors.
/// Uses sticky snapping: once snapped, stays snapped until user drags outside threshold.
///
/// This function modifies the tree's shares in-place.
pub fn snap_graphics_pane_sizes(tree: &mut Tree<Pane>, snap_state: &mut SnapState) {
    let Some(_root_id) = tree.root else {
        return;
    };

    // Collect all adjustments to make
    // (tile_in_linear, linear_parent_id, target_scale, pane_tile_id)
    let mut snap_adjustments: Vec<(TileId, TileId, f32, TileId, PaneDimensions, Direction)> = Vec::new();
    let mut to_unsnap: Vec<TileId> = Vec::new();

    for (tile_id, tile) in tree.tiles.iter() {
        if let Tile::Pane(pane) = tile {
            if let Some(dims) = PaneDimensions::for_pane(pane) {
                if let Some(rect) = tree.tiles.rect(*tile_id) {
                    // Account for UI chrome (label, padding, etc.)
                    let available_width = rect.width();
                    let available_height = rect.height() - UI_CHROME_HEIGHT;

                    let current_scale = calculate_scale(available_width, available_height, dims);
                    let nearest_integer = get_nearest_integer_scale(current_scale);

                    // Check if this pane is currently snapped
                    if let Some(&snapped_scale) = snap_state.snapped_scales.get(tile_id) {
                        // Currently snapped - check if we should unsnap
                        if !is_within_snap_threshold(current_scale, snapped_scale) {
                            // User has dragged outside the threshold - unsnap
                            to_unsnap.push(*tile_id);
                        } else {
                            // Still within threshold - keep snapped by adjusting back to integer
                            if let Some((linear_id, tile_in_linear, direction)) =
                                find_parent_linear_and_child(tree, *tile_id)
                            {
                                snap_adjustments.push((tile_in_linear, linear_id, snapped_scale, *tile_id, dims, direction));
                            }
                        }
                    } else {
                        // Not currently snapped - check if we should snap
                        if is_within_snap_threshold(current_scale, nearest_integer) {
                            // Enter snap zone - snap to integer
                            if let Some((linear_id, tile_in_linear, direction)) =
                                find_parent_linear_and_child(tree, *tile_id)
                            {
                                snap_adjustments.push((tile_in_linear, linear_id, nearest_integer, *tile_id, dims, direction));
                            }
                        }
                    }
                }
            }
        }
    }

    // Remove unsnapped panes from state
    for tile_id in to_unsnap {
        snap_state.snapped_scales.remove(&tile_id);
    }

    // Apply the snap adjustments and update state
    for (tile_in_linear, linear_id, target_scale, pane_tile_id, dims, direction) in snap_adjustments {
        if apply_snap_to_scale(tree, tile_in_linear, linear_id, pane_tile_id, target_scale, dims, direction) {
            // Successfully snapped - record in state
            snap_state.snapped_scales.insert(pane_tile_id, target_scale);
        }
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

/// Apply snapping to achieve a specific target scale
/// Returns true if successfully applied
fn apply_snap_to_scale(
    tree: &mut Tree<Pane>,
    tile_in_linear: TileId,
    linear_id: TileId,
    pane_tile_id: TileId,
    target_scale: f32,
    dims: PaneDimensions,
    direction: Direction,
) -> bool {
    // Get the pane rect to calculate current size
    let Some(pane_rect) = tree.tiles.rect(pane_tile_id) else {
        return false;
    };

    // Get the parent rect to calculate total available size
    let Some(parent_rect) = tree.tiles.rect(linear_id) else {
        return false;
    };

    // Calculate current and target sizes
    let (current_size, target_size) = match direction {
        Direction::Horizontal => {
            (pane_rect.width(), dims.width * target_scale)
        }
        Direction::Vertical => {
            (pane_rect.height(), dims.height * target_scale + UI_CHROME_HEIGHT)
        }
    };

    let size_delta = target_size - current_size;

    // Get the parent container
    let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get(linear_id) else {
        return false;
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
        return false;
    }

    // Calculate shares per pixel
    let shares_per_pixel = total_shares / total_size;
    
    // Calculate the share delta needed
    let share_delta = size_delta * shares_per_pixel;

    // Get mutable reference and apply the change
    let Some(Tile::Container(Container::Linear(linear))) = tree.tiles.get_mut(linear_id) else {
        return false;
    };

    // Adjust the share for this tile
    let current_share = linear.shares[tile_in_linear];
    let new_share = (current_share + share_delta).max(MIN_TILE_SHARE);
    linear.shares.set_share(tile_in_linear, new_share);

    true
}
