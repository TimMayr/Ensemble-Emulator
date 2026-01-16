use std::collections::HashSet;

use egui::WidgetText;
use egui_tiles::{Behavior, SimplificationOptions, TileId, Tiles, UiResponse};

use crate::emulation::channel_emu::{ChannelEmulator, FETCH_DEPS};
use crate::emulation::messages::EmulatorFetchable;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::{
    render_emulator_output, render_nametable, render_options, render_pattern_table,
};

/// The different pane types that can be displayed in the tile tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pane {
    /// Main emulator output - cannot be closed
    EmulatorOutput,
    /// Options panel - closeable, can be reopened via menu
    Options,
    /// Left pattern table viewer (CHR ROM $0000-$0FFF) - closeable debug viewer
    PatternTableLeft,
    /// Right pattern table viewer (CHR ROM $1000-$1FFF) - closeable debug viewer
    PatternTableRight,
    /// Nametable 0 viewer - closeable debug viewer
    Nametable0,
    /// Nametable 1 viewer - closeable debug viewer
    Nametable1,
    /// Nametable 2 viewer - closeable debug viewer
    Nametable2,
    /// Nametable 3 viewer - closeable debug viewer
    Nametable3,
}

impl Pane {
    /// Returns whether this pane can be closed
    pub fn is_closable(&self) -> bool {
        match self {
            Pane::EmulatorOutput => false,
            _ => true,
        }
    }

    /// Returns the tab title for this pane
    pub fn title(&self) -> &'static str {
        match self {
            Pane::EmulatorOutput => "Emulator Output",
            Pane::Options => "Options",
            Pane::PatternTableLeft => "Pattern Table Left",
            Pane::PatternTableRight => "Pattern Table Right",
            Pane::Nametable0 => "Nametable 0",
            Pane::Nametable1 => "Nametable 1",
            Pane::Nametable2 => "Nametable 2",
            Pane::Nametable3 => "Nametable 3",
        }
    }

    /// Returns true if this is a pattern table pane
    pub fn is_pattern_table(&self) -> bool {
        matches!(self, Pane::PatternTableLeft | Pane::PatternTableRight)
    }

    /// Returns true if this is a nametable pane
    pub fn is_nametable(&self) -> bool {
        matches!(
            self,
            Pane::Nametable0 | Pane::Nametable1 | Pane::Nametable2 | Pane::Nametable3
        )
    }

    /// Returns the pattern table index (0 or 1), if this is a pattern table pane
    pub fn pattern_table_index(&self) -> Option<usize> {
        match self {
            Pane::PatternTableLeft => Some(0),
            Pane::PatternTableRight => Some(1),
            _ => None,
        }
    }

    /// Returns the nametable index (0-3), if this is a nametable pane
    pub fn nametable_index(&self) -> Option<usize> {
        match self {
            Pane::Nametable0 => Some(0),
            Pane::Nametable1 => Some(1),
            Pane::Nametable2 => Some(2),
            Pane::Nametable3 => Some(3),
            _ => None,
        }
    }
}

/// Behavior implementation for the tile tree
pub struct TreeBehavior<'a> {
    pub config: &'a mut AppConfig,
    pub emu_textures: &'a EmuTextures,
}

impl<'a> TreeBehavior<'a> {
    pub fn new(config: &'a mut AppConfig, emu_textures: &'a EmuTextures) -> Self {
        Self {
            config,
            emu_textures,
        }
    }
}

impl Behavior<Pane> for TreeBehavior<'_> {
    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        match pane {
            Pane::EmulatorOutput => {
                render_emulator_output(ui, self.emu_textures);
            }
            Pane::Options => {
                render_options(ui, self.config);
            }
            Pane::PatternTableLeft | Pane::PatternTableRight => {
                if let Some(index) = pane.pattern_table_index() {
                    render_pattern_table(ui, self.config, self.emu_textures, index);
                }
            }
            Pane::Nametable0 | Pane::Nametable1 | Pane::Nametable2 | Pane::Nametable3 => {
                if let Some(index) = pane.nametable_index() {
                    render_nametable(ui, self.emu_textures, index);
                }
            }
        }
        UiResponse::None
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText { pane.title().into() }

    fn is_tab_closable(&self, tiles: &Tiles<Pane>, tile_id: TileId) -> bool {
        if let Some(egui_tiles::Tile::Pane(pane)) = tiles.get(tile_id) {
            pane.is_closable()
        } else {
            true
        }
    }

    fn on_tab_close(&mut self, _tiles: &mut Tiles<Pane>, _tile_id: TileId) -> bool {
        // Always allow close for closable panes
        true
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}

