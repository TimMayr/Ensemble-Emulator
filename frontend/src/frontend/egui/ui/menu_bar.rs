use crossbeam_channel::Sender;
use egui::Ui;

use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::keybindings::OnKeyAction;
use crate::frontend::egui::tiles::{Pane, add_pane_if_missing};
use crate::frontend::egui::ui::widgets::HotKeyButton;
use crate::frontend::messages::AsyncFrontendMessage;

pub fn add_menu_bar(
    ui: &mut Ui,
    config: &mut AppConfig,
    async_sender: &Sender<AsyncFrontendMessage>,
    tree: &mut egui_tiles::Tree<Pane>,
) {
    egui::Panel::top("menu_bar").show_inside(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                ui.add(HotKeyButton::for_action(
                    OnKeyAction::LoadRom,
                    config,
                    async_sender,
                ));

                ui.menu_button("Savestates", |ui| {
                    ui.add(HotKeyButton::for_action(
                        OnKeyAction::LoadSavestate,
                        config,
                        async_sender,
                    ));

                    if config.console_config.loaded_rom.is_some() {
                        ui.add(HotKeyButton::for_action(
                            OnKeyAction::CreateSavestate,
                            config,
                            async_sender,
                        ));

                        ui.separator();
                        ui.add(HotKeyButton::for_action(
                            OnKeyAction::BrowseSavestates,
                            config,
                            async_sender,
                        ));
                    }
                });
            });
            ui.menu_button("Console", |ui| {
                ui.add(HotKeyButton::for_action(
                    OnKeyAction::Reset,
                    config,
                    async_sender,
                ));
                ui.add(HotKeyButton::for_action(
                    OnKeyAction::PowerToggle,
                    config,
                    async_sender,
                ));
                ui.add(HotKeyButton::for_action(
                    OnKeyAction::PowerCycle,
                    config,
                    async_sender,
                ));
            });
            ui.menu_button("View", |ui| {
                if ui.button("Options").clicked() {
                    add_pane_if_missing(tree, Pane::Options);
                    ui.close();
                }
                if ui.button("Keybindings").clicked() {
                    add_pane_if_missing(tree, Pane::Keybindings);
                    ui.close();
                }
                ui.separator();
                ui.label("Debug Viewers");
                if ui.button("Palettes").clicked() {
                    add_pane_if_missing(tree, Pane::Palettes);
                    ui.close();
                }
                if ui.button("Pattern Tables").clicked() {
                    add_pane_if_missing(tree, Pane::PatternTables);
                    ui.close();
                }
                if ui.button("Nametables").clicked() {
                    add_pane_if_missing(tree, Pane::Nametables);
                    ui.close();
                }
                if ui.button("Sprite Viewer").clicked() {
                    add_pane_if_missing(tree, Pane::Sprites);
                    ui.close();
                }
            });
        });
    });
}
