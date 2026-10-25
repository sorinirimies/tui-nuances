//! Built-in named themes and lookup / cycling helpers.
//!
//! Cycling order of [`Theme::all`]: `default`, the popular editor palettes
//! (`catppuccin-mocha` … `monokai`, then the extra editor/palette themes ported from gitkraft, `porsche`), then the accessibility themes
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
    default_light, "default-light", "Default Light (light, from gitkraft)." =>
    [0xfafaff, 0x1e2332, 0x828ca0, 0xebeef8, 0x008cdc, 0xff641e, 0x1ea050, 0xc88c00, 0xd23232, 0xff641e],
    [0x008cdc, 0x1ea050, 0xd23232, 0xc88c00, 0xff641e, 0x8c50c8, 0x009696, 0xbe3c78];
    grape, "grape", "Grape (dark, from gitkraft)." =>
    [0x120c1e, 0xe6dcff, 0x6e6482, 0x1e1432, 0xc878ff, 0x82b4ff, 0xa06eff, 0xd2aaff, 0xff5096, 0x82b4ff],
    [0xc878ff, 0xa06eff, 0xff5096, 0xd2aaff, 0x82b4ff, 0x50dcc8, 0xffb464, 0x78ffa0];
    ocean, "ocean", "Ocean (dark, from gitkraft)." =>
    [0x001423, 0xc8f0f5, 0x507882, 0x002337, 0x00c8b4, 0x00afd2, 0x50e6c8, 0xffdc50, 0xff6464, 0x00afd2],
    [0x00c8b4, 0x50e6c8, 0xff6464, 0xffdc50, 0x00afd2, 0xb482ff, 0xffa064, 0xdc78c8];
    sunset, "sunset", "Sunset (dark, from gitkraft)." =>
    [0x160806, 0xffebd2, 0x8c6450, 0x280f0a, 0xff5050, 0xff9632, 0xffb450, 0xffe650, 0xff3232, 0xff9632],
    [0xff5050, 0xffb450, 0xff3232, 0xffe650, 0xff9632, 0x64b4ff, 0xb478ff, 0x50dcc8];
    forest, "forest", "Forest (dark, from gitkraft)." =>
    [0x081208, 0xd2ebc8, 0x5a7850, 0x0f1e0f, 0x64c850, 0x50a050, 0x78d25a, 0xdcc850, 0xd25050, 0x50a050],
    [0x64c850, 0x78d25a, 0xd25050, 0xdcc850, 0x50a050, 0x82a0ff, 0xc882b4, 0xc8b464];
    rose, "rose", "Rose (dark, from gitkraft)." =>
    [0x1c0610, 0xffe6eb, 0x8c5a6e, 0x320c1e, 0xff6496, 0xff8cb4, 0xffa0be, 0xffdcb4, 0xdc3c64, 0xff8cb4],
    [0xff6496, 0xffa0be, 0xdc3c64, 0xffdcb4, 0xff8cb4, 0x64b4ff, 0x78dcc8, 0xc8b464];
    neon, "neon", "Neon (dark, from gitkraft)." =>
    [0x06000e, 0xe6e6ff, 0x645078, 0x0f001e, 0xff00c8, 0x00ffc8, 0x00ff82, 0xffdc00, 0xff1e50, 0x00ffc8],
    [0xff00c8, 0x00ff82, 0xff1e50, 0xffdc00, 0x00ffc8, 0x6450ff, 0xff7800, 0x00b4ff];
    gruvbox_light, "gruvbox-light", "Gruvbox Light (light, from gitkraft)." =>
    [0xfbf1c7, 0x3c3836, 0x928374, 0xd5c4a1, 0xd65d0e, 0xd79921, 0x79740e, 0xd79921, 0xd65d0e, 0xd79921],
    [0xd65d0e, 0x79740e, 0xcc241d, 0xd79921, 0x458588, 0xb16286, 0x689d6a, 0x076678];
    catppuccin_frappe, "catppuccin-frappe", "Catppuccin Frappé (dark, from gitkraft)." =>
    [0x303446, 0xc6d0f5, 0x737994, 0x414559, 0xca9ee6, 0x8caaee, 0xa6d189, 0xe5c890, 0xe78284, 0x8caaee],
    [0xca9ee6, 0xa6d189, 0xe78284, 0xe5c890, 0x8caaee, 0x81c8be, 0xf4b8e4, 0xef9f76];
    catppuccin_macchiato, "catppuccin-macchiato", "Catppuccin Macchiato (dark, from gitkraft)." =>
    [0x24273a, 0xcad3f5, 0x6e738d, 0x363a4f, 0xc6a0f6, 0x8aadf4, 0xa6da95, 0xeed49f, 0xed8796, 0x8aadf4],
    [0xc6a0f6, 0xa6da95, 0xed8796, 0xeed49f, 0x8aadf4, 0x8bd5ca, 0xf5bde6, 0xf0a880];
    tokyo_night_storm, "tokyo-night-storm", "Tokyo Night Storm (dark, from gitkraft)." =>
    [0x24283b, 0xc0caf5, 0x565f89, 0x2d314b, 0xbb9af7, 0x7aa2f7, 0x9ece6a, 0xe0af68, 0xf7768e, 0x7aa2f7],
    [0xbb9af7, 0x9ece6a, 0xf7768e, 0xe0af68, 0x7aa2f7, 0x7dcfff, 0xff9e64, 0x73daca];
    tokyo_night_light, "tokyo-night-light", "Tokyo Night Light (light, from gitkraft)." =>
    [0xd5d6db, 0x343b58, 0x848cb0, 0xd0d5e3, 0x5a4a78, 0x2e7ee9, 0x485e30, 0x8c6c3e, 0xd20f39, 0x2e7ee9],
    [0x5a4a78, 0x485e30, 0xd20f39, 0x8c6c3e, 0x2e7ee9, 0x388c96, 0xa6528c, 0xb46432];
    kanagawa_wave, "kanagawa-wave", "Kanagawa Wave (dark, from gitkraft)." =>
    [0x16161e, 0xdcd7ba, 0x727169, 0x2a2a37, 0xd27e99, 0x7e9cd8, 0x76946a, 0xdca561, 0xd27e99, 0x7e9cd8],
    [0xd27e99, 0x76946a, 0xff5a64, 0xdca561, 0x7e9cd8, 0x6a9589, 0xe46876, 0x957fb8];
    kanagawa_dragon, "kanagawa-dragon", "Kanagawa Dragon (dark, from gitkraft)." =>
    [0x141414, 0xc5c9c5, 0xa6a69c, 0x282727, 0xd27e99, 0x8ba4b0, 0x87a987, 0xc8aa6d, 0xd27e99, 0x8ba4b0],
    [0xd27e99, 0x87a987, 0xe36464, 0xc8aa6d, 0x8ba4b0, 0x6a9589, 0xc46c7c, 0xa591c4];
    kanagawa_lotus, "kanagawa-lotus", "Kanagawa Lotus (light, from gitkraft)." =>
    [0xf6f3e4, 0x545464, 0xc4b28a, 0xe7dba0, 0xa09abe, 0x4d699b, 0x6f894e, 0x77713f, 0xc04747, 0x4d699b],
    [0xa09abe, 0x6f894e, 0xc04747, 0x77713f, 0x4d699b, 0x648c82, 0xb46478, 0x8c78aa];
    moonfly, "moonfly", "Moonfly (dark, from gitkraft)." =>
    [0x080808, 0xb2b2b2, 0x4e4e4e, 0x1c1c1c, 0xae81ff, 0x80a0ff, 0x8cc85f, 0xe2a478, 0xff7383, 0x80a0ff],
    [0xae81ff, 0x8cc85f, 0xff7383, 0xe2a478, 0x80a0ff, 0x74b4bb, 0xffc078, 0xc88cff];
    nightfly, "nightfly", "Nightfly (dark, from gitkraft)." =>
    [0x011626, 0xacbbcb, 0x4b6479, 0x0b2942, 0xc792ea, 0x82aaff, 0xa1cd5e, 0xf3da0b, 0xfc5749, 0x82aaff],
    [0xc792ea, 0xa1cd5e, 0xfc5749, 0xf3da0b, 0x82aaff, 0x21c8d7, 0xee8262, 0xaec8ff];
    oxocarbon, "oxocarbon", "Oxocarbon (dark, from gitkraft)." =>
    [0x161616, 0xf2f4f8, 0x525252, 0x262626, 0xff7eb6, 0x78a9ff, 0x42be65, 0xfacc37, 0xff6165, 0x78a9ff],
    [0xff7eb6, 0x42be65, 0xff6165, 0xfacc37, 0x78a9ff, 0x08bdba, 0xbe95ff, 0xffaa78];
    cyberpunk, "cyberpunk", "Cyberpunk (dark, from gitkraft)." =>
    [0x0a0a10, 0xe6e6dc, 0x5a5a64, 0x14141e, 0xfcee0a, 0x00d2eb, 0x00dcb4, 0xff9600, 0xff3246, 0x00d2eb],
    [0xfcee0a, 0x00dcb4, 0xff3246, 0xff9600, 0x00d2eb, 0xb400ff, 0xff0096, 0x00ff64];
    rose_pine_moon, "rose-pine-moon", "Rose Pine Moon (dark, from gitkraft)." =>
    [0x232136, 0xe0def4, 0x6e6a86, 0x302e46, 0xc4a7e7, 0x9ccfd8, 0x3e8fb0, 0xf6c177, 0xeb6f92, 0x9ccfd8],
    [0xc4a7e7, 0x3e8fb0, 0xeb6f92, 0xf6c177, 0x9ccfd8, 0xea9a97, 0x9ccfd8, 0xe0def4];
    rose_pine_dawn, "rose-pine-dawn", "Rose Pine Dawn (light, from gitkraft)." =>
    [0xfaf4ed, 0x575279, 0x9893a5, 0xf2e9dd, 0x907aa9, 0x56949f, 0x286983, 0xea9d34, 0xb4637a, 0x56949f],
    [0x907aa9, 0x286983, 0xb4637a, 0xea9d34, 0x56949f, 0xd7827e, 0x575279, 0x9893a5];
    ayu_mirage, "ayu-mirage", "Ayu Mirage (dark, from gitkraft)." =>
    [0x1f2430, 0xcccac2, 0x686868, 0x2a303e, 0x73d0ff, 0x73d0ff, 0x87d96c, 0xfacc6e, 0xed8274, 0x73d0ff],
    [0x73d0ff, 0x87d96c, 0xed8274, 0xfacc6e, 0x409fff, 0xd9bfff, 0xffaa6c, 0xcccac2];
    everforest_dark, "everforest-dark", "Everforest Dark (dark, from gitkraft)." =>
    [0x1e2326, 0xd3c6aa, 0x7a8478, 0x2a3032, 0xa7c080, 0x7fbbb3, 0xa7c080, 0xdbbc7f, 0xe67e80, 0x7fbbb3],
    [0xa7c080, 0x7fbbb3, 0xe67e80, 0xdbbc7f, 0xd699b6, 0xd3c6aa, 0xa6b0a0, 0x7a8478];
    atom_one_light, "atom-one-light", "Atom One Light (light, from gitkraft)." =>
    [0xf9f9f9, 0x2a2c33, 0x767676, 0xededed, 0x2f5af3, 0x2f5af3, 0x3f953a, 0xd2b67c, 0xde3e35, 0x2f5af3],
    [0x2f5af3, 0x3f953a, 0xde3e35, 0xd2b67c, 0xa626a4, 0x0184bc, 0x2a2c33, 0xb4b4b4];
    night_owl, "night-owl", "Night Owl (dark, from gitkraft)." =>
    [0x011627, 0xd6deeb, 0x575656, 0x0c2337, 0x82aaff, 0x82aaff, 0x22da6e, 0xaddb67, 0xef5350, 0x82aaff],
    [0x82aaff, 0x22da6e, 0xef5350, 0xaddb67, 0xc792ea, 0x7fdbca, 0xd6deeb, 0xffcb8b];
    poimandres, "poimandres", "Poimandres (dark, from gitkraft)." =>
    [0x1a1e28, 0xa6accd, 0x646a82, 0x262a37, 0x5de4c7, 0x89ddff, 0x5de4c7, 0xfffac2, 0xd0679d, 0x89ddff],
    [0x5de4c7, 0x89ddff, 0xd0679d, 0xfffac2, 0xa6accd, 0xaddb67, 0x646a82, 0xffb478];
    flexoki_dark, "flexoki-dark", "Flexoki Dark (dark, from gitkraft)." =>
    [0x100f0f, 0xcecdc3, 0x575653, 0x1e1c1c, 0x4385be, 0x4385be, 0x879a39, 0xd0a215, 0xd14d41, 0x4385be],
    [0x4385be, 0x879a39, 0xd14d41, 0xd0a215, 0xce5d97, 0x3aa99f, 0xcecdc3, 0xab8d48];
    flexoki_light, "flexoki-light", "Flexoki Light (light, from gitkraft)." =>
    [0xfffcf0, 0x100f0f, 0x6f6e69, 0xf2eede, 0x205ea6, 0x205ea6, 0x66800b, 0xad8301, 0xaf3029, 0x205ea6],
    [0x205ea6, 0x66800b, 0xaf3029, 0xad8301, 0xa34077, 0x24837b, 0x100f0f, 0x85682e];
    carbonfox, "carbonfox", "Carbonfox (dark, from gitkraft)." =>
    [0x161616, 0xf2f4f8, 0x64646e, 0x232323, 0x78a9ff, 0x78a9ff, 0x25be6a, 0x08bdba, 0xee5396, 0x78a9ff],
    [0x78a9ff, 0x25be6a, 0xee5396, 0x08bdba, 0xbe95ff, 0xff7eb6, 0xf2f4f8, 0xffaa78];
    andromeda, "andromeda", "Andromeda (dark, from gitkraft)." =>
    [0x262a33, 0xe5e5e5, 0x666666, 0x323742, 0x05bc79, 0x0fa8cd, 0x05bc79, 0xe5e512, 0xcd3131, 0x0fa8cd],
    [0x05bc79, 0x0fa8cd, 0xcd3131, 0xe5e512, 0xbe95ff, 0xff7eb6, 0xe5e5e5, 0x666666];
    synthwave, "synthwave", "Synthwave (dark, from gitkraft)." =>
    [0x0a0810, 0xdad9c7, 0x7f7094, 0x14101e, 0xf6188f, 0x12c3e2, 0x1ebb2b, 0xfdf834, 0xf6188f, 0x12c3e2],
    [0xf6188f, 0x1ebb2b, 0x12c3e2, 0xfdf834, 0xbe95ff, 0xff7eb6, 0xdad9c7, 0x7f7094];
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
        let key: String = name
            .trim()
            .chars()
            .map(|c| match c {
                'é' | 'è' | 'É' => 'e',
                c => c.to_ascii_lowercase(),
            })
            .collect::<String>()
            .replace(['_', ' '], "-");
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
    fn all_names_unique() {
        let names = Theme::names();
        assert_eq!(names.len(), 47);
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len());
    }

    #[test]
    fn by_name_accepts_variants() {
        assert_eq!(Theme::by_name("Tokyo Night").unwrap().name, "tokyo-night");
        assert_eq!(Theme::by_name("ROSE_PINE").unwrap().name, "rose-pine");
        assert_eq!(
            Theme::by_name("Catppuccin Frappé").unwrap().name,
            "catppuccin-frappe"
        );
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
