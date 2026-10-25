//! The [`Theme`] struct: one named set of colors for every widget.

use crate::color::{mix, readable_on, to_rgb};
use ratatui::style::Color;

/// Default series palette, used when a theme does not bring its own.
pub const DEFAULT_PALETTE: [Color; 8] = [
    Color::Cyan,
    Color::Yellow,
    Color::Magenta,
    Color::Green,
    Color::LightRed,
    Color::LightBlue,
    Color::LightGreen,
    Color::White,
];

/// Compact color spec the named themes are generated from (hex `0xRRGGBB`).
pub(crate) struct Spec {
    pub bg: u32,
    pub fg: u32,
    pub muted: u32,
    pub surface: u32,
    pub accent: u32,
    pub secondary: u32,
    pub success: u32,
    pub warning: u32,
    pub error: u32,
    pub info: u32,
    pub palette: &'static [u32],
}

/// `0xRRGGBB` → [`Color::Rgb`].
pub(crate) const fn rgb(hex: u32) -> Color {
    Color::Rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// A named set of colors shared by every widget of an application.
///
/// The first group of fields (`foreground` … `highlight`) is the general UI
/// vocabulary; the second group (`palette` … `tooltip_bg`) are the data
/// visualisation colors, kept under the same names `tui-chart` used so
/// migrating is a rename of the import.
///
/// With the `serde` feature a theme (de)serialises as a flat map of
/// `"#rrggbb"` / color-name strings; missing keys fall back to
/// [`Theme::default`], and unset `background`/`surface` are omitted.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Theme {
    /// Display name (also used by [`Theme::next`] / [`Theme::prev`]).
    pub name: String,
    /// Background fill; `None` keeps the terminal background.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub background: Option<Color>,
    /// Panel / card background; `None` means "same as background".
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub surface: Option<Color>,
    /// Default text color.
    pub foreground: Color,
    /// De-emphasised text (hints, tick labels, legends).
    pub muted: Color,
    /// Text of disabled elements.
    pub disabled: Color,
    /// Unfocused border color.
    pub border: Color,
    /// Focused border color.
    pub border_focused: Color,
    /// Title color.
    pub title: Color,
    /// Main brand / interaction color.
    pub primary: Color,
    /// Secondary brand color.
    pub secondary: Color,
    /// Accent color (single-series charts, gradients, highlights).
    pub accent: Color,
    /// Positive status color.
    pub success: Color,
    /// Warning status color.
    pub warning: Color,
    /// Error status color.
    pub error: Color,
    /// Informational status color.
    pub info: Color,
    /// Background of the selected item.
    pub selection_bg: Color,
    /// Foreground of the selected item.
    pub selection_fg: Color,
    /// Highlight color (search matches, markers).
    pub highlight: Color,
    /// Series palette (at least 8 colors); series `i` uses `palette[i % len]`.
    pub palette: Vec<Color>,
    /// "Up" color (bullish candles, bids, increases).
    pub bull: Color,
    /// "Down" color (bearish candles, asks, decreases).
    pub bear: Color,
    /// Neutral / total color.
    pub neutral: Color,
    /// Axis line color.
    pub axis: Color,
    /// Grid line color.
    pub grid: Color,
    /// Crosshair guide line color.
    pub crosshair: Color,
    /// Crosshair tooltip text color.
    pub tooltip_fg: Color,
    /// Crosshair tooltip background color.
    pub tooltip_bg: Color,
}

impl Default for Theme {
    /// Terminal-native theme built from named ANSI colors only.
    fn default() -> Self {
        Self {
            name: "default".into(),
            background: None,
            surface: None,
            foreground: Color::White,
            muted: Color::Gray,
            disabled: Color::DarkGray,
            border: Color::Gray,
            border_focused: Color::Cyan,
            title: Color::White,
            primary: Color::Cyan,
            secondary: Color::Magenta,
            accent: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            selection_bg: Color::Blue,
            selection_fg: Color::White,
            highlight: Color::Yellow,
            palette: DEFAULT_PALETTE.to_vec(),
            bull: Color::Green,
            bear: Color::Red,
            neutral: Color::Blue,
            axis: Color::DarkGray,
            grid: Color::DarkGray,
            crosshair: Color::Gray,
            tooltip_fg: Color::White,
            tooltip_bg: Color::DarkGray,
        }
    }
}

