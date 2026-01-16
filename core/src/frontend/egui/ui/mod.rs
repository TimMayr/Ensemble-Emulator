//! UI components for the emulator frontend

mod emulator_output;
mod nametable_pane;
mod options_pane;
mod pattern_table;
mod pattern_table_pane;
mod status_bar;

pub use emulator_output::render_emulator_output;
pub use nametable_pane::render_nametable;
pub use options_pane::render_options;
pub use pattern_table::draw_pattern_table;
pub use pattern_table_pane::render_pattern_table;
pub use status_bar::add_status_bar;
