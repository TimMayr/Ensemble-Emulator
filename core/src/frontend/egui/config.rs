use std::cmp::max;
use std::collections::HashSet;

use crate::emulation::messages::EmulatorFetchable;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ViewConfig {
    pub show_palette: bool,
    pub show_pattern_table: bool,
    pub show_nametable: bool,
    pub required_debug_fetches: HashSet<EmulatorFetchable>,
    pub palette_rgb_data: [u32; 64],
    pub debug_active_palette: usize,
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self {
            show_palette: false,
            show_pattern_table: false,
            show_nametable: false,
            required_debug_fetches: Default::default(),
            palette_rgb_data: [
                0xFF545454, 0xFF001E74, 0xFF081090, 0xFF300088, 0xFF440064, 0xFF5C0030, 0xFF540400,
                0xFF3C1800, 0xFF202A00, 0xFF083A00, 0xFF004000, 0xFF003C00, 0xFF00323C, 0xFF000000,
                0xFF000000, 0xFF000000, 0xFF989698, 0xFF084CC4, 0xFF3032EC, 0xFF5C1EE4, 0xFF8814B0,
                0xFFA01464, 0xFF982220, 0xFF783C00, 0xFF545A00, 0xFF287200, 0xFF087C00, 0xFF007628,
                0xFF006678, 0xFF000000, 0xFF000000, 0xFF000000, 0xFFECEEEC, 0xFF4C9AEC, 0xFF787CEC,
                0xFFB062EC, 0xFFE454EC, 0xFFEC58B4, 0xFFEC6A64, 0xFFD48820, 0xFFA0AA00, 0xFF74C400,
                0xFF4CD020, 0xFF38CC6C, 0xFF38B4CC, 0xFF3C3C3C, 0xFF000000, 0xFF000000, 0xFFECEEEC,
                0xFFA8CCEC, 0xFFBCBCEC, 0xFFD4B2EC, 0xFFECAEEC, 0xFFECAED4, 0xFFECB4B0, 0xFFE4C490,
                0xFFCCD278, 0xFFB4DE78, 0xFFA8E290, 0xFF98E2B4, 0xFFA0D6E4, 0xFFA0A2A0, 0xFF000000,
                0xFF000000,
            ],
            debug_active_palette: 0,
        }
    }
}

/// Main application configuration
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct AppConfig {
    pub view_config: ViewConfig,
    pub speed_config: SpeedConfig,
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
    pub fn get_fps(&self, speed_config: &SpeedConfig) -> u16 {
        match self {
            AppSpeed::DefaultSpeed => 60,
            AppSpeed::Uncapped => u16::MAX,
            AppSpeed::Custom => (60.0 * (speed_config.custom_speed as f32 / 100.0)) as u16,
        }
    }
}

/// Debug viewer speed mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum DebugSpeed {
    #[default]
    Default,
    InStep,
    Custom,
}

impl DebugSpeed {
    pub fn get_fps(&self, speed_config: &SpeedConfig) -> u16 {
        match self {
            DebugSpeed::Default => 10,
            DebugSpeed::InStep => speed_config.app_speed.get_fps(speed_config),
            DebugSpeed::Custom => {
                if speed_config.debug_custom_speed == 0 {
                    return 0;
                }

                if speed_config.app_speed == AppSpeed::Uncapped {
                    return 10;
                }

                max(
                    ((speed_config.debug_custom_speed as f64 / 100.0)
                        * speed_config.app_speed.get_fps(speed_config) as f64)
                        as u16,
                    1,
                )
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