impl Theme {
    /// Build a theme from the handful of colors an application already has.
    ///
    /// Everything else is derived: RGB (and named/indexed) colors are blended
    /// between `background` (black when `None`) and `foreground`, so muted
    /// text, grid, borders, disabled text, surface and selection stay in
    /// tune. Success/bull use `bull`, error/bear use `bear`, primary/accent use
    /// `accent`; the palette starts with `accent, bull, bear`. Use the builder
    /// setters afterwards to fine-tune.
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_nuances::Theme;
    ///
    /// let t = Theme::from_base(
    ///     "app",
    ///     Some(Color::Rgb(16, 16, 24)),
    ///     Color::Rgb(230, 230, 240),
    ///     Color::Rgb(213, 0, 28),
    ///     Color::Green,
    ///     Color::Red,
    /// );
    /// assert_eq!(t.primary, Color::Rgb(213, 0, 28));
    /// assert!(t.palette.len() >= 8);
    /// ```
    #[must_use]
    pub fn from_base(
        name: &str,
        background: Option<Color>,
        foreground: Color,
        accent: Color,
        bull: Color,
        bear: Color,
    ) -> Self {
        let base = background.unwrap_or(Color::Rgb(0, 0, 0));
        let blend = |t: f64, fallback: Color| {
            if to_rgb(base).is_some() && to_rgb(foreground).is_some() {
                mix(base, foreground, t)
            } else {
                fallback
            }
        };
        let muted = blend(0.6, Color::Gray);
        let grid = blend(0.25, Color::DarkGray);
        let disabled = blend(0.4, Color::DarkGray);
        let surface = background.map(|_| blend(0.08, base));
        let selection_bg = if to_rgb(base).is_some() && to_rgb(accent).is_some() {
            mix(base, accent, 0.35)
        } else {
            accent
        };
        let mut palette = vec![accent, bull, bear];
        palette.extend(
            DEFAULT_PALETTE
                .iter()
                .copied()
                .filter(|c| ![accent, bull, bear].contains(c)),
        );
        Self {
            name: name.into(),
            background,
            surface,
            foreground,
            muted,
            disabled,
            border: blend(0.35, Color::Gray),
            border_focused: accent,
            title: foreground,
            primary: accent,
            secondary: blend(0.5, Color::Magenta),
            accent,
            success: bull,
            warning: Color::Yellow,
            error: bear,
            info: accent,
            selection_bg,
            selection_fg: readable_on(selection_bg),
            highlight: Color::Yellow,
            palette,
            bull,
            bear,
            neutral: accent,
            axis: grid,
            grid,
            crosshair: muted,
            tooltip_fg: foreground,
            tooltip_bg: surface.or(background).unwrap_or(grid),
        }
    }

    /// Build a theme from just a series palette (terminal background, white
    /// text). The first three colors become accent / bull / bear; the palette
    /// is cycled up to at least 8 entries. An empty slice yields the default
    /// palette.
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_nuances::Theme;
    ///
    /// let t = Theme::from_palette("pair", &[Color::Red, Color::Blue]);
    /// assert_eq!(t.palette.len(), 8);
    /// assert_eq!(t.color(2), Color::Red);
    /// ```
    #[must_use]
    pub fn from_palette(name: &str, colors: &[Color]) -> Self {
        let src: &[Color] = if colors.is_empty() {
            &DEFAULT_PALETTE
        } else {
            colors
        };
        let at = |i: usize| src[i % src.len()];
        let mut t = Self::from_base(name, None, Color::White, at(0), at(1), at(2));
        let n = src.len().max(8);
        t.palette = (0..n).map(at).collect();
        t
    }

    /// Expand a hex [`Spec`] into a full theme (used by the named themes).
    pub(crate) fn from_spec(name: &str, s: Spec) -> Self {
        let (bg, fg, accent) = (rgb(s.bg), rgb(s.fg), rgb(s.accent));
        let surface = rgb(s.surface);
        let selection_bg = mix(bg, accent, 0.35);
        let mut t = Self {
            name: name.into(),
            background: Some(bg),
            surface: Some(surface),
            foreground: fg,
            muted: rgb(s.muted),
            disabled: mix(bg, fg, 0.38),
            border: mix(bg, fg, 0.3),
            border_focused: accent,
            title: fg,
            primary: accent,
            secondary: rgb(s.secondary),
            accent,
            success: rgb(s.success),
            warning: rgb(s.warning),
            error: rgb(s.error),
            info: rgb(s.info),
            selection_bg,
            selection_fg: readable_on(selection_bg),
            highlight: rgb(s.warning),
            palette: s.palette.iter().map(|&c| rgb(c)).collect(),
            bull: rgb(s.success),
            bear: rgb(s.error),
            neutral: rgb(s.muted),
            axis: rgb(s.muted),
            grid: mix(bg, fg, 0.2),
            crosshair: rgb(s.muted),
            tooltip_fg: fg,
            tooltip_bg: surface,
        };
        crate::roles::apply(&mut t);
        t
    }

