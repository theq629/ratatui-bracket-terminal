use bracket_terminal::prelude::{self as bracket_prelude, RGBA};
use ratatui::style::{Color, Modifier};

/// Handling of conversions from Ratatui [colours][Color] to Bracket [colours][RGBA].
pub trait ColourConverter {
    fn convert_fg(&self, color: Color, modifier: Modifier) -> RGBA;
    fn convert_bg(&self, color: Color, modifier: Modifier) -> RGBA;
}

/// Basic colour conversion.
///
/// - Ratatui reset is handled as default foreground or default background as appropriate.
/// - Ratatui RGB colours are converted directly.
/// - Ratatui named colours are converted to the most obvious corresponding named Bracket colour.
/// - Ratatui indexed colours are converted by a lookup table if given. If not given, they are just
///   converted to the appropriate default.
/// - Modifiers are ignored.
#[derive(Clone)]
pub struct BasicColourConverter {
    pub default_fg: RGBA,
    pub default_bg: RGBA,
    pub indexed_colours: Option<Vec<RGBA>>,
}

impl Default for BasicColourConverter {
    fn default() -> Self {
        Self {
            default_fg: bracket_prelude::WHITE.into(),
            default_bg: bracket_prelude::BLACK.into(),
            indexed_colours: None,
        }
    }
}

impl BasicColourConverter {
    fn convert_colour(&self, color: Color, default: RGBA) -> RGBA {
        match color {
            Color::Reset => default,
            Color::Black => bracket_prelude::BLACK.into(),
            Color::Red => bracket_prelude::DARKRED.into(),
            Color::Green => bracket_prelude::GREEN.into(),
            Color::Yellow => bracket_prelude::YELLOW.into(),
            Color::Blue => bracket_prelude::BLUE.into(),
            Color::Magenta => bracket_prelude::DARKMAGENTA.into(),
            Color::Cyan => bracket_prelude::CYAN.into(),
            Color::Gray => bracket_prelude::GRAY.into(),
            Color::DarkGray => bracket_prelude::DARKGRAY.into(),
            Color::LightRed => bracket_prelude::RED.into(),
            Color::LightGreen => bracket_prelude::LIGHTGREEN.into(),
            Color::LightYellow => bracket_prelude::LIGHTYELLOW.into(),
            Color::LightBlue => bracket_prelude::LIGHTBLUE.into(),
            Color::LightMagenta => bracket_prelude::MAGENTA.into(),
            Color::LightCyan => bracket_prelude::LIGHTCYAN.into(),
            Color::White => bracket_prelude::WHITE.into(),
            Color::Rgb(r, g, b) => (r, g, b).into(),
            Color::Indexed(i) => self
                .indexed_colours
                .as_ref()
                .and_then(|cs| cs.get(i as usize).copied())
                .unwrap_or(default),
        }
    }
}

impl ColourConverter for BasicColourConverter {
    fn convert_fg(&self, color: Color, _modifier: Modifier) -> RGBA {
        self.convert_colour(color, self.default_fg)
    }

    fn convert_bg(&self, color: Color, _modifier: Modifier) -> RGBA {
        self.convert_colour(color, self.default_bg)
    }
}
