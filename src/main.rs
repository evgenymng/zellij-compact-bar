mod config;
mod util;

use std::collections::BTreeMap;

use config::Config;
use unicode_width::UnicodeWidthStr;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

struct Segment {
    /// Segment's name (tab name).
    text: String,
    /// Segment's text color.
    fg: PaletteColor,
    /// Segment's background color.
    bg: PaletteColor,
}

#[derive(Default)]
struct CompactBarPlugin {
    /// Plugin's configuration.
    config: Config,
    /// Information about open tabs.
    tabs: Vec<TabInfo>,
    /// Information about current mode.
    mode_info: ModeInfo,
}

register_plugin!(CompactBarPlugin);

impl ZellijPlugin for CompactBarPlugin {
    fn load(&mut self, settings: BTreeMap<String, String>) {
        set_selectable(true);

        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);

        subscribe(&[
            EventType::TabUpdate,
            EventType::ModeUpdate,
            EventType::PermissionRequestResult,
        ]);

        self.config = Config::from_settings(&settings);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    self.mode_info = mode_info;
                    return true;
                }
            }
            Event::TabUpdate(tabs) => {
                self.tabs = tabs;
                return true;
            }
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                set_selectable(false);
                return true;
            }
            _ => {}
        }
        false
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        if self.tabs.is_empty() {
            return;
        }

        let mode = self.mode_info.mode;
        let tabs = self
            .tabs
            .iter()
            .map(|tab| tab_segment(tab, mode, &self.config));
        let line = line_segments(cols, tabs, mode, &self.config);

        print(line, &self.config);
    }
}

fn is_default_name(tab: &TabInfo) -> bool {
    tab.name.starts_with("Tab #")
}

fn tab_segment(tab: &TabInfo, mode: InputMode, config: &Config) -> Segment {
    let text = if tab.active && tab.name.is_empty() && matches!(mode, InputMode::RenameTab) {
        "..."
    } else if is_default_name(tab) {
        "_"
    } else {
        &tab.name
    };

    Segment {
        text: format!("{}:{text} ", tab.position + 1),
        fg: if tab.active {
            config.color.active
        } else {
            config.color.inactive
        },
        bg: config.color.text_background,
    }
}

fn prefix(mode: InputMode, config: &Config) -> Segment {
    let fg = match mode {
        InputMode::Locked => config.color.dot_locked,
        InputMode::Normal => config.color.dot_normal,
        _ => config.color.dot_action,
    };

    Segment {
        text: format!(" {} ", config.general.dot),
        fg,
        bg: config.color.pane_background,
    }
}

fn line_segments(
    cols: usize,
    tabs: impl Iterator<Item = Segment>,
    mode: InputMode,
    config: &Config,
) -> impl Iterator<Item = Segment> {
    vec![prefix(mode, config)]
        .into_iter()
        .chain(tabs)
        .scan(cols as isize, |remaining, segment| {
            *remaining -= segment.text.width() as isize;
            (*remaining >= 0).then_some(segment)
        })
}

fn print(segments: impl Iterator<Item = Segment>, config: &Config) {
    let render_segment = |segment: Segment| {
        style!(segment.fg, segment.bg)
            .paint(segment.text)
            .to_string()
    };
    let rendered = segments.map(render_segment).collect::<String>();

    match config.color.pane_background {
        PaletteColor::Rgb((r, g, b)) => {
            print!("{}\u{1b}[48;2;{};{};{}m\u{1b}[0K", rendered, r, g, b);
        }
        PaletteColor::EightBit(color) => {
            print!("{}\u{1b}[48;5;{}m\u{1b}[0K", rendered, color);
        }
    }
}
