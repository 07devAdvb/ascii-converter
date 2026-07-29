//! Reactive application state shared through Leptos context.

use artscii_core::{ConvertConfig, DitheringStrategy};
use artscii_img::{AsciiResult, convert_image};
use leptos::prelude::*;

use crate::render::render_fragment;

pub(crate) const MIN_FONT_SIZE: u32 = 4;
pub(crate) const MAX_FONT_SIZE: u32 = 16;

/// Clamp the preview font size to the zoom bounds.
pub(crate) fn clamp_font_size(size: u32) -> u32 {
    size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE)
}

/// Conversion parameters shown in the sidebar.
#[derive(Clone, Copy)]
pub struct Settings {
    pub resolution: RwSignal<f64>,
    pub contrast: RwSignal<f64>,
    pub brightness: RwSignal<f64>,
    pub inverted: RwSignal<bool>,
    pub colored: RwSignal<bool>,
    pub dithering: RwSignal<DitheringStrategy>,
}

impl Settings {
    fn new() -> Self {
        let default = ConvertConfig::default();
        Self {
            resolution: RwSignal::new(f64::from(default.resolution)),
            contrast: RwSignal::new(f64::from(default.contrast)),
            brightness: RwSignal::new(f64::from(default.brightness)),
            inverted: RwSignal::new(default.inverted),
            colored: RwSignal::new(default.colored),
            dithering: RwSignal::new(default.dithering),
        }
    }

    /// Snapshot the current values as a core conversion config.
    fn config(self) -> ConvertConfig {
        ConvertConfig {
            resolution: self.resolution.get() as f32,
            contrast: self.contrast.get() as f32,
            brightness: self.brightness.get() as f32,
            inverted: self.inverted.get(),
            colored: self.colored.get(),
            dithering: self.dithering.get(),
        }
    }
}

/// Signals and derived values shared by every component.
#[derive(Clone, Copy)]
pub struct AppState {
    pub image: RwSignal<Option<image::DynamicImage>>,
    pub file_name: RwSignal<Option<String>>,
    pub thumb_url: RwSignal<Option<String>>,
    pub error: RwSignal<Option<String>>,
    pub is_dragging: RwSignal<bool>,
    pub copied: RwSignal<bool>,
    pub font_size: RwSignal<u32>,
    pub settings: Settings,
    pub result: Memo<Option<AsciiResult>>,
    pub html_output: Memo<Option<String>>,
    pub plain_text: Memo<Option<String>>,
    pub dims: Memo<Option<(usize, usize)>>,
}

impl AppState {
    pub fn new() -> Self {
        let image = RwSignal::new(None);
        let settings = Settings::new();

        // The image is decoded once on upload; conversion re-runs only when
        // the image or any setting changes.
        let result = Memo::new(move |_| {
            image.with(|img| convert_image(img.as_ref()?, &settings.config()).ok())
        });
        let html_output = Memo::new(move |_| result.with(|r| r.as_ref().map(render_fragment)));
        let plain_text =
            Memo::new(move |_| result.with(|r| r.as_ref().map(AsciiResult::to_plain_text)));
        let dims = Memo::new(move |_| result.with(|r| r.as_ref().map(|a| (a.width, a.height))));

        Self {
            image,
            file_name: RwSignal::new(None),
            thumb_url: RwSignal::new(None),
            error: RwSignal::new(None),
            is_dragging: RwSignal::new(false),
            copied: RwSignal::new(false),
            font_size: RwSignal::new(8),
            settings,
            result,
            html_output,
            plain_text,
            dims,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_size_stays_within_zoom_bounds() {
        assert_eq!(clamp_font_size(MIN_FONT_SIZE - 1), MIN_FONT_SIZE);
        assert_eq!(clamp_font_size(MAX_FONT_SIZE + 1), MAX_FONT_SIZE);
        assert_eq!(clamp_font_size(8), 8);
    }
}
