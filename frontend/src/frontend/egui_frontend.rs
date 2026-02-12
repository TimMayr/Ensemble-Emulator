/// Main application entry point for the egui frontend.
///
/// This module provides the main `EguiApp` struct and the `run` function
/// to start the frontend. The implementation is split across several
/// submodules for maintainability:
///
/// - `egui::config`: Configuration types
/// - `egui::fps_counter`: FPS tracking
/// - `egui::input`: Input handling
/// - `egui::message_handlers`: Async and emulator message processing
/// - `egui::textures`: Texture management
/// - `egui::tiles`: Tile tree behavior and pane management
/// - `egui::ui`: UI rendering components
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use egui::{Context, Style, ViewportCommand, Visuals};
use ensemble_lockstep::emulation::nes::Nes;
use ensemble_lockstep::emulation::ppu::{
    EmulatorFetchable, PaletteData ,TILE_COUNT, TileData,
};
use ensemble_lockstep::emulation::savestate::SaveState;
use ensemble_lockstep::emulation::screen_renderer::ScreenRenderer;
use ensemble_lockstep::util::ToBytes;

use crate::channel_emu::ChannelEmulator;
use crate::frontend::egui::config::{AppConfig, AppSpeed};
use crate::frontend::egui::fps_counter::FpsCounter;
use crate::frontend::egui::input::handle_keyboard_input;
use crate::frontend::egui::message_handlers::{AsyncMessageHandler, EmulatorMessageHandler};
use crate::frontend::egui::textures::EmuTextures;
use crate::frontend::egui::tiles::{
    Pane, TreeBehavior, compute_required_fetches_from_tree, create_tree,
};
use crate::frontend::egui::ui::{add_menu_bar, add_status_bar, render_savestate_dialogs};
use crate::frontend::messages::{AsyncFrontendMessage, FrontendEvent, SavestateLoadContext};
use crate::frontend::persistence::{
    get_data_dir, get_data_file_path, get_egui_storage_path, load_config, write_file_async,
};
use crate::frontend::util;
use crate::messages::{EmulatorMessage, FrontendMessage, SaveType};

/// Key used for storing egui_tiles tree state in egui's persistence
const EGUI_TILES_TREE_KEY: &str = "emulator_tiles_tree";

/// Interval between periodic autosaves (5 minutes)
const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Maximum number of autosaves to keep per game
const MAX_AUTOSAVES_PER_GAME: usize = 1024;

/// Shared deque for frontend events that can be pushed from UI components
pub type FrontendEventQueue = Rc<RefCell<VecDeque<FrontendEvent>>>;

/// Main egui application state, generic over a ScreenRenderer implementation.
/// 
/// The type parameter `R` specifies the renderer used for converting palette
/// indices to RGB colors. This allows the frontend to work with any renderer
/// that implements the `ScreenRenderer` trait.
pub struct EguiApp<R: ScreenRenderer> {
    pub(crate) channel_emu: ChannelEmulator,
    pub(crate) to_emulator: Sender<FrontendMessage>,
    pub(crate) from_emulator: Receiver<EmulatorMessage>,
    pub(crate) from_async: Receiver<AsyncFrontendMessage>,
    pub(crate) async_sender: Sender<AsyncFrontendMessage>,
    /// Queue for visual/frontend-only events processed on each update
    pub(crate) event_queue: FrontendEventQueue,
    pub(crate) emu_textures: EmuTextures,
    pub(crate) fps_counter: FpsCounter,
    accumulator: Duration,
    pub(crate) config: AppConfig<R>,
    /// The tile tree for docking behavior
    tree: egui_tiles::Tree<Pane>,
    /// Track if pattern tables was visible last frame to detect when it becomes visible
    pattern_tables_was_visible: bool,
    /// Track if nametables was visible last frame to detect when it becomes visible
    nametables_was_visible: bool,
    /// Time of last periodic autosave
    last_autosave: Instant,
    /// Track if window was focused last frame to detect focus loss
    was_focused: bool,
}

