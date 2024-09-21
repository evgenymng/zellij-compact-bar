use std::collections::BTreeMap;
use zellij_tile::prelude::*;

use crate::util::ColorParser;

#[derive(Default)]
pub struct Config {
    /// Plugin's color configuration.
    pub(crate) color: ColorConfig,
    /// Plugin's general configuration.
    pub(crate) general: GeneralConfig,
}

impl Config {
    /// Creates a new `ColorConfig` from the plugin's settings.
    pub fn from_settings(settings: &BTreeMap<String, String>) -> Self {
        let parser = ColorParser::new();
        let def = Config::default();

        let active = settings.get("active_color").and_then(|s| parser.hex_rgb(s));
        let inactive = settings
            .get("inactive_color")
            .and_then(|s| parser.hex_rgb(s));
        let text_bg = settings
            .get("text_background_color")
            .and_then(|s| parser.hex_rgb(s));
        let pane_bg = settings
            .get("background_color")
            .and_then(|s| parser.hex_rgb(s));
        let dot = settings.get("dot").unwrap_or(&def.general.dot);
        let dot_locked = settings
            .get("dot_locked_color")
            .and_then(|s| parser.hex_rgb(s));
        let dot_normal = settings
            .get("dot_normal_color")
            .and_then(|s| parser.hex_rgb(s));
        let dot_action = settings
            .get("dot_action_color")
            .and_then(|s| parser.hex_rgb(s));

        Self {
            color: ColorConfig {
                active: active.unwrap_or(def.color.active),
                inactive: inactive.unwrap_or(def.color.inactive),
                text_background: text_bg.unwrap_or(def.color.text_background),
                pane_background: pane_bg.unwrap_or(def.color.pane_background),
                dot_locked: dot_locked.unwrap_or(def.color.dot_locked),
                dot_normal: dot_normal.unwrap_or(def.color.dot_normal),
                dot_action: dot_action.unwrap_or(def.color.dot_action),
            },
            general: GeneralConfig {
                dot: if dot.is_empty() {
                    "".to_owned()
                } else {
                    dot[..1].to_owned()
                },
            },
        }
    }
}

const DEFAULT_DOT: &str = "•";

pub struct GeneralConfig {
    /// A dot character to use in the prefix (reflects mode).
    /// Only the first character is used (if any).
    pub(crate) dot: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            dot: DEFAULT_DOT.to_owned(),
        }
    }
}

pub struct ColorConfig {
    /// Active text color.
    pub(crate) active: PaletteColor,
    /// Inactive text color.
    pub(crate) inactive: PaletteColor,
    /// Segment's background color.
    pub(crate) text_background: PaletteColor,
    /// Pane's background color (space, not taken by any of the segments).
    pub(crate) pane_background: PaletteColor,
    /// Prefix's dot color in the locked mode.
    pub(crate) dot_locked: PaletteColor,
    /// Prefix's dot color in the normal mode.
    pub(crate) dot_normal: PaletteColor,
    /// Prefix's dot color in any other mode.
    pub(crate) dot_action: PaletteColor,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            active: PaletteColor::Rgb((240, 240, 240)),
            inactive: PaletteColor::Rgb((140, 140, 140)),
            text_background: PaletteColor::Rgb((20, 20, 20)),
            pane_background: PaletteColor::Rgb((20, 20, 20)),
            dot_locked: PaletteColor::Rgb((60, 60, 60)),
            dot_normal: PaletteColor::Rgb((90, 120, 50)),
            dot_action: PaletteColor::Rgb((140, 120, 50)),
        }
    }
}
