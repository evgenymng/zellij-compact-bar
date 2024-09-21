use regex::Regex;
use zellij_tile::prelude::*;

pub(crate) struct ColorParser {
    color_re: Regex,
}

impl ColorParser {
    pub(crate) fn new() -> Self {
        Self {
            color_re: Regex::new(r"^#[0-9a-fA-F]{6}$").expect("didn't compile"),
        }
    }

    /// Parses color in the form `#aa90ff`
    pub(crate) fn hex_rgb(&self, s: &str) -> Option<PaletteColor> {
        if !self.color_re.is_match(s) {
            None
        } else {
            let s = &s[1..];
            let r = u8::from_str_radix(&s[0..2], 16).expect("unexpected hex byte");
            let g = u8::from_str_radix(&s[2..4], 16).expect("unexpected hex byte");
            let b = u8::from_str_radix(&s[4..6], 16).expect("unexpected hex byte");
            Some(PaletteColor::Rgb((r, g, b)))
        }
    }
}
