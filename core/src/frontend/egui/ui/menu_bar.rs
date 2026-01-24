use crossbeam_channel::Sender;
use egui::Context;

use crate::emulation::messages::FrontendMessage;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::tiles::{Pane, add_pane_if_missing};
use crate::frontend::messages::{AsyncFrontendMessage, RelayType};
use crate::frontend::util::{FileType, spawn_file_picker, spawn_savestate_picker};

pub fn add_menu_bar(
    ctx: &Context,
    config: &mut AppConfig,
    async_sender: &Sender<AsyncFrontendMessage>,
    to_emu: &Sender<FrontendMessage>,
    tree: &mut egui_tiles::Tree<Pane>,
) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Load Rom").clicked() {
                    spawn_file_picker(
                        async_sender,
                        config.user_config.previous_rom_path.as_ref(),
                        FileType::Rom,
                        RelayType::LoadRom,
                    );
                }
                ui.menu_button("Savestates", |ui| {
                    if ui.button("Save State").clicked() {
                        let _ = to_emu.send(FrontendMessage::CreateSaveState);
                    }

                    if ui.button("Load State").clicked() {
                        // Use the new multi-step savestate loading flow
                        spawn_savestate_picker(
                            async_sender,
                            config.user_config.previous_savestate_path.as_ref(),
                            config.user_config.previous_rom_path.clone(),
                        );
                    }
                })
            });
            ui.menu_button("Console", |ui| {
                if ui.button("Reset").clicked() {
                    let _ = to_emu.send(FrontendMessage::Reset);
                }
                if ui.button("Power cycle").clicked() {
                    let _ = to_emu.send(FrontendMessage::PowerOff);
                    if let Some(p) = config.user_config.previous_rom_path.clone() {
                        let _ = to_emu.send(FrontendMessage::LoadRom(p));
                    }
                    let _ = to_emu.send(FrontendMessage::Power);
                    config.console_config.is_powered = true;
                }
                if !config.console_config.is_powered {
                    if ui.button("Power On").clicked() {
                        if let Some(p) = config.user_config.previous_rom_path.clone() {
                            let _ = to_emu.send(FrontendMessage::LoadRom(p));
                        }
                        let _ = to_emu.send(FrontendMessage::Power);
                        config.console_config.is_powered = true;
                    }
                } else if ui.button("Power Off").clicked() {
                    let _ = to_emu.send(FrontendMessage::PowerOff);
                    config.console_config.is_powered = false;
                }
            });
            ui.menu_button("View", |ui| {
                if ui.button("Options").clicked() {
                    add_pane_if_missing(tree, Pane::Options);
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
            });
        });
    });
}
