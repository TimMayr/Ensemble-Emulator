/// Main application entry point for the egui frontend.
///
/// This module provides the main `EguiApp` struct and the `run` function
/// to start the frontend. The implementation is split across several
/// submodules for maintainability:
///
/// - `egui::config`: Configuration types
/// - `egui::fps_counter`: FPS tracking
/// - `egui::input`: Input handling
/// - `egui::textures`: Texture management
/// - `egui::tiles`: Tile tree behavior and pane management
/// - `egui::ui`: UI rendering components
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};


use crossbeam_channel::{Receiver, Sender};
use egui::{Context, Style, ViewportCommand, Visuals};

use crate::emulation::channel_emu::ChannelEmulator;
use crate::emulation::messages::{
    EmulatorFetchable, EmulatorMessage, FrontendMessage, PaletteData, RgbPalette, SaveType,
    TileData, TILE_COUNT,
};
use crate::emulation::nes::Nes;
use crate::emulation::savestate;
use crate::frontend::egui::config::{
    AppConfig, AppSpeed, ChecksumMismatchDialogState, ErrorDialogState, MatchingRomDialogState,
    RomSelectionDialogState,
};
use crate::frontend::egui::fps_counter::FpsCounter;
use crate::frontend::egui::input::handle_keyboard_input;
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::tiles::{
    compute_required_fetches_from_tree, create_tree, Pane, TreeBehavior,
};
use crate::frontend::egui::ui::{add_menu_bar, add_status_bar, render_savestate_dialogs};
use crate::frontend::messages::{AsyncFrontendMessage, RelayType, SavestateLoadContext};
use crate::frontend::palettes::parse_palette_from_file;
use crate::frontend::persistence::{
    get_data_dir, get_data_file_path, get_egui_storage_path, load_config, write_file_async,
};
use crate::frontend::util;
use crate::frontend::util::{FileType, SavestateLoadError, ToBytes};

/// Key used for storing egui_tiles tree state in egui's persistence
const EGUI_TILES_TREE_KEY: &str = "emulator_tiles_tree";

/// Main egui application state
pub struct EguiApp {
    channel_emu: ChannelEmulator,
    to_emulator: Sender<FrontendMessage>,
    from_emulator: Receiver<EmulatorMessage>,
    from_async: Receiver<AsyncFrontendMessage>,
    async_sender: Sender<AsyncFrontendMessage>,
    emu_textures: EmuTextures,
    fps_counter: FpsCounter,
    accumulator: Duration,
    config: AppConfig,
    /// The tile tree for docking behavior
    tree: egui_tiles::Tree<Pane>,
    /// Track if pattern tables was visible last frame to detect when it becomes visible
    pattern_tables_was_visible: bool,
    /// Track if nametables was visible last frame to detect when it becomes visible
    nametables_was_visible: bool,
}

const UNCAPPED_EMU_TIME: u64 = 70;

