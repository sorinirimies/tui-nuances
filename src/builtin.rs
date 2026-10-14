//! Built-in named themes and lookup / cycling helpers.
//!
//! Cycling order of [`Theme::all`]: `default`, the popular editor palettes
//! (`catppuccin-mocha` … `monokai`, `porsche`), then the accessibility themes
//! `colorblind`, `high-contrast` and `mono`.

use crate::theme::{rgb, Spec, Theme};
use ratatui::style::Color;

named_themes! {
    catppuccin_mocha, "catppuccin-mocha", "Catppuccin Mocha (dark)." =>
    [0x1e1e2e, 0xcdd6f4, 0x9399b2, 0x313244, 0x89b4fa, 0xcba6f7, 0xa6e3a1, 0xf9e2af, 0xf38ba8, 0x89dceb],
    [0x89b4fa, 0xf9e2af, 0xcba6f7, 0xa6e3a1, 0xf38ba8, 0x94e2d5, 0xfab387, 0xf5c2e7];
    catppuccin_latte, "catppuccin-latte", "Catppuccin Latte (light)." =>
    [0xeff1f5, 0x4c4f69, 0x7c7f93, 0xccd0da, 0x1e66f5, 0x8839ef, 0x40a02b, 0xdf8e1d, 0xd20f39, 0x04a5e5],
    [0x1e66f5, 0xdf8e1d, 0x8839ef, 0x40a02b, 0xd20f39, 0x179299, 0xfe640b, 0xea76cb];
    dracula, "dracula", "Dracula (dark)." =>
    [0x282a36, 0xf8f8f2, 0x8e9bc4, 0x44475a, 0xbd93f9, 0xff79c6, 0x50fa7b, 0xf1fa8c, 0xff5555, 0x8be9fd],
    [0xbd93f9, 0xff79c6, 0x50fa7b, 0xf1fa8c, 0xffb86c, 0x8be9fd, 0xff5555, 0x6272a4];
    nord, "nord", "Nord (dark, arctic blues)." =>
    [0x2e3440, 0xd8dee9, 0x8b96ab, 0x3b4252, 0x88c0d0, 0x81a1c1, 0xa3be8c, 0xebcb8b, 0xbf616a, 0x5e81ac],
    [0x88c0d0, 0xa3be8c, 0xebcb8b, 0xb48ead, 0xd08770, 0x81a1c1, 0xbf616a, 0x8fbcbb];
    gruvbox, "gruvbox", "Gruvbox (dark, warm retro)." =>
    [0x282828, 0xebdbb2, 0xa89984, 0x3c3836, 0x83a598, 0xd3869b, 0xb8bb26, 0xfabd2f, 0xfb4934, 0x8ec07c],
    [0x83a598, 0xfabd2f, 0xb8bb26, 0xd3869b, 0xfb4934, 0x8ec07c, 0xfe8019, 0xebdbb2];
    tokyo_night, "tokyo-night", "Tokyo Night (dark)." =>
    [0x1a1b26, 0xc0caf5, 0x737aa2, 0x292e42, 0x7aa2f7, 0xbb9af7, 0x9ece6a, 0xe0af68, 0xf7768e, 0x7dcfff],
    [0x7aa2f7, 0xe0af68, 0xbb9af7, 0x9ece6a, 0xf7768e, 0x7dcfff, 0xff9e64, 0x73daca];
    solarized_dark, "solarized-dark", "Solarized Dark." =>
    [0x002b36, 0x93a1a1, 0x839496, 0x073642, 0x268bd2, 0x6c71c4, 0x859900, 0xb58900, 0xdc322f, 0x2aa198],
    [0x268bd2, 0xb58900, 0x6c71c4, 0x859900, 0xdc322f, 0x2aa198, 0xcb4b16, 0xd33682];
    solarized_light, "solarized-light", "Solarized Light." =>
    [0xfdf6e3, 0x586e75, 0x657b83, 0xeee8d5, 0x268bd2, 0x6c71c4, 0x859900, 0xb58900, 0xdc322f, 0x2aa198],
    [0x268bd2, 0xb58900, 0x6c71c4, 0x859900, 0xdc322f, 0x2aa198, 0xcb4b16, 0xd33682];
    one_dark, "one-dark", "One Dark (Atom)." =>
    [0x282c34, 0xabb2bf, 0x7f848e, 0x3e4451, 0x61afef, 0xc678dd, 0x98c379, 0xe5c07b, 0xe06c75, 0x56b6c2],
    [0x61afef, 0xe5c07b, 0xc678dd, 0x98c379, 0xe06c75, 0x56b6c2, 0xd19a66, 0xabb2bf];
    rose_pine, "rose-pine", "Rosé Pine (dark)." =>
    [0x191724, 0xe0def4, 0x908caa, 0x1f1d2e, 0xc4a7e7, 0xebbcba, 0x9ccfd8, 0xf6c177, 0xeb6f92, 0x31748f],
    [0xc4a7e7, 0xf6c177, 0xebbcba, 0x9ccfd8, 0xeb6f92, 0x31748f, 0xea9a97, 0xe0def4];
    monokai, "monokai", "Monokai (dark)." =>
    [0x272822, 0xf8f8f2, 0x90908a, 0x3e3d32, 0x66d9ef, 0xae81ff, 0xa6e22e, 0xe6db74, 0xf92672, 0x66d9ef],
    [0x66d9ef, 0xa6e22e, 0xf92672, 0xe6db74, 0xae81ff, 0xfd971f, 0x75715e, 0xf8f8f2];
}

