//! Async message handler for frontend operations.
//!
//! This module handles messages from async operations such as file dialogs,
//! savestate loading workflows, and other deferred operations.

use std::path::Path;

use egui::Context;
use ensemble_lockstep::emulation::ppu::EmulatorFetchable;
use ensemble_lockstep::emulation::savestate;
use ensemble_lockstep::emulation::screen_renderer::{RgbPalette, ScreenRenderer};
use crate::frontend::egui::config::{
    ChecksumMismatchDialogState, ErrorDialogState, MatchingRomDialogState, RomSelectionDialogState,
};
use crate::frontend::egui_frontend::EguiApp;
use crate::frontend::messages::{AsyncFrontendMessage, SavestateLoadContext};
use crate::frontend::util;
use crate::frontend::util::SavestateLoadError;
use crate::messages::{FrontendMessage, SaveType};

/// Trait for handling async frontend messages.
///
/// This trait is implemented by `EguiApp` to provide message handling
/// in a separate module for better code organization.
pub trait AsyncMessageHandler {
    /// Process all pending async messages from the channel.
    fn handle_async_messages(&mut self, ctx: &Context);
}

impl<R: ScreenRenderer + Default + Clone> AsyncMessageHandler for EguiApp<R> {
    fn handle_async_messages(&mut self, ctx: &Context) {
        while let Ok(msg) = self.from_async.try_recv() {
            self.handle_single_async_message(msg, ctx);
        }
    }
}

