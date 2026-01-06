use std::cmp::max;

/// Configuration for which debug views are visible
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct ViewConfig {
    pub show_nametable: bool,
    pub show_pattern_table: bool,
    pub show_sprite_viewer: bool,
}

/// Main application configuration
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
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