impl Theme {
    /// Porsche: black/white with the signature red accent (`#D5001C`).
    ///
    /// Bull is white and bear is red, as in the Porsche chart look.
    #[must_use]
    pub fn porsche() -> Self {
        let mut t = Self::from_spec(
            "porsche",
            Spec {
                bg: 0x000000,
                fg: 0xffffff,
                muted: 0x9b9ba0,
                surface: 0x1a1a1c,
                accent: 0xd5001c,
                secondary: 0xe3e3e6,
                success: 0x2e9e5b,
                warning: 0xf5a623,
                error: 0xff4d5e,
                info: 0x4aa3ff,
                palette: &[
                    0xd5001c, 0xffffff, 0x9b9ba0, 0xff5a6e, 0x6b6b70, 0xe3e3e6, 0x8c0014, 0xc4c4c8,
                ],
            },
        );
        t.bull = rgb(0xffffff);
        t.bear = rgb(0xd5001c);
        t
    }

    /// Colorblind-safe theme (Okabe–Ito palette; blue/orange instead of
    /// green/red). Keeps the terminal background.
    #[must_use]
    pub fn colorblind() -> Self {
        let mut t = Self::from_spec(
            "colorblind",
            Spec {
                bg: 0x000000,
                fg: 0xffffff,
                muted: 0xbbbbbb,
                surface: 0x222222,
                accent: 0x56b4e9,
                secondary: 0xcc79a7,
                success: 0x0072b2,
                warning: 0xf0e442,
                error: 0xe69f00,
                info: 0x56b4e9,
                palette: &[
                    0x0072b2, 0xe69f00, 0x009e73, 0xf0e442, 0x56b4e9, 0xd55e00, 0xcc79a7, 0xbbbbbb,
                ],
            },
        );
        t.background = None;
        t.surface = None;
        t.tooltip_bg = Color::DarkGray;
        t.neutral = rgb(0xbbbbbb);
        t
    }

    /// Maximum-contrast theme: pure black/white with saturated status colors.
    #[must_use]
    pub fn high_contrast() -> Self {
        let mut t = Self::from_spec(
            "high-contrast",
            Spec {
                bg: 0x000000,
                fg: 0xffffff,
                muted: 0xd0d0d0,
                surface: 0x1c1c1c,
                accent: 0xffff00,
                secondary: 0x00ffff,
                success: 0x00ff00,
                warning: 0xffa500,
                error: 0xff5050,
                info: 0x00ffff,
                palette: &[
                    0xffff00, 0x00ffff, 0xff00ff, 0x00ff00, 0xff5050, 0xffa500, 0x5f9fff, 0xffffff,
                ],
            },
        );
        t.border = rgb(0xffffff);
        t.selection_bg = rgb(0xffffff);
        t.selection_fg = rgb(0x000000);
        t.disabled = rgb(0xa0a0a0);
        t
    }

