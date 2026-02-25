use std::fmt::{Debug, Formatter};

use crate::emulation::palette_util::{RgbColor, RgbPalette};

/// Trait for rendering palette indices to RGB colors.
///
/// Implementations must be serializable to enable persistence of renderer state.
pub trait ScreenRenderer: Debug {
    /// Convert a buffer of palette indices to RGB colors.
    fn buffer_to_image(&mut self, buffer: &[u16]) -> &[RgbColor];

    /// Set the palette to use for rendering.
    ///
    /// Called when the user loads a new palette file.
    fn set_palette(&mut self, palette: RgbPalette);

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_id(&self) -> &'static str;
    fn get_display_name(&self) -> &'static str;
}

type RendererFactory = fn() -> Box<dyn ScreenRenderer>;

#[derive(Copy, Clone, Eq)]
pub struct RendererRegistration {
    pub key: &'static str,
    pub display_name: &'static str,
    pub factory: RendererFactory,
}

impl PartialEq for RendererRegistration {
    fn eq(&self, other: &Self) -> bool { self.key == other.key }
}

#[macro_export]
macro_rules! declare_renderers {
    ($($ty:ty),* $(,)?) => {
        pub fn get_all_renderers() -> Vec<RendererRegistration> {
            vec![
                $(
                    RendererRegistration {
                        key: <$ty>::new().get_id(),
                        display_name: <$ty>::new().get_display_name(),
                        factory: || Box::new(<$ty>::new()),
                    }
                ),*
            ]
        }
    };
}

pub fn create_renderer(
    name: Option<&str>,
    renderers: Vec<RendererRegistration>,
) -> Box<dyn ScreenRenderer> {
    if let Some(name) = name {
        let renderer = renderers.iter().find(|r| r.key == name);

        if let Some(renderer) = renderer {
            (renderer.factory)()
        } else {
            Box::new(NoneRenderer::new())
        }
    } else {
        Box::new(NoneRenderer::new())
    }
}

pub struct NoneRenderer {
    image: [RgbColor; 1],
}

impl Default for NoneRenderer {
    fn default() -> Self { Self::new() }
}

impl NoneRenderer {
    pub fn new() -> Self {
        NoneRenderer {
            image: [RgbColor::default()],
        }
    }
}

impl Debug for NoneRenderer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_display_name())
    }
}

impl ScreenRenderer for NoneRenderer {
    fn buffer_to_image(&mut self, _: &[u16]) -> &[RgbColor] { &self.image }

    fn set_palette(&mut self, _: RgbPalette) {}

    fn get_width(&self) -> usize { 1 }

    fn get_height(&self) -> usize { 1 }

    fn get_id(&self) -> &'static str { "none" }

    fn get_display_name(&self) -> &'static str { "None" }
}
