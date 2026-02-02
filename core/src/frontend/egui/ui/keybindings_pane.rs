//! Keybindings pane rendering

use egui_keybind::Keybind;

use crate::frontend::egui::config::AppConfig;

/// Render the keybindings panel
pub fn render_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        render_controller_keybindings(ui, config);
        ui.separator();
        render_emulation_keybindings(ui, config);
        ui.separator();
        render_debug_keybindings(ui, config);
        ui.separator();
        render_reset_button(ui, config);
    });
}

/// Render NES controller keybindings section
fn render_controller_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) {
    ui.collapsing("Controller", |ui| {
        egui::Grid::new("controller_keybindings_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Up");
                ui.add(Keybind::new(&mut config.keybindings.controller.up.0, "kb_up"));
                ui.end_row();

                ui.label("Down");
                ui.add(Keybind::new(
                    &mut config.keybindings.controller.down.0,
                    "kb_down",
                ));
                ui.end_row();

                ui.label("Left");
                ui.add(Keybind::new(
                    &mut config.keybindings.controller.left.0,
                    "kb_left",
                ));
                ui.end_row();

                ui.label("Right");
                ui.add(Keybind::new(
                    &mut config.keybindings.controller.right.0,
                    "kb_right",
                ));
                ui.end_row();

                ui.label("A Button");
                ui.add(Keybind::new(&mut config.keybindings.controller.a.0, "kb_a"));
                ui.end_row();

                ui.label("B Button");
                ui.add(Keybind::new(&mut config.keybindings.controller.b.0, "kb_b"));
                ui.end_row();

                ui.label("Start");
                ui.add(Keybind::new(
                    &mut config.keybindings.controller.start.0,
                    "kb_start",
                ));
                ui.end_row();

                ui.label("Select");
                ui.add(Keybind::new(
                    &mut config.keybindings.controller.select.0,
                    "kb_select",
                ));
                ui.end_row();
            });
    });
}

/// Render emulation control keybindings section
fn render_emulation_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) {
    ui.collapsing("Emulation Controls", |ui| {
        egui::Grid::new("emulation_keybindings_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Pause/Resume");
                ui.add(Keybind::new(
                    &mut config.keybindings.emulation.pause.0,
                    "kb_pause",
                ));
                ui.end_row();

                ui.label("Reset");
                ui.add(Keybind::new(
                    &mut config.keybindings.emulation.reset.0,
                    "kb_reset",
                ));
                ui.end_row();

                ui.label("Quicksave");
                ui.add(Keybind::new(
                    &mut config.keybindings.emulation.quicksave.0,
                    "kb_quicksave",
                ));
                ui.end_row();

                ui.label("Quickload");
                ui.add(Keybind::new(
                    &mut config.keybindings.emulation.quickload.0,
                    "kb_quickload",
                ));
                ui.end_row();
            });
    });
}

/// Render debug control keybindings section
fn render_debug_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) {
    ui.collapsing("Debug Controls", |ui| {
        egui::Grid::new("debug_keybindings_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Cycle Palette");
                ui.add(Keybind::new(
                    &mut config.keybindings.debug.cycle_palette.0,
                    "kb_cycle_palette",
                ));
                ui.end_row();
            });
    });
}

/// Render reset to defaults button
fn render_reset_button(ui: &mut egui::Ui, config: &mut AppConfig) {
    if ui.button("Reset to Defaults").clicked() {
        config.keybindings.reset_to_defaults();
    }
}
