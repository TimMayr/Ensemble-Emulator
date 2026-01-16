use std::collections::HashSet;

use egui::WidgetText;
use egui_tiles::{Behavior, SimplificationOptions, TileId, Tiles, UiResponse};

use crate::emulation::channel_emu::{ChannelEmulator, FETCH_DEPS};
use crate::emulation::messages::{
    EmulatorFetchable, NAMETABLE_HEIGHT, NAMETABLE_WIDTH, TOTAL_OUTPUT_HEIGHT, TOTAL_OUTPUT_WIDTH,
};
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::ui::draw_pattern_table;

/// The different pane types that can be displayed in the tile tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pane {
    /// Main emulator output - cannot be closed
    EmulatorOutput,
    /// Options panel - closeable, can be reopened via menu
    Options,
    /// Pattern table viewer - closeable debug viewer
    PatternTable,
    /// Nametable viewer - closeable debug viewer
    Nametable,
}

impl Pane {
    /// Returns whether this pane can be closed
    pub fn is_closable(&self) -> bool {
        match self {
            Pane::EmulatorOutput => false,
            Pane::Options | Pane::PatternTable | Pane::Nametable => true,
        }
    }

    /// Returns the tab title for this pane
    pub fn title(&self) -> &'static str {
        match self {
            Pane::EmulatorOutput => "Emulator Output",
            Pane::Options => "Options",
            Pane::PatternTable => "Pattern Table",
            Pane::Nametable => "Nametable",
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
                self.render_emulator_output(ui);
            }
            Pane::Options => {
                self.render_options(ui);
            }
            Pane::PatternTable => {
                self.render_pattern_table(ui);
            }
            Pane::Nametable => {
                self.render_nametable(ui);
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

impl TreeBehavior<'_> {
    fn render_emulator_output(&self, ui: &mut egui::Ui) {
        if let Some(ref texture) = self.emu_textures.frame_texture {
            let available = ui.available_size();

            let scale = (available.x / TOTAL_OUTPUT_WIDTH as f32)
                .min(available.y / TOTAL_OUTPUT_HEIGHT as f32);

            let display_width = TOTAL_OUTPUT_WIDTH as f32 * scale;
            let display_height = TOTAL_OUTPUT_HEIGHT as f32 * scale;

            ui.label(format!(
                "{}x{} at {:.1}x scale",
                TOTAL_OUTPUT_WIDTH, TOTAL_OUTPUT_HEIGHT, scale
            ));

            ui.image((texture.id(), egui::vec2(display_width, display_height)));
        } else {
            ui.label("Waiting for first frame...");
        }
    }

    fn render_options(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("Settings");
            self.add_speed_settings(ui);
        });
    }

    fn add_speed_settings(&mut self, ui: &mut egui::Ui) {
        use crate::frontend::egui::config::{AppSpeed, DebugSpeed};

        ui.collapsing("Speed", |ui| {
            ui.label("Emulation Speed")
                .on_hover_text("Sets the speed at which the emulation runs");
            ui.radio_value(
                &mut self.config.speed_config.app_speed,
                AppSpeed::DefaultSpeed,
                "Default (60fps)",
            );
            ui.radio_value(
                &mut self.config.speed_config.app_speed,
                AppSpeed::Custom,
                "Custom",
            );
            ui.radio_value(
                &mut self.config.speed_config.app_speed,
                AppSpeed::Uncapped,
                "Uncapped",
            );

            if self.config.speed_config.app_speed == AppSpeed::Custom {
                ui.add(
                    egui::Slider::new(&mut self.config.speed_config.custom_speed, 0..=500)
                        .text("Speed")
                        .suffix("%")
                        .fixed_decimals(0)
                        .logarithmic(true),
                );
            }
            ui.separator();
            ui.label("Debug Viewer Speed")
                .on_hover_text("Sets the speed at which the debug views update");
            ui.radio_value(
                &mut self.config.speed_config.debug_speed,
                DebugSpeed::Default,
                "10fps",
            );
            ui.radio_value(
                &mut self.config.speed_config.debug_speed,
                DebugSpeed::Custom,
                "Custom",
            );
            ui.radio_value(
                &mut self.config.speed_config.debug_speed,
                DebugSpeed::InStep,
                "Realtime",
            );
            if self.config.speed_config.debug_speed == DebugSpeed::Custom {
                ui.add(
                    egui::Slider::new(&mut self.config.speed_config.debug_custom_speed, 0..=100)
                        .text("Debug Speed")
                        .suffix("%")
                        .fixed_decimals(0)
                        .logarithmic(true),
                )
                .on_hover_text("% of main view fps");
            }
        });
    }

    fn render_pattern_table(&mut self, ui: &mut egui::Ui) {
        if self.emu_textures.tile_textures.is_some()
            && let Some(palettes) = &self.emu_textures.palette_data
        {
            let full_width = ui.available_width();
            let half_width = (full_width - ui.spacing().item_spacing.x * 3.0) * 0.5;

            ui.label(format!(
                "Pattern Tables (128x128x2 at {:.1}x scale)",
                (half_width / 256.0) - 0.1
            ));

            let selected_palette = palettes.colors[self.config.view_config.debug_active_palette];
            let transformed_palette = selected_palette
                .map(|color_index| self.config.view_config.palette_rgb_data[color_index as usize]);

            ui.horizontal_top(|ui| {
                ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                    draw_pattern_table(
                        ui,
                        0,
                        self.emu_textures,
                        self.config.view_config.debug_active_palette,
                        transformed_palette,
                    );
                });

                ui.separator();
                ui.allocate_ui(egui::vec2(half_width, half_width), |ui| {
                    draw_pattern_table(
                        ui,
                        1,
                        self.emu_textures,
                        self.config.view_config.debug_active_palette,
                        transformed_palette,
                    );
                });
            });
        } else {
            ui.label("Waiting for pattern table data...");
        }
    }

    fn render_nametable(&self, ui: &mut egui::Ui) {
        if let Some(ref data) = self.emu_textures.nametable_data
            && let Some(ref textures) = self.emu_textures.tile_textures
        {
            let available = ui.available_width();
            let base_size = 4.0;
            let logical_width = 16.0 * base_size;
            let scale = available / logical_width;
            let tex_size = egui::vec2(base_size, base_size) * scale;

            ui.label(format!(
                "Nametables ({}x{} at {:.1}x)",
                NAMETABLE_WIDTH, NAMETABLE_HEIGHT, scale
            ));

            egui::Grid::new("nametables")
                .num_columns(2)
                .spacing(egui::vec2(0.0, 0.0))
                .show(ui, |ui| {
                    for (nametable_id, nametable) in data.tiles.iter().enumerate() {
                        egui::Grid::new("nametable")
                            .num_columns(32)
                            .min_row_height(scale * base_size)
                            .min_col_width(scale * base_size)
                            .max_col_width(scale * base_size)
                            .spacing(egui::vec2(0.0, 0.0))
                            .show(ui, |ui| {
                                for (i, tile) in nametable.iter().enumerate() {
                                    let texture = &textures[0][*tile as usize];

                                    ui.image((texture.id(), tex_size));

                                    if (i + 1) % 32 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                        if nametable_id + 1 % 2 == 0 {
                            ui.end_row()
                        }
                    }
                });
        } else {
            ui.label("Waiting for nametable data...");
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

/// Compute required debug fetches based on which panes are visible
pub fn compute_required_fetches_from_tree(
    tree: &egui_tiles::Tree<Pane>,
) -> HashSet<EmulatorFetchable> {
    let mut explicit_fetches = HashSet::new();

    // Check if pattern table is visible
    if find_pane(&tree.tiles, &Pane::PatternTable).is_some() {
        explicit_fetches.insert(EmulatorFetchable::Tiles(None));
        explicit_fetches.insert(EmulatorFetchable::Palettes(None));
    }

    // Check if nametable is visible
    if find_pane(&tree.tiles, &Pane::Nametable).is_some() {
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