impl<R: ScreenRenderer + Default + Clone> EguiApp<R> {
    /// Handle a single async message.
    pub(crate) fn handle_single_async_message(&mut self, msg: AsyncFrontendMessage, ctx: &Context) {
        match msg {
            AsyncFrontendMessage::PaletteLoaded(palette, path) => {
                self.handle_palette_loaded(ctx, palette, path);
            }
            AsyncFrontendMessage::FileSaveCompleted(error) => {
                if let Some(e) = error {
                    eprintln!("File save error: {}", e);
                }
            }
            AsyncFrontendMessage::SavestateLoaded(context) => {
                self.config.pending_dialogs.rom_selection_dialog = Some(RomSelectionDialogState {
                    context,
                });
            }
            AsyncFrontendMessage::ShowMatchingRomDialog(context, matching_rom_path) => {
                self.config.pending_dialogs.matching_rom_dialog = Some(MatchingRomDialogState {
                    context,
                    matching_rom_path,
                });
            }
            AsyncFrontendMessage::UseMatchingRom(context, rom_path) => {
                self.load_savestate_with_rom(&context, &rom_path);
            }
            AsyncFrontendMessage::ManuallySelectRom(context) => {
                util::spawn_rom_picker_for_savestate(
                    &self.async_sender,
                    context,
                    self.config.user_config.previous_rom_path.as_ref(),
                );
            }
            AsyncFrontendMessage::RomSelectedForSavestate(context, rom_path) => {
                self.handle_rom_selected_for_savestate(&context, &rom_path);
            }
            AsyncFrontendMessage::ShowChecksumMismatchDialog(context, rom_path) => {
                self.config.pending_dialogs.checksum_mismatch_dialog =
                    Some(ChecksumMismatchDialogState {
                        context,
                        selected_rom_path: rom_path,
                    });
            }
            AsyncFrontendMessage::LoadSavestateAnyway(context, rom_path) => {
                self.load_savestate_with_rom(&context, &rom_path);
            }
            AsyncFrontendMessage::SelectAnotherRom(context) => {
                util::spawn_rom_picker_for_savestate(
                    &self.async_sender,
                    context,
                    self.config.user_config.previous_rom_path.as_ref(),
                );
            }
            AsyncFrontendMessage::SavestateLoadFailed(error) => {
                self.handle_savestate_load_failed(error);
            }
            AsyncFrontendMessage::RomVerificationFailed(context, error) => {
                self.handle_rom_verification_failed(context, error);
            }
            AsyncFrontendMessage::Quickload => {
                self.handle_quickload();
            }
            AsyncFrontendMessage::Quicksave => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::CreateSaveState(SaveType::Quicksave));
            }
            AsyncFrontendMessage::LoadRom(path) => {
                if let Some(path) = path {
                    let _ = self
                        .to_emulator
                        .send(FrontendMessage::CreateSaveState(SaveType::Autosave));
                    let _ = self.to_emulator.send(FrontendMessage::PowerOff);
                    self.load_rom(path);
                    let _ = self.to_emulator.send(FrontendMessage::Power);
                    self.config.console_config.is_powered = true;
                }
            }
            AsyncFrontendMessage::PowerOn => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::CreateSaveState(SaveType::Autosave));
                let _ = self.to_emulator.send(FrontendMessage::Power);
                self.config.console_config.is_powered = true;
            }
            AsyncFrontendMessage::PowerOff => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::CreateSaveState(SaveType::Autosave));
                let _ = self.to_emulator.send(FrontendMessage::PowerOff);
                self.config.console_config.is_powered = false;
            }
            AsyncFrontendMessage::Reset => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::CreateSaveState(SaveType::Autosave));
                let _ = self.to_emulator.send(FrontendMessage::Reset);
            }
            AsyncFrontendMessage::CreateSavestate => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::CreateSaveState(SaveType::Manual));
            }
            AsyncFrontendMessage::SetPalette(palette) => {
                self.handle_set_palette(ctx, palette);
            }
            AsyncFrontendMessage::WritePpuPalette {
                address,
                value,
            } => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::WritePpu(address, value));
                let _ = self.to_emulator.send(FrontendMessage::RequestDebugData(
                    EmulatorFetchable::Palettes(None),
                ));
            }
            AsyncFrontendMessage::WritePpuPattern {
                addr_0,
                value_0,
                addr_1,
                value_1,
            } => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::WritePpu(addr_0, value_0));
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::WritePpu(addr_1, value_1));
                let _ = self.to_emulator.send(FrontendMessage::RequestDebugData(
                    EmulatorFetchable::Tiles(None),
                ));
            }
            AsyncFrontendMessage::ControllerInput(event) => {
                let _ = self
                    .to_emulator
                    .send(FrontendMessage::ControllerInput(event));
            }
        }
    }

    fn handle_palette_loaded(
        &mut self,
        ctx: &Context,
        palette: RgbPalette,
        path: Option<std::path::PathBuf>,
    ) {
        self.config.view_config.palette_rgb_data = palette;
        self.config.user_config.previous_palette_path = path;
        // Update the renderer's palette
        self.config.view_config.renderer.set_palette(palette);
        // Re-render the current frame with the new palette
        self.emu_textures.update_emulator_texture(ctx, &mut self.config.view_config.renderer);
        if self.is_tile_viewer_visible() {
            self.emu_textures.update_tile_textures(
                ctx,
                &self.config.view_config.palette_rgb_data,
                None,
                None,
            );
        }
    }

    fn handle_rom_selected_for_savestate(
        &mut self,
        context: &SavestateLoadContext,
        rom_path: &Path,
    ) {
        match util::compute_file_checksum(rom_path) {
            Some(checksum) => {
                if checksum == context.savestate.rom_file.data_checksum {
                    self.load_savestate_with_rom(context, rom_path);
                } else {
                    self.config.pending_dialogs.checksum_mismatch_dialog =
                        Some(ChecksumMismatchDialogState {
                            context: Box::new(context.clone()),
                            selected_rom_path: rom_path.to_path_buf(),
                        });
                }
            }
            None => {
                self.config.pending_dialogs.error_dialog = Some(ErrorDialogState {
                    title: "Error Reading ROM".to_string(),
                    message: "Failed to read or compute checksum for the selected ROM file."
                        .to_string(),
                });
            }
        }
    }

    fn handle_savestate_load_failed(&mut self, error: SavestateLoadError) {
        let message = match error {
            SavestateLoadError::FailedToLoadSavestate => {
                "Failed to load or parse the savestate file. The file may be corrupted or invalid."
            }
            SavestateLoadError::FailedToComputeChecksum => {
                "Failed to compute checksum for the ROM file."
            }
        };
        self.config.pending_dialogs.error_dialog = Some(ErrorDialogState {
            title: "Savestate Load Error".to_string(),
            message: message.to_string(),
        });
    }

    fn handle_rom_verification_failed(
        &mut self,
        context: Box<SavestateLoadContext>,
        error: SavestateLoadError,
    ) {
        let message = match error {
            SavestateLoadError::FailedToLoadSavestate => {
                "Unexpected error during ROM verification."
            }
            SavestateLoadError::FailedToComputeChecksum => {
                "Failed to read or compute checksum for the ROM file."
            }
        };
        self.config.pending_dialogs.error_dialog = Some(ErrorDialogState {
            title: "ROM Verification Error".to_string(),
            message: format!("{}\n\nWould you like to select a different ROM?", message),
        });
        self.config.pending_dialogs.rom_selection_dialog = Some(RomSelectionDialogState {
            context,
        });
    }

    fn handle_quickload(&mut self) {
        if let Some(path) = self.get_current_quicksave_path() {
            let savestate = match savestate::try_load_state(&path) {
                Some(s) => s,
                None => {
                    let _ = self
                        .async_sender
                        .send(AsyncFrontendMessage::SavestateLoadFailed(
                            SavestateLoadError::FailedToLoadSavestate,
                        ));
                    return;
                }
            };

            let context = SavestateLoadContext {
                savestate,
                savestate_path: path,
            };

            if let Some(rom_path) = self.config.user_config.previous_rom_path.clone() {
                self.load_savestate_with_rom(&context, &rom_path)
            }
        }
    }

    fn handle_set_palette(&mut self, ctx: &Context, palette: RgbPalette) {
        self.config.view_config.palette_rgb_data = palette;
        // Update the renderer's palette
        self.config.view_config.renderer.set_palette(palette);
        // Re-render the current frame with the new palette
        self.emu_textures.update_emulator_texture(ctx, &mut self.config.view_config.renderer);
        if self.is_tile_viewer_visible() {
            self.emu_textures.update_tile_textures(
                ctx,
                &self.config.view_config.palette_rgb_data,
                None,
                None,
            );
        }
    }
}
