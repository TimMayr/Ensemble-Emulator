use std::collections::HashSet;

use egui::{Context, Ui};

use crate::emulation::channel_emu::{ChannelEmulator, FETCH_DEPS};
use crate::emulation::messages::EmulatorFetchable;
use crate::frontend::egui::config::{AppConfig, AppSpeed, DebugSpeed};

/// Add speed settings to the options panel
fn add_speed_settings(config: &mut AppConfig, ui: &mut Ui) {
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
            DebugSpeed::Default,
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

/// Add view settings to the options panel
fn add_view_settings(config: &mut AppConfig, ui: &mut Ui) {
    let mut changed = false;
    ui.collapsing("View", |ui| {
        changed |= ui
            .checkbox(&mut config.view_config.show_palette, "Palette Visualizer")
            .changed();
        changed |= ui
            .checkbox(
                &mut config.view_config.show_pattern_table,
                "Pattern Table Viewer",
            )
            .changed();
        changed |= ui
            .checkbox(&mut config.view_config.show_nametable, "Nametable Viewer")
            .changed();
    });

    if changed {
        let mut explicit_fetches = HashSet::new();
        if config.view_config.show_palette {
            explicit_fetches.insert(EmulatorFetchable::Palettes(None));
        }

        if config.view_config.show_pattern_table {
            explicit_fetches.insert(EmulatorFetchable::Tiles(None));
        }

        if config.view_config.show_nametable {
            explicit_fetches.insert(EmulatorFetchable::Nametables(None));
        }

        config.view_config.required_debug_fetches =
            ChannelEmulator::compute_required_fetches(&explicit_fetches, FETCH_DEPS.get().unwrap())
    }
}

/// Add the options panel to the context
pub fn add_options_panel(ctx: &Context, config: &mut AppConfig) {
    egui::SidePanel::right("options")
        .min_width(100.0)
        .default_width(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.take_available_width();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("Settings");

                add_view_settings(config, ui);
                add_speed_settings(config, ui)
            });
        });
}
