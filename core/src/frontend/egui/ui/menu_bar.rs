use std::path::PathBuf;

use crossbeam_channel::Sender;
use egui::Context;

use crate::emulation::messages::FrontendMessage;
use crate::frontend::egui::config::AppConfig;
use crate::frontend::egui::tiles::{add_pane_if_missing, Pane};
use crate::frontend::messages::AsyncFrontendMessage;
use crate::frontend::util::pick_rom;

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
                    std::thread::spawn({
                        let sender = async_sender.clone();
                        let prev_path = config.user_config.previous_rom_path.clone();

                        let prev_dir = if let Some(prev_path) = prev_path {
                            if let Some(prev_path) = prev_path.parent() {
                                prev_path.to_path_buf()
                            } else {
                                PathBuf::default()
                            }
                        } else {
                            PathBuf::default()
                        };

                        move || {
                            let path = pick_rom(prev_dir);
                            sender.send(AsyncFrontendMessage::LoadRom(path))
                        }
                    });
                }
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
                } else {
                    if ui.button("Power Off").clicked() {
                        let _ = to_emu.send(FrontendMessage::PowerOff);
                        config.console_config.is_powered = false;
                    }
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