    /// Apply `f` to every color field (and palette entry).
    pub(crate) fn map_colors(&mut self, f: impl Fn(Color) -> Color) {
        self.background = self.background.map(&f);
        self.surface = self.surface.map(&f);
        for c in [
            &mut self.foreground,
            &mut self.muted,
            &mut self.disabled,
            &mut self.border,
            &mut self.border_focused,
            &mut self.title,
            &mut self.primary,
            &mut self.secondary,
            &mut self.accent,
            &mut self.success,
            &mut self.warning,
            &mut self.error,
            &mut self.info,
            &mut self.selection_bg,
            &mut self.selection_fg,
            &mut self.highlight,
            &mut self.bull,
            &mut self.bear,
            &mut self.neutral,
            &mut self.axis,
            &mut self.grid,
            &mut self.crosshair,
            &mut self.tooltip_fg,
            &mut self.tooltip_bg,
        ] {
            *c = f(*c);
        }
        for c in &mut self.palette {
            *c = f(*c);
        }
    }

    /// Every color of the theme (background and surface included when set,
    /// palette last). Handy for audits and tests.
    #[must_use]
    pub fn colors(&self) -> Vec<Color> {
        let mut t = self.clone();
        let out = std::cell::RefCell::new(Vec::new());
        t.map_colors(|c| {
            out.borrow_mut().push(c);
            c
        });
        out.into_inner()
    }

    /// Set the display name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the series palette (an empty palette is ignored).
    #[must_use]
    pub fn palette(mut self, palette: Vec<Color>) -> Self {
        if !palette.is_empty() {
            self.palette = palette;
        }
        self
    }

    /// Set (or clear) the background fill.
    #[must_use]
    pub fn background(mut self, background: Option<Color>) -> Self {
        self.background = background;
        self
    }

    /// Set (or clear) the panel / card background.
    #[must_use]
    pub fn surface(mut self, surface: Option<Color>) -> Self {
        self.surface = surface;
        self
    }

    color_setters!(
        foreground,
        muted,
        disabled,
        border,
        border_focused,
        title,
        primary,
        secondary,
        accent,
        success,
        warning,
        error,
        info,
        selection_bg,
        selection_fg,
        highlight,
        bull,
        bear,
        neutral,
        axis,
        grid,
        crosshair,
        tooltip_fg,
        tooltip_bg,
    );

    /// Set the tooltip foreground/background colors.
    #[must_use]
    pub fn tooltip(mut self, fg: Color, bg: Color) -> Self {
        self.tooltip_fg = fg;
        self.tooltip_bg = bg;
        self
    }

    /// Set the selection foreground/background colors.
    #[must_use]
    pub fn selection(mut self, fg: Color, bg: Color) -> Self {
        self.selection_fg = fg;
        self.selection_bg = bg;
        self
    }

    /// Palette color for series `i` (cycles; falls back to `accent` if empty).
    #[must_use]
    pub fn color(&self, i: usize) -> Color {
        if self.palette.is_empty() {
            self.accent
        } else {
            self.palette[i % self.palette.len()]
        }
    }
}