impl EguiApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        channel_emu: ChannelEmulator,
        to_emulator: Sender<FrontendMessage>,
        from_emulator: Receiver<EmulatorMessage>,
        to_async: Sender<AsyncFrontendMessage>,
        from_async: Receiver<AsyncFrontendMessage>,
        rgb_palette: RgbPalette,
    ) -> Self {
        // Load configuration from TOML file
        let loaded_config = load_config();

        // Try to restore the tile tree from egui's storage, fall back to default
        let tree = cc
            .storage
            .and_then(|storage| {
                eframe::get_value::<egui_tiles::Tree<Pane>>(storage, EGUI_TILES_TREE_KEY)
            })
            .unwrap_or_else(create_tree);

        // Create default config and apply loaded settings
        let mut config = AppConfig::default();
        if let Some(ref persistent_config) = loaded_config {
            config = persistent_config.into();
        }
        config.view_config.palette_rgb_data = rgb_palette;

        Self {
            channel_emu,
            to_emulator,
            from_emulator,
            from_async,
            async_sender: to_async,
            emu_textures: Default::default(),
            fps_counter: Default::default(),
            accumulator: Default::default(),
            config,
            tree,
            pattern_tables_was_visible: false,
            nametables_was_visible: false,
        }
    }

    /// Calculate the frame budget based on current speed settings
    fn get_frame_budget(&self) -> Duration {
        let speed = self
            .config
            .speed_config
            .app_speed
            .get_fps(&self.config.speed_config);

        if speed == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / speed as u64)
    }

    /// Calculate the debug viewers frame budget based on current speed settings
    fn get_debug_viewers_frame_budget(&self) -> Duration {
        let fps = self
            .config
            .speed_config
            .debug_speed
            .get_fps(&self.config.speed_config);

        if fps == 0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / fps as u64)
    }

    /// Process messages received from various sources
    fn process_messages(&mut self, ctx: &Context) {
        self.process_async_messages(ctx);
        self.process_emulator_messages(ctx);
    }

    /// Process messages from async operations (file dialogs, etc.)
    fn process_async_messages(&mut self, ctx: &Context) {
        while let Ok(msg) = self.from_async.try_recv() {
            match msg {
                AsyncFrontendMessage::EmuRelay(r, p) => {
                    match r {
                        RelayType::LoadPalette => {
                            // Legacy path - when no file data is provided, use default palette
                            let palette = parse_palette_from_file(
                                p.clone(),
                                self.config.user_config.previous_palette_path.clone(),
                            );
                            self.config.view_config.palette_rgb_data = palette;
                            if let Some(p) = p {
                                self.config.user_config.previous_palette_path = Some(p)
                            }
                            let _ = self
                                .to_emulator
                                .send(FrontendMessage::SetPalette(Box::new(palette)));
                            // Only update tile textures if a tile viewer is visible
                            if self.is_tile_viewer_visible() {
                                self.emu_textures.update_tile_textures(
                                    ctx,
                                    &self.config.view_config.palette_rgb_data,
                                    None,
                                    None,
                                );
                            }
                        }
                        RelayType::LoadRom => {
                            if let Some(p) = p
                                && let Ok(p) = p.canonicalize()
                            {
                                let _ = self.to_emulator.send(FrontendMessage::PowerOff);
                                self.load_rom(ctx, p);
                                let _ = self.to_emulator.send(FrontendMessage::Power);
                            }
                        }
                    }
                }
                AsyncFrontendMessage::PaletteLoaded(palette, path) => {
                    // Palette was loaded asynchronously - apply it
                    self.config.view_config.palette_rgb_data = palette;
                    self.config.user_config.previous_palette_path = Some(path);
                    let _ = self
                        .to_emulator
                        .send(FrontendMessage::SetPalette(Box::new(palette)));
                    // Only update tile textures if a tile viewer is visible
                    if self.is_tile_viewer_visible() {
                        self.emu_textures.update_tile_textures(
                            ctx,
                            &self.config.view_config.palette_rgb_data,
                            None,
                            None,
                        );
                    }
                }
                AsyncFrontendMessage::FileSaveCompleted(error) => {
                    // Handle save completion - log any errors
                    if let Some(e) = error {
                        eprintln!("File save error: {}", e);
                    }
                }
                AsyncFrontendMessage::RefreshPalette => {
                    // Only update tile textures if a tile viewer is visible
                    if self.is_tile_viewer_visible() {
                        self.emu_textures.update_tile_textures(
                            ctx,
                            &self.config.view_config.palette_rgb_data,
                            None,
                            None,
                        );
                    }
                }
                AsyncFrontendMessage::SavestateLoaded(context) => {
                    // Savestate loaded, now need to select ROM
                    // Show dialog to select ROM with the expected filename displayed
                    self.config.pending_dialogs.rom_selection_dialog =
                        Some(RomSelectionDialogState {
                            context,
                        });
                }
                AsyncFrontendMessage::ShowMatchingRomDialog(context, matching_rom_path) => {
                    // Found a matching ROM in the directory
                    self.config.pending_dialogs.matching_rom_dialog =
                        Some(MatchingRomDialogState {
                            context,
                            matching_rom_path,
                        });
                }
                AsyncFrontendMessage::UseMatchingRom(context, rom_path) => {
                    // User chose to use the matching ROM - load the savestate
                    self.load_savestate_with_rom(ctx, &context, &rom_path);
                }
                AsyncFrontendMessage::ManuallySelectRom(context) => {
                    // User chose to manually select a ROM
                    util::spawn_rom_picker_for_savestate(
                        &self.async_sender,
                        context,
                        self.config.user_config.previous_rom_path.as_ref(),
                    );
                }
                AsyncFrontendMessage::RomSelectedForSavestate(context, rom_path) => {
                    // User selected a ROM file, now verify checksum
                    match util::compute_file_checksum(&rom_path) {
                        Some(checksum) => {
                            if checksum == context.savestate.rom_file.data_checksum {
                                // Checksum matches, load the savestate
                                self.load_savestate_with_rom(ctx, &context, &rom_path);
                            } else {
                                // Checksum mismatch, show warning dialog
                                self.config.pending_dialogs.checksum_mismatch_dialog =
                                    Some(ChecksumMismatchDialogState {
                                        context,
                                        selected_rom_path: rom_path,
                                    });
                            }
                        }
                        None => {
                            // Failed to compute checksum - show error dialog
                            self.config.pending_dialogs.error_dialog = Some(ErrorDialogState {
                                title: "Error Reading ROM".to_string(),
                                message:
                                    "Failed to read or compute checksum for the selected ROM file."
                                        .to_string(),
                            });
                        }
                    }
                }
                AsyncFrontendMessage::ShowChecksumMismatchDialog(context, rom_path) => {
                    // Show checksum mismatch warning dialog
                    self.config.pending_dialogs.checksum_mismatch_dialog =
                        Some(ChecksumMismatchDialogState {
                            context,
                            selected_rom_path: rom_path,
                        });
                }
                AsyncFrontendMessage::LoadSavestateAnyway(context, rom_path) => {
                    // User chose to load anyway despite checksum mismatch
                    self.load_savestate_with_rom(ctx, &context, &rom_path);
                }
                AsyncFrontendMessage::SelectAnotherRom(context) => {
                    // User chose to select a different ROM after checksum mismatch
                    util::spawn_rom_picker_for_savestate(
                        &self.async_sender,
                        context,
                        self.config.user_config.previous_rom_path.as_ref(),
                    );
                }
                AsyncFrontendMessage::SavestateLoadFailed(error) => {
                    // Show error dialog for failed savestate loading
                    let message = match error {
                        SavestateLoadError::FailedToLoadSavestate => {
                            "Failed to load or parse the savestate file. The file may be corrupted or invalid."
                        }
                        SavestateLoadError::FailedToComputeChecksum => {
                            "Failed to compute checksum for the ROM file."
                        }
                    };
                    self.config.pending_dialogs.error_dialog = Some(ErrorDialogState {
                        title: "Savestate Load Error".to_string(),
                        message: message.to_string(),
                    });
                }
                AsyncFrontendMessage::RomVerificationFailed(context, error) => {
                    // Show error dialog for failed ROM verification
                    let message = match error {
                        SavestateLoadError::FailedToLoadSavestate => {
                            "Unexpected error during ROM verification."
                        }
                        SavestateLoadError::FailedToComputeChecksum => {
                            "Failed to read or compute checksum for the ROM file."
                        }
                    };
                    self.config.pending_dialogs.error_dialog = Some(ErrorDialogState {
                        title: "ROM Verification Error".to_string(),
                        message: format!(
                            "{}\n\nWould you like to select a different ROM?",
                            message
                        ),
                    });
                    // Also store the context to allow retrying
                    self.config.pending_dialogs.rom_selection_dialog =
                        Some(RomSelectionDialogState {
                            context,
                        });
                }
                AsyncFrontendMessage::Quickload => {
                    if let Some(path) = self.get_current_quicksave_path() {
                        let savestate = match savestate::try_load_state(&path) {
                            Some(s) => s,
                            None => {
                                // Failed to load savestate - send error notification
                                let _ = self.async_sender.send(
                                    AsyncFrontendMessage::SavestateLoadFailed(
                                        SavestateLoadError::FailedToLoadSavestate,
                                    ),
                                );
                                return;
                            }
                        };

                        let context = SavestateLoadContext {
                            savestate,
                            savestate_path: path,
                        };

                        if let Some(rom_path) = self.config.user_config.previous_rom_path.clone() {
                            self.load_savestate_with_rom(ctx, &context, &rom_path)
                        }
                    }
                }
                AsyncFrontendMessage::Quicksave => {
                    let _ = self
                        .to_emulator
                        .send(FrontendMessage::CreateSaveState(SaveType::Quicksave));
                }
                AsyncFrontendMessage::LoadRom(path) => {
                    self.load_rom(ctx, path);
                }
                AsyncFrontendMessage::ChangeWindowTitle(title) => {
                    EguiApp::set_window_tile(ctx, title)
                }
            }
        }
    }

    fn set_window_tile(ctx: &Context, title: String) {
        ctx.send_viewport_cmd(ViewportCommand::Title(title))
    }

    fn load_rom(&mut self, ctx: &Context, rom_path: PathBuf) {
        let _ = self
            .to_emulator
            .send(FrontendMessage::LoadRom(rom_path.clone()));

        self.config.user_config.previous_rom_path = Some(rom_path.clone());
        EguiApp::set_window_tile(
            ctx,
            rom_path
                .file_stem()
                .map(|f| format!("Tensordance - {}", f.to_string_lossy()))
                .unwrap_or("Tensordance".to_string()),
        );
    }

    /// Load a savestate after ROM has been verified/selected
    fn load_savestate_with_rom(
        &mut self,
        ctx: &Context,
        context: &SavestateLoadContext,
        rom_path: &Path,
    ) {
        // First power off, load ROM, power on
        let _ = self.to_emulator.send(FrontendMessage::PowerOff);
        self.load_rom(ctx, rom_path.to_path_buf());
        let _ = self.to_emulator.send(FrontendMessage::Power);

        // Then load the savestate
        let _ = self
            .to_emulator
            .send(FrontendMessage::LoadSaveState(Box::new(
                context.savestate.clone(),
            )));

        // Update config paths
        self.config.user_config.previous_savestate_path = Some(context.savestate_path.clone());
    }

    /// Process messages from the emulator (frames, debug data, etc.)
    fn process_emulator_messages(&mut self, ctx: &Context) {
        while let Ok(msg) = self.from_emulator.try_recv() {
            match msg {
                EmulatorMessage::FrameReady(frame) => {
                    self.emu_textures.current_frame = Some(frame);
                    self.fps_counter.update();
                    self.emu_textures.update_emulator_texture(ctx);
                }
                EmulatorMessage::DebugData(data) => match data {
                    EmulatorFetchable::Palettes(p) => {
                        // Only rebuild textures if palette data actually changed and a tile viewer is visible
                        if self.emu_textures.palette_data != p {
                            // Detect which palettes changed
                            let changed_palettes = self.detect_changed_palettes(&p);
                            self.emu_textures.palette_data = p;

                            // Only update tile textures if a tile viewer is visible
                            if self.is_tile_viewer_visible() {
                                // Update only the changed palettes
                                for palette_idx in changed_palettes {
                                    self.emu_textures.update_tile_textures(
                                        ctx,
                                        &self.config.view_config.palette_rgb_data,
                                        Some(palette_idx),
                                        None, // Update all tiles for changed palettes
                                    );
                                }
                            }
                        }
                    }
                    EmulatorFetchable::Tiles(t) => {
                        // Detect which tiles changed for efficient updates
                        let changed_tiles = self.detect_changed_tiles(&t);
                        self.emu_textures.tile_data = t;

                        // Only update tile textures if a tile viewer is visible
                        if self.is_tile_viewer_visible() {
                            if changed_tiles.is_empty() || changed_tiles.len() > 10 {
                                // If many tiles changed, or we couldn't detect changes, rebuild all
                                self.emu_textures.update_tile_textures(
                                    ctx,
                                    &self.config.view_config.palette_rgb_data,
                                    None,
                                    None,
                                );
                            } else {
                                // Only rebuild the specific tiles that changed (for all palettes)
                                for tile_idx in changed_tiles {
                                    self.emu_textures.update_tile_textures(
                                        ctx,
                                        &self.config.view_config.palette_rgb_data,
                                        None, // All palettes
                                        Some(tile_idx),
                                    );
                                }
                            }
                        }
                    }
                    EmulatorFetchable::Nametables(n) => {
                        self.emu_textures.nametable_data = n;
                    }
                },
                EmulatorMessage::Stopped => {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
                EmulatorMessage::SaveState(s, t) => match t {
                    SaveType::Manual => util::spawn_save_dialog(
                        Some(&self.async_sender),
                        self.config.user_config.previous_savestate_path.as_ref(),
                        FileType::Savestate,
                        s.to_bytes(),
                    ),
                    SaveType::Quicksave => {
                        if let Some(rom) = &self.channel_emu.nes.rom_file {
                            let rom_hash = &rom.data_checksum;
                            let prev_path = &self.config.user_config.previous_rom_path;
                            if let Some(prev_path) = prev_path {
                                let display_name = util::rom_display_name(prev_path, rom_hash);

                                let path = get_data_file_path(
                                    format!(
                                        "saves/{}/quicksaves/quicksave_{}.sav",
                                        display_name,
                                        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                                    )
                                    .as_str(),
                                );

                                if let Some(path) = path {
                                    let _ = write_file_async(path, s.to_bytes(), false);
                                }
                            }
                        }
                    }
                    SaveType::Autosave => {}
                },
            }
        }
    }

    fn get_current_quicksave_path(&self) -> Option<PathBuf> {
        let data_dir = get_data_dir();

        if let Some(rom) = &self.channel_emu.nes.rom_file
            && let Some(prev_path) = &self.config.user_config.previous_rom_path
            && let Some(data_dir) = data_dir
            && data_dir.exists()
        {
            let rom_hash = &rom.data_checksum;

            let quicksave_dir = data_dir
                .join("saves")
                .join(util::rom_display_name(prev_path, rom_hash))
                .join("quicksaves");

            if std::fs::create_dir_all(quicksave_dir.clone()).is_ok() {
                let children = quicksave_dir.read_dir();

                let mut quicksave_path = None;
                if let Ok(children) = children {
                    for child in children {
                        if let Ok(child) = child {
                            let stem = child.path().file_stem()?.to_string_lossy().to_string();

                            let time_version = stem.split_once('_')?.1;

                            let (timestamp, version) =
                                if time_version.chars().filter(|c| *c == '_').count() > 1 {
                                    time_version.rsplit_once('_')?
                                } else {
                                    (time_version, "0")
                                };

                            let time =
                                chrono::NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d_%H-%M-%S");

                            let version = version.parse::<u8>().unwrap_or(0);

                            if let Ok(time) = time {
                                if quicksave_path.is_none() {
                                    quicksave_path = Some((child.path(), time, version))
                                }

                                if let Some(path) = quicksave_path.clone() {
                                    if path.1 < time {
                                        quicksave_path = Some((child.path(), time, version));
                                    }

                                    if path.1 == time {
                                        if path.2 < version {
                                            quicksave_path = Some((child.path(), time, version));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                return quicksave_path.map(|p| p.0);
            }
        }

        None
    }

    /// Check if the pattern tables pane is visible
    fn is_pattern_tables_visible(&self) -> bool {
        use crate::frontend::egui::tiles::{find_pane, Pane};
        find_pane(&self.tree.tiles, &Pane::PatternTables).is_some()
    }

    /// Check if the nametables pane is visible
    fn is_nametables_visible(&self) -> bool {
        use crate::frontend::egui::tiles::{find_pane, Pane};
        find_pane(&self.tree.tiles, &Pane::Nametables).is_some()
    }

    /// Check if any viewer that needs tile textures is visible
    fn is_tile_viewer_visible(&self) -> bool {
        self.is_pattern_tables_visible() || self.is_nametables_visible()
    }

    /// Check if pattern tables or nametables viewer just became visible and force rebuild if so
    fn check_and_handle_viewer_visibility(&mut self, ctx: &Context) {
        let pattern_tables_visible = self.is_pattern_tables_visible();
        let nametables_visible = self.is_nametables_visible();

        // If either viewer just became visible, force full rebuild
        let pattern_just_opened = pattern_tables_visible && !self.pattern_tables_was_visible;
        let nametables_just_opened = nametables_visible && !self.nametables_was_visible;

        if pattern_just_opened || nametables_just_opened {
            self.emu_textures
                .force_rebuild_all_tiles(ctx, &self.config.view_config.palette_rgb_data);
        }

        self.pattern_tables_was_visible = pattern_tables_visible;
        self.nametables_was_visible = nametables_visible;
    }

    /// Detect which palettes changed between old and new palette data
    fn detect_changed_palettes(&self, new_palette_data: &Option<Box<PaletteData>>) -> Vec<usize> {
        match (&self.emu_textures.palette_data, new_palette_data) {
            (Some(old), Some(new)) => old
                .colors
                .iter()
                .zip(new.colors.iter())
                .enumerate()
                .filter(|(_, (old_pal, new_pal))| old_pal != new_pal)
                .map(|(idx, _)| idx)
                .collect(),
            (None, Some(_)) => (0..8).collect(), // All palettes are new
            _ => vec![],                         // No update needed
        }
    }

    /// Detect which tiles changed between old and new tile data
    fn detect_changed_tiles(
        &self,
        new_tile_data: &Option<Box<[TileData; TILE_COUNT]>>,
    ) -> Vec<usize> {
        match (&self.emu_textures.tile_data, new_tile_data) {
            (Some(old), Some(new)) => old
                .iter()
                .zip(new.iter())
                .enumerate()
                .filter(|(_, (old_tile, new_tile))| old_tile != new_tile)
                .map(|(idx, _)| idx)
                .collect(),
            (None, Some(_)) => vec![], // All tiles are new - return empty to trigger full rebuild
            _ => vec![],               // No update needed
        }
    }

    /// Update emulator textures and run emulation frames
    fn update_emu_textures(&mut self, ctx: &Context) {
        let now = Instant::now();
        if !self.config.speed_config.is_paused {
            let delta = now.duration_since(self.emu_textures.last_frame_request);
            self.accumulator += delta;
            self.emu_textures.last_frame_request = now;

            let frame_budget = self.get_frame_budget();
            let is_uncapped = self.config.speed_config.app_speed == AppSpeed::Uncapped;

            // Maximum time to spend emulating per UI update to keep UI responsive
            // For uncapped mode, allow more time; for normal mode, limit to prevent UI lag
            let max_emulation_time = if is_uncapped {
                Duration::from_millis(UNCAPPED_EMU_TIME) // Allow up to 70ms of emulation per UI frame
            } else {
                Duration::from_millis(50) // More conservative for normal speeds
            };
            let emulation_start = Instant::now();

            // Effectively paused, so we skip
            if frame_budget < Duration::from_secs(5) {
                while self.accumulator >= frame_budget {
                    if let Err(e) = self.channel_emu.execute_frame() {
                        eprintln!("Emulator error: {}", e);
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                        break;
                    }

                    self.accumulator -= frame_budget;

                    // For uncapped mode, keep running frames until we hit our time budget
                    // For normal modes, check if we've spent too much time and need to drop frames
                    let elapsed = emulation_start.elapsed();
                    if elapsed > max_emulation_time {
                        if !is_uncapped {
                            // Drop backlog only in non-uncapped mode when falling behind
                            self.accumulator = Duration::ZERO;
                        }
                        break;
                    }
                }
            }

            self.request_debug_views(now);
        }
    }

    /// Request debug data from the emulator based on timing and visibility.
    ///
    /// Active fetches (like nametables) are requested on a regular interval.
    /// Passive fetches (like tiles and palettes) are only requested once initially,
    /// then re-requested when the emulator notifies of changes via
    /// `PatternTableChanged` or `PaletteChanged` messages.
    fn request_debug_views(&mut self, now: Instant) {
        let debug_frame_budget = self.get_debug_viewers_frame_budget();

        // Effectively paused, so we skip
        if debug_frame_budget < Duration::from_secs(5)
            && now.duration_since(self.emu_textures.last_debug_request) >= debug_frame_budget
        {
            for to_fetch in &self.config.view_config.required_debug_fetches {
                // Passive fetches (tiles, palettes) are only requested once initially.
                // After that, they're re-requested when the emulator notifies of changes.
                let should_skip_passive = to_fetch.is_passive()
                    && match to_fetch {
                        EmulatorFetchable::Tiles(_) => self.emu_textures.tile_textures.is_some(),
                        EmulatorFetchable::Palettes(_) => self.emu_textures.palette_data.is_some(),
                        _ => false,
                    };

                if !should_skip_passive {
                    let _ = self
                        .to_emulator
                        .send(FrontendMessage::RequestDebugData(to_fetch.clone()));
                }
            }

            self.emu_textures.last_debug_request = Instant::now();
        }
    }

    /// Render any pending dialogs for savestate loading
    fn render_savestate_dialogs_impl(&mut self, ctx: &Context) {
        render_savestate_dialogs(
            ctx,
            &mut self.config.pending_dialogs,
            &self.config.user_config,
            &self.async_sender,
        );
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        handle_keyboard_input(
            ctx,
            &self.to_emulator,
            &mut self.config,
            &mut self.emu_textures.last_frame_request,
            &self.async_sender,
        );

        if let Err(e) = self.channel_emu.process_messages() {
            eprintln!("Emulator error: {}", e);
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        self.update_emu_textures(ctx);

        // Process messages from emulator
        self.process_messages(ctx);

        // Check if pattern tables viewer just became visible and force rebuild if needed
        self.check_and_handle_viewer_visibility(ctx);

        // Update required debug fetches based on visible panes
        self.config.view_config.required_debug_fetches =
            compute_required_fetches_from_tree(&self.tree);

        add_menu_bar(
            ctx,
            &mut self.config,
            &self.async_sender,
            &self.to_emulator,
            &mut self.tree,
        );

        // Status bar at bottom
        add_status_bar(
            ctx,
            &self.fps_counter,
            &self.config.speed_config,
            &self.emu_textures,
        );

        // Central panel with tile tree
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = TreeBehavior::new(
                &mut self.config,
                &self.emu_textures,
                &self.to_emulator,
                &self.async_sender,
            );
            self.tree.ui(&mut behavior, ui);
        });

        // Render any pending savestate dialogs
        self.render_savestate_dialogs_impl(ctx);

        // Request continuous repaint for animation
        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save the tile tree layout to egui's built-in storage
        eframe::set_value(storage, EGUI_TILES_TREE_KEY, &self.tree);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save configuration to TOML file before exiting (synchronous to ensure completion)
        let persistent_config = (&self.config).into();
        if let Err(e) = crate::frontend::persistence::save_config(&persistent_config) {
            eprintln!("Failed to save configuration: {}", e);
        }

        let _ = self.to_emulator.send(FrontendMessage::Quit);
    }
}

impl Debug for EguiApp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("EguiApp") }
}

/// Run the egui frontend
pub fn run(
    rom: Option<PathBuf>,
    palette: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create the emulator instance
    let console = Nes::default();

    let palette = parse_palette_from_file(palette, None);

    // Create channel-based emulator wrapper
    let (channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);
    let (to_frontend, from_async) = crossbeam_channel::unbounded();

    // Setup Emulator State via messages
    let _ = to_frontend.send(AsyncFrontendMessage::EmuRelay(RelayType::LoadRom, rom));
    let _ = tx_to_emu.send(FrontendMessage::SetPalette(Box::new(palette)));

    // Get the storage path for egui persistence
    let storage_path = get_egui_storage_path();

    // Configure eframe options
    // Disable vsync to allow uncapped frame rates - emulator handles its own timing
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Tensordance")
            .with_app_id("tensordance"),
        vsync: false, // Disable vsync for uncapped performance
        // Enable persistence with custom storage path
        persistence_path: storage_path,
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Tensordance",
        options,
        Box::new(move |cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Default::default()
            };
            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(EguiApp::new(
                cc,
                channel_emu,
                tx_to_emu,
                rx_from_emu,
                to_frontend,
                from_async,
                palette,
            )))
        }),
    )?;

    Ok(())
}
