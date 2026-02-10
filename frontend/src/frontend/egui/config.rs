use std::collections::HashSet;
use std::fmt::Debug;
use std::path::PathBuf;
use ensemble_lockstep::emulation::ppu::EmulatorFetchable;
use ensemble_lockstep::emulation::rom::RomFile;
use ensemble_lockstep::emulation::screen_renderer::RgbPalette;
use crate::frontend::egui::keybindings::KeybindingsConfig;
use crate::frontend::messages::SavestateLoadContext;

/// Enumeration of available renderer types.
/// 
/// This allows for selecting different rendering strategies while maintaining
/// serializability for persistence. New renderers can be added as variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RendererType {
    /// Lookup table-based palette renderer (default)
    /// Uses a precomputed lookup table for fast color conversion
    #[default]
    LookupPaletteRenderer,
    // Future renderers can be added here:
    // NtscFilterRenderer,
    // CrtShaderRenderer,
    // etc.
}

impl RendererType {
    /// Returns a list of all available renderer types
    pub fn all_variants() -> &'static [RendererType] {
        &[
            RendererType::LookupPaletteRenderer,
            // Add new variants here as they are implemented
        ]
    }
    
    /// Returns a human-readable display name for the renderer
    pub fn display_name(&self) -> &'static str {
        match self {
            RendererType::LookupPaletteRenderer => "Palette Lookup Table",
        }
    }
    
    /// Returns a description of the renderer
    pub fn description(&self) -> &'static str {
        match self {
            RendererType::LookupPaletteRenderer => 
                "Fast lookup table-based renderer using the selected palette file",
        }
    }
}

#[derive(Debug, Default)]
pub struct ViewConfig {
    pub show_palette: bool,
    pub show_pattern_table: bool,
    pub show_nametable: bool,
    pub required_debug_fetches: HashSet<EmulatorFetchable>,
    /// The currently selected renderer type
    pub renderer_type: RendererType,
    /// The RGB palette data used for rendering
    pub palette_rgb_data: RgbPalette,
    pub debug_active_palette: usize,
}

/// Main application configuration
///
/// Note: `Eq` and `PartialEq` are not derived because `PendingDialogs` contains
/// `SavestateLoadContext` which includes `SaveState`, which is not trivially comparable.
#[derive(Default)]
pub struct AppConfig {
    pub view_config: ViewConfig,
    pub speed_config: SpeedConfig,
    pub user_config: UserConfig,
    pub console_config: ConsoleConfig,
    pub pending_dialogs: PendingDialogs,
    pub keybindings: KeybindingsConfig,
}

/// Pending dialog states for multi-step operations
#[derive(Default, Clone)]
pub struct PendingDialogs {
    /// Dialog to ask user if they want to use a matching ROM found in the directory
    pub matching_rom_dialog: Option<MatchingRomDialogState>,
    /// Dialog to ask user what to do when ROM checksum doesn't match
    pub checksum_mismatch_dialog: Option<ChecksumMismatchDialogState>,
    /// Dialog to ask user to select a ROM file (shows expected filename)
    pub rom_selection_dialog: Option<RomSelectionDialogState>,
    /// Generic error dialog for displaying error messages
    pub error_dialog: Option<ErrorDialogState>,
}

/// State for the matching ROM dialog
#[derive(Clone)]
pub struct MatchingRomDialogState {
    pub context: Box<SavestateLoadContext>,
    pub matching_rom_path: PathBuf,
}

/// State for the checksum mismatch warning dialog
#[derive(Clone)]
pub struct ChecksumMismatchDialogState {
    pub context: Box<SavestateLoadContext>,
    pub selected_rom_path: PathBuf,
}

/// State for the ROM selection dialog
#[derive(Clone)]
pub struct RomSelectionDialogState {
    pub context: Box<SavestateLoadContext>,
}

/// State for a generic error dialog
#[derive(Clone)]
pub struct ErrorDialogState {
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct UserConfig {
    pub previous_palette_path: Option<PathBuf>,
    pub previous_rom_path: Option<PathBuf>,
    pub previous_savestate_path: Option<PathBuf>,
    pub pattern_edit_color: u8,
    pub loaded_rom: Option<RomFile>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConsoleConfig {
    pub is_powered: bool,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            is_powered: true,
        }
    }
}

/// Emulation speed mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum AppSpeed {
    #[default]
    DefaultSpeed,
    Uncapped,
    Custom,
}

impl AppSpeed {
    pub fn get_fps(&self, speed_config: &SpeedConfig) -> f32 {
        match self {
            AppSpeed::DefaultSpeed => 60.0988,
            AppSpeed::Uncapped => f32::MAX,
            AppSpeed::Custom => 60.0988 * (speed_config.custom_speed as f32 / 100.0),
        }
    }
}

/// Debug viewer speed mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum DebugSpeed {
    #[default]
    DefaultSpeed,
    InStep,
    Custom,
}

impl DebugSpeed {
    pub fn get_fps(&self, speed_config: &SpeedConfig) -> f32 {
        match self {
            DebugSpeed::DefaultSpeed => 10.0,
            DebugSpeed::InStep => speed_config.app_speed.get_fps(speed_config),
            DebugSpeed::Custom => {
                if speed_config.debug_custom_speed == 0 {
                    return 0.0;
                }

                if speed_config.app_speed == AppSpeed::Uncapped {
                    return 10.0;
                }

                ((speed_config.debug_custom_speed as f64 / 100.0)
                    * speed_config.app_speed.get_fps(speed_config) as f64)
                    .max(1.0) as f32
            }
        }
    }
}

/// Speed-related configuration
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpeedConfig {
    pub app_speed: AppSpeed,
    pub debug_speed: DebugSpeed,
    pub is_paused: bool,
    pub custom_speed: u16,
    pub debug_custom_speed: u16,
}

impl Default for SpeedConfig {
    fn default() -> Self {
        Self {
            app_speed: Default::default(),
            debug_speed: Default::default(),
            is_paused: false,
            custom_speed: 100,
            debug_custom_speed: 10,
        }
    }
}