impl From<&Theme> for Theme {
    fn from(t: &Theme) -> Self {
        t.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Theme {
        Theme::from_base(
            "t",
            Some(Color::Rgb(0, 0, 0)),
            Color::Rgb(200, 200, 200),
            Color::Rgb(10, 20, 30),
            Color::Green,
            Color::Red,
        )
    }

    #[test]
    fn from_base_keeps_inputs() {
        let t = base();
        assert_eq!(t.name, "t");
        assert_eq!(t.background, Some(Color::Rgb(0, 0, 0)));
        assert_eq!(t.foreground, Color::Rgb(200, 200, 200));
        assert_eq!(
            (t.accent, t.primary),
            (Color::Rgb(10, 20, 30), Color::Rgb(10, 20, 30))
        );
        assert_eq!((t.bull, t.success), (Color::Green, Color::Green));
        assert_eq!((t.bear, t.error), (Color::Red, Color::Red));
    }

    #[test]
    fn from_base_blends_muted_and_grid() {
        let t = base();
        assert_eq!(t.muted, Color::Rgb(120, 120, 120));
        assert_eq!(t.grid, Color::Rgb(50, 50, 50));
        assert_eq!(t.axis, t.grid);
        assert_eq!(t.crosshair, t.muted);
    }

    #[test]
    fn from_base_palette_starts_with_base_colors() {
        let t = base();
        assert_eq!(&t.palette[..3], &[t.accent, Color::Green, Color::Red]);
        assert!(t.palette.len() >= 8);
        assert_eq!(t.palette.iter().filter(|c| **c == Color::Green).count(), 1);
    }

    #[test]
    fn from_base_without_background() {
        let t = Theme::from_base(
            "n",
            None,
            Color::Rgb(200, 200, 200),
            Color::Cyan,
            Color::Green,
            Color::Red,
        );
        assert_eq!(t.background, None);
        assert_eq!(t.surface, None);
        assert_eq!(t.muted, Color::Rgb(120, 120, 120));
    }

    #[test]
    fn from_base_unresolvable_falls_back() {
        let t = Theme::from_base(
            "r",
            None,
            Color::Reset,
            Color::Cyan,
            Color::Green,
            Color::Red,
        );
        assert_eq!(t.muted, Color::Gray);
        assert_eq!(t.grid, Color::DarkGray);
    }

    #[test]
    fn from_base_selection_is_readable() {
        let t = base();
        assert!(crate::color::contrast_ratio(t.selection_fg, t.selection_bg) >= 4.5);
    }

    #[test]
    fn from_palette_cycles_to_eight() {
        let t = Theme::from_palette("p", &[Color::Red, Color::Blue, Color::Green]);
        assert_eq!(t.palette.len(), 8);
        assert_eq!(t.palette[3], Color::Red);
        assert_eq!(
            (t.accent, t.bull, t.bear),
            (Color::Red, Color::Blue, Color::Green)
        );
    }

    #[test]
    fn from_palette_empty_uses_default() {
        let t = Theme::from_palette("e", &[]);
        assert_eq!(t.palette, DEFAULT_PALETTE.to_vec());
    }

    #[test]
    fn from_palette_keeps_long_palettes() {
        let long: Vec<Color> = (0..12).map(Color::Indexed).collect();
        assert_eq!(Theme::from_palette("l", &long).palette, long);
    }

    #[test]
    fn setters_override_fields() {
        let t = Theme::default()
            .name("x")
            .background(Some(Color::Black))
            .surface(Some(Color::Blue))
            .foreground(Color::Red)
            .primary(Color::Green)
            .tooltip(Color::Black, Color::White)
            .selection(Color::Cyan, Color::Magenta);
        assert_eq!(t.name, "x");
        assert_eq!(t.background, Some(Color::Black));
        assert_eq!(t.surface, Some(Color::Blue));
        assert_eq!(t.foreground, Color::Red);
        assert_eq!(t.primary, Color::Green);
        assert_eq!((t.tooltip_fg, t.tooltip_bg), (Color::Black, Color::White));
        assert_eq!(
            (t.selection_fg, t.selection_bg),
            (Color::Cyan, Color::Magenta)
        );
    }

    #[test]
    fn empty_palette_setter_ignored() {
        let before = Theme::default();
        assert_eq!(before.clone().palette(vec![]).palette, before.palette);
        assert_eq!(before.palette(vec![Color::Red]).palette, vec![Color::Red]);
    }

    #[test]
    fn color_cycles_and_handles_empty() {
        let t = Theme::default().palette(vec![Color::Red, Color::Blue]);
        assert_eq!(t.color(0), Color::Red);
        assert_eq!(t.color(3), Color::Blue);
        let mut e = Theme::default();
        e.palette.clear();
        assert_eq!(e.color(5), e.accent);
    }

    #[test]
    fn colors_lists_everything() {
        let t = base();
        assert!(t.colors().len() >= 24 + 8);
    }

    #[test]
    fn from_ref_clones() {
        let t = base();
        assert_eq!(Theme::from(&t), t);
    }
}