impl<R: ScreenRenderer + Default + Clone> EguiApp<R> {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        channel_emu: ChannelEmulator,
        to_emulator: Sender<FrontendMessage>,
        from_emulator: Receiver<EmulatorMessage>,
        to_async: Sender<AsyncFrontendMessage>,
        from_async: Receiver<AsyncFrontendMessage>,
    ) -> Self {
        // Load configuration from TOML file
        let loaded_config = load_config::<R>();

        // Try to restore the tile tree from egui's storage, fall back to default
        let tree = cc
            .storage
            .and_then(|storage| {
                eframe::get_value::<egui_tiles::Tree<Pane>>(storage, EGUI_TILES_TREE_KEY)
            })
            .unwrap_or_else(create_tree);

        // Create default config and apply loaded settings
        let mut config = AppConfig::<R>::default();
        if let Some(ref persistent_config) = loaded_config {
            config = persistent_config.into();
        }

        Self {
            channel_emu,
            to_emulator,
            from_emulator,
            from_async,
            async_sender: to_async,
            event_queue: Rc::new(RefCell::new(VecDeque::new())),
            emu_textures: Default::default(),
            fps_counter: Default::default(),
            accumulator: Default::default(),
            config,
            tree,
            pattern_tables_was_visible: false,
            nametables_was_visible: false,
            last_autosave: Instant::now(),
            was_focused: true,
        }
    }

    /// Calculate the frame budget based on current speed settings
    fn get_frame_budget(&self) -> Duration {
        let speed = self
            .config
            .speed_config
            .app_speed
            .get_fps(&self.config.speed_config);

        if speed == 0.0 {
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

        if fps == 0.0 {
            return Duration::from_secs(5);
        }

        Duration::from_nanos(1_000_000_000 / fps as u64)
    }

    /// Process messages received from various sources
    fn process_messages(&mut self, ctx: &Context) {
        self.process_frontend_events(ctx);
        // Delegate to trait implementations in message_handlers module
        self.handle_async_messages(ctx);
        self.handle_emulator_messages(ctx);
    }

    /// Process visual/frontend-only events from the deque
    fn process_frontend_events(&mut self, ctx: &Context) {
        let mut queue = self.event_queue.borrow_mut();
        while let Some(event) = queue.pop_front() {
            match event {
                FrontendEvent::ChangeWindowTitle(title) => {
                    ctx.send_viewport_cmd(ViewportCommand::Title(title))
                }
                FrontendEvent::RefreshPalette => {
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
            }
        }
    }

    pub(crate) fn load_rom(&mut self, rom_path: PathBuf) {
        let _ = self
            .to_emulator
            .send(FrontendMessage::CreateSaveState(SaveType::Autosave));

        let _ = self
            .to_emulator
            .send(FrontendMessage::LoadRom(rom_path.clone()));

        self.config.user_config.previous_rom_path = Some(rom_path.clone());

        self.event_queue
            .borrow_mut()
            .push_back(FrontendEvent::ChangeWindowTitle(
                rom_path
                    .file_stem()
                    .map(|f| format!("Tensordance - {}", f.to_string_lossy()))
                    .unwrap_or("Tensordance".to_string()),
            ));
    }

    /// Load a savestate after ROM has been verified/selected
    pub(crate) fn load_savestate_with_rom(
        &mut self,
        context: &SavestateLoadContext,
        rom_path: &Path,
    ) {
        if self.config.user_config.loaded_rom.is_some() {
            let _ = self
                .to_emulator
                .send(FrontendMessage::CreateSaveState(SaveType::Autosave));
        }

        // First power off, load ROM, power on
        let _ = self.to_emulator.send(FrontendMessage::PowerOff);
        self.load_rom(rom_path.to_path_buf());
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

    pub(crate) fn create_auto_save(&self, savestate: Box<SaveState>) {
        if let Some(rom) = &self.config.user_config.loaded_rom {
            let rom_hash = &rom.data_checksum;
            let prev_path = &self.config.user_config.previous_rom_path;
            if let Some(prev_path) = prev_path {
                let display_name = util::rom_display_name(prev_path, rom_hash);

                let path = get_data_file_path(
                    format!(
                        "saves/{}/autosaves/autosave_{}.sav",
                        display_name,
                        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                    )
                    .as_str(),
                );

                if let Some(path) = path {
                    let _ = write_file_async(path, savestate.to_bytes(None), false);

                    // Clean up old autosaves asynchronously to avoid blocking the UI
                    Self::cleanup_old_autosaves_async(display_name);
                }
            }
        }
    }

    /// Remove the oldest autosaves if there are more than MAX_AUTOSAVES_PER_GAME.
    /// Runs asynchronously in a background thread to avoid blocking the UI.
    fn cleanup_old_autosaves_async(display_name: String) {
        std::thread::spawn(move || {
            if let Some(data_dir) = get_data_dir() {
                let autosaves_dir = data_dir.join("saves").join(&display_name).join("autosaves");

                if let Ok(entries) = std::fs::read_dir(&autosaves_dir) {
                    // Collect all autosave files with their modification times
                    let mut autosaves: Vec<(PathBuf, std::time::SystemTime)> = entries
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "sav"))
                        .filter_map(|entry| {
                            let path = entry.path();
                            let modified = entry.metadata().ok()?.modified().ok()?;
                            Some((path, modified))
                        })
                        .collect();

                    // If we have at least the limit, delete oldest to make room
                    // (use >= to maintain exactly MAX_AUTOSAVES_PER_GAME after the new save)
                    if autosaves.len() >= MAX_AUTOSAVES_PER_GAME {
                        // Sort by modification time (oldest first)
                        autosaves.sort_by_key(|a| a.1);

                        // Delete enough files to get back under the limit
                        let to_delete = autosaves.len() - MAX_AUTOSAVES_PER_GAME + 1;
                        for (path, _) in autosaves.into_iter().take(to_delete) {
                            let _ = std::fs::remove_file(path);
                        }
                    }
                }
            }
        });
    }

    pub(crate) fn get_current_quicksave_path(&self) -> Option<PathBuf> {
        let data_dir = get_data_dir();

        if let Some(rom) = &self.config.user_config.loaded_rom
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
                    for child in children.flatten() {
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

                                if path.1 == time && path.2 < version {
                                    quicksave_path = Some((child.path(), time, version));
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
        use crate::frontend::egui::tiles::{Pane, find_pane};
        find_pane(&self.tree.tiles, &Pane::PatternTables).is_some()
    }

    /// Check if the nametables pane is visible
    fn is_nametables_visible(&self) -> bool {
        use crate::frontend::egui::tiles::{Pane, find_pane};
        find_pane(&self.tree.tiles, &Pane::Nametables).is_some()
    }

    /// Check if any viewer that needs tile textures is visible
    pub(crate) fn is_tile_viewer_visible(&self) -> bool {
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
    pub(crate) fn detect_changed_palettes(
        &self,
        new_palette_data: &Option<Box<PaletteData>>,
    ) -> Vec<usize> {
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
    pub(crate) fn detect_changed_tiles(
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

            let max_emulation_time = Duration::from_millis(17);

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

    /// Check if a periodic autosave should be triggered based on elapsed time
    fn check_periodic_autosave(&mut self) {
        // Only autosave if a ROM is loaded and console is powered
        if self.config.user_config.loaded_rom.is_some()
            && self.config.console_config.is_powered
            && self.last_autosave.elapsed() >= AUTOSAVE_INTERVAL
        {
            // Update timestamp first to prevent overlapping save operations
            self.last_autosave = Instant::now();
            let savestate = self.channel_emu.nes.save_state();
            self.create_auto_save(Box::new(savestate));
        }
    }

    /// Check if window focus was lost and trigger autosave if needed
    fn check_focus_autosave(&mut self, ctx: &Context) {
        let is_focused = ctx.input(|i| i.focused);

        // Only autosave when focus is lost (transition from focused to unfocused)
        // This also resets the periodic timer since we just saved
        if self.was_focused
            && !is_focused
            && self.config.user_config.loaded_rom.is_some()
            && self.config.console_config.is_powered
        {
            // Update timestamp first to prevent overlapping save operations
            self.last_autosave = Instant::now();
            let savestate = self.channel_emu.nes.save_state();
            self.create_auto_save(Box::new(savestate));
        }

        self.was_focused = is_focused;
    }
}

impl<R: ScreenRenderer + Default + Clone> eframe::App for EguiApp<R> {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        handle_keyboard_input(
            ctx,
            &self.async_sender,
            &mut self.config,
            &mut self.emu_textures.last_frame_request,
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

        // Check for periodic autosave (every 5 minutes)
        self.check_periodic_autosave();

        // Check for focus loss autosave (when window loses focus)
        self.check_focus_autosave(ctx);

        add_menu_bar(ctx, &self.config, &self.async_sender, &mut self.tree);

        // Status bar at bottom
        add_status_bar(
            ctx,
            &self.fps_counter,
            &self.config.speed_config,
            &self.emu_textures,
        );

        // Central panel with tile tree
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior =
                TreeBehavior::new(&mut self.config, &self.emu_textures, &self.async_sender);
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

        let savestate = self.channel_emu.nes.save_state();
        self.create_auto_save(Box::new(savestate));

        let _ = self.to_emulator.send(FrontendMessage::Quit);
    }
}

impl<R: ScreenRenderer> Debug for EguiApp<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str("EguiApp") }
}

/// Run the egui frontend with the specified renderer type.
/// 
/// The renderer type `R` must implement `ScreenRenderer`, `Default`, `Clone`, and `Send + 'static`.
pub fn run<R>(
    rom: Option<PathBuf>,
    _palette: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> 
where
    R: ScreenRenderer + Default + Clone + Send + 'static,
{
    // Create the emulator instance
    let console = Nes::default();

    // Palette is now loaded from persistent config, not command line
    // The _palette parameter is kept for API compatibility but unused

    // Create channel-based emulator wrapper
    let (channel_emu, tx_to_emu, rx_from_emu) = ChannelEmulator::new(console);
    let (to_frontend, from_async) = crossbeam_channel::unbounded();

    // Setup Emulator State via messages
    let _ = to_frontend.send(AsyncFrontendMessage::LoadRom(rom));

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
            Ok(Box::new(EguiApp::<R>::new(
                cc,
                channel_emu,
                tx_to_emu,
                rx_from_emu,
                to_frontend,
                from_async,
            )))
        }),
    )?;

    Ok(())
}
