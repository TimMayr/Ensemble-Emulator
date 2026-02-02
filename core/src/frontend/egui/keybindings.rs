//! Keybindings configuration for the emulator.
//!
//! This module provides configurable keybindings for:
//! - NES controller inputs (Up, Down, Left, Right, A, B, Start, Select)
//! - Emulation controls (Pause, Reset, Quicksave, Quickload)
//! - Debug controls (Cycle palette)

use egui::{Key, KeyboardShortcut, Modifiers};
use serde::{Deserialize, Serialize};

/// A wrapper around `Option<KeyboardShortcut>` that provides custom serialization.
///
/// This is needed because `KeyboardShortcut` doesn't implement Serialize/Deserialize
/// directly in a TOML-friendly format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keybind(pub Option<KeyboardShortcut>);

impl Default for Keybind {
    fn default() -> Self {
        Self(None)
    }
}

impl Keybind {
    /// Create a new keybind from a key without modifiers.
    pub const fn key(key: Key) -> Self {
        Self(Some(KeyboardShortcut {
            modifiers: Modifiers::NONE,
            logical_key: key,
        }))
    }

    /// Create a new keybind from a key with modifiers.
    pub const fn with_modifiers(modifiers: Modifiers, key: Key) -> Self {
        Self(Some(KeyboardShortcut {
            modifiers,
            logical_key: key,
        }))
    }

    /// Create an unbound keybind.
    pub const fn none() -> Self {
        Self(None)
    }
}

impl Serialize for Keybind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            Some(shortcut) => {
                let formatted = shortcut.format(&egui::ModifierNames::NAMES, false);
                serializer.serialize_str(&formatted)
            }
            None => serializer.serialize_str("None"),
        }
    }
}

impl<'de> Deserialize<'de> for Keybind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "None" || s.is_empty() {
            return Ok(Keybind(None));
        }

        // Parse the formatted string back into a KeyboardShortcut
        // Format is like "Ctrl+Shift+A" or just "A"
        let mut modifiers = Modifiers::NONE;
        let parts: Vec<&str> = s.split('+').collect();

        let key_name = parts.last().unwrap_or(&"");
        for part in &parts[..parts.len().saturating_sub(1)] {
            match *part {
                "Ctrl" | "⌘" => modifiers |= Modifiers::CTRL,
                "Shift" | "⇧" => modifiers |= Modifiers::SHIFT,
                "Alt" | "⌥" => modifiers |= Modifiers::ALT,
                "Cmd" | "Meta" => modifiers |= Modifiers::COMMAND,
                _ => {}
            }
        }

        let key = parse_key_name(key_name);
        match key {
            Some(k) => Ok(Keybind(Some(KeyboardShortcut {
                modifiers,
                logical_key: k,
            }))),
            None => Ok(Keybind(None)),
        }
    }
}

/// Parse a key name string to an egui Key.
fn parse_key_name(name: &str) -> Option<Key> {
    // Match against common key names
    match name.trim() {
        "A" => Some(Key::A),
        "B" => Some(Key::B),
        "C" => Some(Key::C),
        "D" => Some(Key::D),
        "E" => Some(Key::E),
        "F" => Some(Key::F),
        "G" => Some(Key::G),
        "H" => Some(Key::H),
        "I" => Some(Key::I),
        "J" => Some(Key::J),
        "K" => Some(Key::K),
        "L" => Some(Key::L),
        "M" => Some(Key::M),
        "N" => Some(Key::N),
        "O" => Some(Key::O),
        "P" => Some(Key::P),
        "Q" => Some(Key::Q),
        "R" => Some(Key::R),
        "S" => Some(Key::S),
        "T" => Some(Key::T),
        "U" => Some(Key::U),
        "V" => Some(Key::V),
        "W" => Some(Key::W),
        "X" => Some(Key::X),
        "Y" => Some(Key::Y),
        "Z" => Some(Key::Z),
        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),
        "Enter" => Some(Key::Enter),
        "Escape" => Some(Key::Escape),
        "Backspace" => Some(Key::Backspace),
        "Delete" => Some(Key::Delete),
        "Insert" => Some(Key::Insert),
        "Home" => Some(Key::Home),
        "End" => Some(Key::End),
        "PageUp" => Some(Key::PageUp),
        "PageDown" => Some(Key::PageDown),
        "ArrowLeft" | "Left" | "←" => Some(Key::ArrowLeft),
        "ArrowRight" | "Right" | "→" => Some(Key::ArrowRight),
        "ArrowUp" | "Up" | "↑" => Some(Key::ArrowUp),
        "ArrowDown" | "Down" | "↓" => Some(Key::ArrowDown),
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "-" | "Minus" => Some(Key::Minus),
        "=" | "Equals" => Some(Key::Equals),
        "[" | "OpenBracket" => Some(Key::OpenBracket),
        "]" | "CloseBracket" => Some(Key::CloseBracket),
        ";" | "Semicolon" => Some(Key::Semicolon),
        "'" | "Quote" => Some(Key::Quote),
        "`" | "Backtick" => Some(Key::Backtick),
        "\\" | "Backslash" => Some(Key::Backslash),
        "," | "Comma" => Some(Key::Comma),
        "." | "Period" => Some(Key::Period),
        "/" | "Slash" => Some(Key::Slash),
        _ => None,
    }
}

/// Keybindings for NES controller (Player 1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControllerKeybindings {
    pub up: Keybind,
    pub down: Keybind,
    pub left: Keybind,
    pub right: Keybind,
    pub a: Keybind,
    pub b: Keybind,
    pub start: Keybind,
    pub select: Keybind,
}

impl Default for ControllerKeybindings {
    fn default() -> Self {
        Self {
            up: Keybind::key(Key::ArrowUp),
            down: Keybind::key(Key::ArrowDown),
            left: Keybind::key(Key::ArrowLeft),
            right: Keybind::key(Key::ArrowRight),
            a: Keybind::key(Key::Space),
            b: Keybind::with_modifiers(Modifiers::SHIFT, Key::Space),
            start: Keybind::key(Key::S),
            select: Keybind::key(Key::Tab),
        }
    }
}

/// Keybindings for emulation controls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmulationKeybindings {
    pub pause: Keybind,
    pub reset: Keybind,
    pub quicksave: Keybind,
    pub quickload: Keybind,
}

impl Default for EmulationKeybindings {
    fn default() -> Self {
        Self {
            pause: Keybind::key(Key::Period),
            reset: Keybind::key(Key::R),
            quicksave: Keybind::key(Key::F5),
            quickload: Keybind::key(Key::F8),
        }
    }
}

/// Keybindings for debug controls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DebugKeybindings {
    pub cycle_palette: Keybind,
}

impl Default for DebugKeybindings {
    fn default() -> Self {
        Self {
            cycle_palette: Keybind::key(Key::N),
        }
    }
}

/// All keybindings for the emulator
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct KeybindingsConfig {
    pub controller: ControllerKeybindings,
    pub emulation: EmulationKeybindings,
    pub debug: DebugKeybindings,
}

impl KeybindingsConfig {
    /// Reset all keybindings to defaults.
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }
}
