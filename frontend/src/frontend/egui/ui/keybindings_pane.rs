//! Keybindings pane rendering

use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::keybindings::Hotkey;

/// Render the keybindings panel
pub fn render_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) -> bool {
    let mut changed = false;
    egui::ScrollArea::vertical().show(ui, |ui| {
        changed |= render_controller_keybindings(ui, config);
        ui.separator();
        changed |= render_emulation_keybindings(ui, config);
        ui.separator();
        changed |= render_debug_keybindings(ui, config);
        ui.separator();
        changed |= render_reset_button(ui, config);
    });
    changed
}

/// Render NES controller keybindings section
fn render_controller_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) -> bool {
    let mut changed = false;
    ui.collapsing("Controller", |ui| {
        egui::Grid::new("controller_keybindings_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Up");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.up,
                        "kb_up",
                    ))
                    .changed();
                ui.end_row();

                ui.label("Down");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.down,
                        "kb_down",
                    ))
                    .changed();
                ui.end_row();

                ui.label("Left");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.left,
                        "kb_left",
                    ))
                    .changed();
                ui.end_row();

                ui.label("Right");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.right,
                        "kb_right",
                    ))
                    .changed();
                ui.end_row();

                ui.label("A Button");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.a,
                        "kb_a",
                    ))
                    .changed();
                ui.end_row();

                ui.label("B Button");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.b,
                        "kb_b",
                    ))
                    .changed();
                ui.end_row();

                ui.label("Start");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.start,
                        "kb_start",
                    ))
                    .changed();
                ui.end_row();

                ui.label("Select");
                changed |= ui
                    .add(Hotkey::with_id(
                        &mut config.keybindings.controller.select,
                        "kb_select",
                    ))
                    .changed();
                ui.end_row();
            });
    });
    changed
}

/// Render emulation control keybindings section
fn render_emulation_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) -> bool {
    let mut changed = false;
    ui.collapsing("Emulation Controls", |ui| {
        egui::Grid::new("emulation_keybindings_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Pause/Resume");
                changed |= ui
                    .add(
                        Hotkey::with_id(&mut config.keybindings.emulation.pause, "kb_pause")
                            .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Step Frame");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.step_frame,
                            "kb_step_frame",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Step Scanline");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.step_scanline,
                            "kb_step_scanline",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Step Master Cycle");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.step_master_cycle,
                            "kb_step_master_cycle",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Step PPU Cycle");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.step_ppu_cycle,
                            "kb_step_cpu_cycle",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Step CPU Cycle");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.step_cpu_cycle,
                            "kb_step_ppu_cycle",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Reset");
                changed |= ui
                    .add(
                        Hotkey::with_id(&mut config.keybindings.emulation.reset, "kb_reset")
                            .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Quicksave");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.quicksave,
                            "kb_quicksave",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();

                ui.label("Quickload");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.emulation.quickload,
                            "kb_quickload",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();
            });
    });
    changed
}

/// Render debug control keybindings section
fn render_debug_keybindings(ui: &mut egui::Ui, config: &mut AppConfig) -> bool {
    let mut changed = false;
    ui.collapsing("Debug Controls", |ui| {
        egui::Grid::new("debug_keybindings_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Cycle Palette");
                changed |= ui
                    .add(
                        Hotkey::with_id(
                            &mut config.keybindings.debug.cycle_palette,
                            "kb_cycle_palette",
                        )
                        .accept_modifier_keys(false),
                    )
                    .changed();
                ui.end_row();
            });
    });
    changed
}

/// Render reset to defaults button
fn render_reset_button(ui: &mut egui::Ui, config: &mut AppConfig) -> bool {
    if ui.button("Reset to Defaults").clicked() {
        config.keybindings.reset_to_defaults();
        return true;
    }
    false
}
