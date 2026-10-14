//! Terminal color capability detection and [`Theme`] downgrade.

use crate::color::{to_ansi16, to_ansi256};
use crate::theme::Theme;
use ratatui::style::Color;

/// How many colors the terminal can show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ColorSupport {
    /// No colors at all (`NO_COLOR` set or `TERM=dumb`).
    None,
    /// The 16 basic ANSI colors.
    Ansi16,
    /// The xterm 256-color palette.
    Ansi256,
    /// 24-bit RGB.
    TrueColor,
}

impl ColorSupport {
    /// Detect support from the process environment (`NO_COLOR`, `COLORTERM`, `TERM`).
    #[must_use]
    pub fn detect() -> Self {
        let get = |k: &str| std::env::var(k).ok();
        Self::from_env(
            get("NO_COLOR").as_deref(),
            get("COLORTERM").as_deref(),
            get("TERM").as_deref(),
        )
    }

    /// Pure detection logic behind [`ColorSupport::detect`], for testing and
    /// for apps that read the environment themselves.
    ///
    /// * a non-empty `no_color` → [`ColorSupport::None`] (<https://no-color.org>)
    /// * `COLORTERM` of `truecolor`/`24bit`, or `TERM` containing `direct` → true color
    /// * `TERM` containing `256color` → 256 colors
    /// * `TERM` of `dumb` → none; anything else → 16 colors
    #[must_use]
    pub fn from_env(no_color: Option<&str>, colorterm: Option<&str>, term: Option<&str>) -> Self {
        if no_color.is_some_and(|v| !v.is_empty()) {
            return Self::None;
        }
        let colorterm = colorterm.unwrap_or("").to_ascii_lowercase();
        let term = term.unwrap_or("").to_ascii_lowercase();
        if colorterm.contains("truecolor") || colorterm.contains("24bit") || term.contains("direct")
        {
            Self::TrueColor
        } else if term.contains("256color") {
            Self::Ansi256
        } else if term == "dumb" {
            Self::None
        } else {
            Self::Ansi16
        }
    }

    /// Reduce one color to what this support level can show.
    #[must_use]
    pub fn downgrade_color(self, color: Color) -> Color {
        match self {
            Self::TrueColor => color,
            Self::Ansi256 => to_ansi256(color),
            Self::Ansi16 => to_ansi16(color),
            Self::None => Color::Reset,
        }
    }
}

impl Theme {
    /// Reduce every color of the theme to what `support` can display.
    ///
    /// `TrueColor` is a no-op, `Ansi256` maps RGB to [`Color::Indexed`],
    /// `Ansi16` maps everything to the 16 named colors and `None` resets all
    /// colors (and clears background/surface) so the terminal default is used.
    #[must_use]
    pub fn downgrade(&self, support: ColorSupport) -> Theme {
        let mut t = self.clone();
        t.map_colors(|c| support.downgrade_color(c));
        if support == ColorSupport::None {
            t.background = None;
            t.surface = None;
        }
        t
    }

    /// [`Theme::downgrade`] using [`ColorSupport::detect`].
    #[must_use]
    pub fn downgrade_for_terminal(&self) -> Theme {
        self.downgrade(ColorSupport::detect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_wins() {
        let s = ColorSupport::from_env(Some("1"), Some("truecolor"), Some("xterm-256color"));
        assert_eq!(s, ColorSupport::None);
    }

    #[test]
    fn empty_no_color_ignored() {
        let s = ColorSupport::from_env(Some(""), Some("truecolor"), None);
        assert_eq!(s, ColorSupport::TrueColor);
    }

    #[test]
    fn colorterm_variants() {
        assert_eq!(
            ColorSupport::from_env(None, Some("24bit"), None),
            ColorSupport::TrueColor
        );
        assert_eq!(
            ColorSupport::from_env(None, Some("TrueColor"), Some("xterm")),
            ColorSupport::TrueColor
        );
    }

    #[test]
    fn term_variants() {
        let f = |t| ColorSupport::from_env(None, None, Some(t));
        assert_eq!(f("xterm-256color"), ColorSupport::Ansi256);
        assert_eq!(f("xterm-direct"), ColorSupport::TrueColor);
        assert_eq!(f("dumb"), ColorSupport::None);
        assert_eq!(f("xterm"), ColorSupport::Ansi16);
        assert_eq!(
            ColorSupport::from_env(None, None, None),
            ColorSupport::Ansi16
        );
    }

    #[test]
    fn ordering_by_capability() {
        assert!(ColorSupport::None < ColorSupport::Ansi16);
        assert!(ColorSupport::Ansi256 < ColorSupport::TrueColor);
    }

    #[test]
    fn downgrade_color_levels() {
        let c = Color::Rgb(255, 0, 0);
        assert_eq!(ColorSupport::TrueColor.downgrade_color(c), c);
        assert_eq!(
            ColorSupport::Ansi256.downgrade_color(c),
            Color::Indexed(196)
        );
        assert_eq!(ColorSupport::Ansi16.downgrade_color(c), Color::LightRed);
        assert_eq!(ColorSupport::None.downgrade_color(c), Color::Reset);
    }

    #[test]
    fn truecolor_downgrade_is_identity() {
        let t = Theme::dracula();
        assert_eq!(t.downgrade(ColorSupport::TrueColor), t);
    }

    #[test]
    fn none_clears_backgrounds() {
        let t = Theme::dracula().downgrade(ColorSupport::None);
        assert_eq!(t.background, None);
        assert_eq!(t.surface, None);
        assert_eq!(t.foreground, Color::Reset);
    }
}