/// Create the initial tile tree with the default layout
pub fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = Tiles::default();

    // Create the main emulator output pane
    let emulator_output = tiles.insert_pane(Pane::EmulatorOutput);

    // Create the options pane
    let options = tiles.insert_pane(Pane::Options);

    // Create a horizontal layout with emulator on left, options on right
    let root = tiles.insert_horizontal_tile(vec![emulator_output, options]);

    // Set shares so emulator takes most of the space
    if let Some(egui_tiles::Tile::Container(container)) = tiles.get_mut(root) {
        if let egui_tiles::Container::Linear(linear) = container {
            linear.shares.set_share(emulator_output, 3.0);
            linear.shares.set_share(options, 1.0);
        }
    }

    egui_tiles::Tree::new("emulator_tiles", root, tiles)
}

/// Find a pane in the tree by its type
pub fn find_pane(tiles: &Tiles<Pane>, pane_type: &Pane) -> Option<TileId> {
    for (tile_id, tile) in tiles.iter() {
        if let egui_tiles::Tile::Pane(pane) = tile {
            if pane == pane_type {
                return Some(*tile_id);
            }
        }
    }
    None
}

/// Add a new pane to the tree if it doesn't exist
pub fn add_pane_if_missing(tree: &mut egui_tiles::Tree<Pane>, pane_type: Pane) {
    if find_pane(&tree.tiles, &pane_type).is_none() {
        let new_tile_id = tree.tiles.insert_pane(pane_type);

        // Add to root container
        if let Some(root) = tree.root() {
            if let Some(egui_tiles::Tile::Container(container)) = tree.tiles.get_mut(root) {
                match container {
                    egui_tiles::Container::Tabs(tabs) => {
                        tabs.add_child(new_tile_id);
                        tabs.set_active(new_tile_id);
                    }
                    egui_tiles::Container::Linear(linear) => {
                        linear.children.push(new_tile_id);
                    }
                    egui_tiles::Container::Grid(grid) => {
                        grid.add_child(new_tile_id);
                    }
                }
            }
        }
    }
}

/// Check if any pattern table pane exists in the tree
fn has_any_pattern_table(tiles: &Tiles<Pane>) -> bool {
    for (_tile_id, tile) in tiles.iter() {
        if let egui_tiles::Tile::Pane(pane) = tile {
            if pane.is_pattern_table() {
                return true;
            }
        }
    }
    false
}

/// Check if any nametable pane exists in the tree
fn has_any_nametable(tiles: &Tiles<Pane>) -> bool {
    for (_tile_id, tile) in tiles.iter() {
        if let egui_tiles::Tile::Pane(pane) = tile {
            if pane.is_nametable() {
                return true;
            }
        }
    }
    false
}

/// Compute required debug fetches based on which panes are visible
pub fn compute_required_fetches_from_tree(
    tree: &egui_tiles::Tree<Pane>,
) -> HashSet<EmulatorFetchable> {
    let mut explicit_fetches = HashSet::new();

    // Check if any pattern table is visible
    if has_any_pattern_table(&tree.tiles) {
        explicit_fetches.insert(EmulatorFetchable::Tiles(None));
        explicit_fetches.insert(EmulatorFetchable::Palettes(None));
    }

    // Check if any nametable is visible
    if has_any_nametable(&tree.tiles) {
        explicit_fetches.insert(EmulatorFetchable::Nametables(None));
    }

    if !explicit_fetches.is_empty() {
        if let Some(fetch_deps) = FETCH_DEPS.get() {
            ChannelEmulator::compute_required_fetches(&explicit_fetches, fetch_deps)
        } else {
            // FETCH_DEPS not yet initialized, return explicit fetches as-is
            explicit_fetches
        }
    } else {
        explicit_fetches
    }
}