    /// Monochrome theme for 1-bit-ish terminals: only white, gray and dark
    /// gray, terminal background. Selection is shown in reverse video colors.
    #[must_use]
    pub fn mono() -> Self {
        Self {
            name: "mono".into(),
            background: None,
            surface: None,
            foreground: Color::White,
            muted: Color::Gray,
            disabled: Color::DarkGray,
            border: Color::Gray,
            border_focused: Color::White,
            title: Color::White,
            primary: Color::White,
            secondary: Color::Gray,
            accent: Color::White,
            success: Color::White,
            warning: Color::Gray,
            error: Color::White,
            info: Color::Gray,
            selection_bg: Color::White,
            selection_fg: Color::Black,
            highlight: Color::White,
            palette: vec![
                Color::White,
                Color::Gray,
                Color::DarkGray,
                Color::White,
                Color::Gray,
                Color::DarkGray,
                Color::White,
                Color::Gray,
            ],
            bull: Color::White,
            bear: Color::DarkGray,
            neutral: Color::Gray,
            axis: Color::DarkGray,
            grid: Color::DarkGray,
            crosshair: Color::Gray,
            tooltip_fg: Color::Black,
            tooltip_bg: Color::White,
        }
    }

    /// All built-in themes, in cycling order.
    #[must_use]
    pub fn all() -> Vec<Theme> {
        let mut v = vec![Theme::default()];
        v.extend(NAMED.iter().map(|(_, f)| f()));
        v.push(Theme::porsche());
        v.push(Theme::colorblind());
        v.push(Theme::high_contrast());
        v.push(Theme::mono());
        v
    }

    /// Names of all built-in themes, in cycling order.
    #[must_use]
    pub fn names() -> Vec<&'static str> {
        let mut v = vec!["default"];
        v.extend(NAMED.iter().map(|(n, _)| *n));
        v.extend(["porsche", "colorblind", "high-contrast", "mono"]);
        v
    }

    /// Look a built-in theme up by name. Case-insensitive; `_` and spaces
    /// are accepted for `-` (`"Tokyo Night"` → `tokyo-night`).
    #[must_use]
    pub fn by_name(name: &str) -> Option<Theme> {
        let key = name.trim().to_ascii_lowercase().replace(['_', ' '], "-");
        Self::all().into_iter().find(|t| t.name == key)
    }

    fn position(&self) -> Option<usize> {
        Self::names().iter().position(|n| *n == self.name)
    }

    /// The next built-in theme (wraps). Custom-named themes start at `default`.
    #[must_use]
    pub fn next(&self) -> Theme {
        let all = Self::all();
        let i = self.position().map_or(0, |i| (i + 1) % all.len());
        all[i].clone()
    }

    /// The previous built-in theme (wraps). Custom-named themes start at the last one.
    #[must_use]
    pub fn prev(&self) -> Theme {
        let all = Self::all();
        let i = self
            .position()
            .map_or(all.len() - 1, |i| (i + all.len() - 1) % all.len());
        all[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_all() {
        let all: Vec<String> = Theme::all().into_iter().map(|t| t.name).collect();
        assert_eq!(all, Theme::names());
    }

    #[test]
    fn sixteen_unique_themes() {
        let names = Theme::names();
        assert_eq!(names.len(), 16);
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 16);
    }

    #[test]
    fn by_name_accepts_variants() {
        assert_eq!(Theme::by_name("Tokyo Night").unwrap().name, "tokyo-night");
        assert_eq!(Theme::by_name("ROSE_PINE").unwrap().name, "rose-pine");
        assert!(Theme::by_name("nope").is_none());
        assert!(Theme::by_name("").is_none());
    }

    #[test]
    fn next_prev_inverse_and_wrap() {
        let all = Theme::all();
        for t in &all {
            assert_eq!(t.next().prev(), *t);
        }
        assert_eq!(all.last().unwrap().next(), all[0]);
        assert_eq!(all[0].prev(), *all.last().unwrap());
    }

    #[test]
    fn custom_theme_cycles_into_builtins() {
        let c = Theme::default().name("mine");
        assert_eq!(c.next().name, "default");
        assert_eq!(c.prev().name, "mono");
    }

    #[test]
    fn porsche_bull_is_white() {
        assert_eq!(Theme::porsche().bull, Color::Rgb(255, 255, 255));
        assert_eq!(Theme::porsche().accent, Color::Rgb(0xd5, 0x00, 0x1c));
    }

    #[test]
    fn colorblind_has_no_green_red_pair() {
        let t = Theme::colorblind();
        assert_eq!(t.background, None);
        assert_eq!(t.bull, Color::Rgb(0, 0x72, 0xb2));
    }
}
