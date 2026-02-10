//! Options pane rendering

use crate::frontend::egui::config::{AppConfig, AppSpeed, DebugSpeed, RendererType};

/// Render the options panel
pub fn render_options(ui: &mut egui::Ui, config: &mut AppConfig) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        render_renderer_settings(ui, config);
        render_speed_settings(ui, config);
    });
}

/// Render renderer selection section
fn render_renderer_settings(ui: &mut egui::Ui, config: &mut AppConfig) {
    ui.collapsing("Renderer", |ui| {
        ui.label("Select Renderer")
            .on_hover_text("Choose the rendering method for converting emulator output to screen");
        
        // Show all available renderer types
        for renderer_type in RendererType::all_variants() {
            let is_selected = config.view_config.renderer_type == *renderer_type;
            let response = ui.selectable_label(is_selected, renderer_type.display_name());
            if response.clicked() {
                config.view_config.renderer_type = *renderer_type;
            }
            // Show description on hover
            response.on_hover_text(renderer_type.description());
        }
        
        ui.separator();
        
        // Renderer-specific settings based on selected renderer
        match config.view_config.renderer_type {
            RendererType::LookupPaletteRenderer => {
                ui.label("Palette Lookup Renderer Settings:");
                ui.label(format!("Current palette: {}", 
                    config.user_config.previous_palette_path
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Default (2C02G)".to_string())
                ));
                ui.small("Use the Palette viewer to load custom palette files.");
            }
            // Future renderers can add their settings here
        }
    });
}

/// Render speed settings section
fn render_speed_settings(ui: &mut egui::Ui, config: &mut AppConfig) {
    ui.collapsing("Speed", |ui| {
        ui.label("Emulation Speed")
            .on_hover_text("Sets the speed at which the emulation runs");
        ui.radio_value(
            &mut config.speed_config.app_speed,
            AppSpeed::DefaultSpeed,
            "Default (60fps)",
        );
        ui.radio_value(
            &mut config.speed_config.app_speed,
            AppSpeed::Custom,
            "Custom",
        );
        ui.radio_value(
            &mut config.speed_config.app_speed,
            AppSpeed::Uncapped,
            "Uncapped",
        );

        if config.speed_config.app_speed == AppSpeed::Custom {
            ui.add(
                egui::Slider::new(&mut config.speed_config.custom_speed, 0..=500)
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
            &mut config.speed_config.debug_speed,
            DebugSpeed::DefaultSpeed,
            "10fps",
        );
        ui.radio_value(
            &mut config.speed_config.debug_speed,
            DebugSpeed::Custom,
            "Custom",
        );
        ui.radio_value(
            &mut config.speed_config.debug_speed,
            DebugSpeed::InStep,
            "Realtime",
        );
        if config.speed_config.debug_speed == DebugSpeed::Custom {
            ui.add(
                egui::Slider::new(&mut config.speed_config.debug_custom_speed, 0..=100)
                    .text("Debug Speed")
                    .suffix("%")
                    .fixed_decimals(0)
                    .logarithmic(true),
            )
            .on_hover_text("% of main view fps");
        }
    });
}
