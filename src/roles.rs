//! Per-theme UI role colors for the built-in named themes.
//!
//! The compact hex specs in `builtin.rs` drive the chart / series colors; this
//! table fine-tunes the *UI chrome* roles (`title`, `primary`, `border_focused`,
//! `selection_bg`, `muted`, `foreground`, `success`) so apps such as
//! `tui-file-explorer` can render with the shared themes without losing their
//! look. Column order: `[title, primary, border_focused, selection_bg, muted,
//! foreground, success]`.

use crate::color::{contrast_ratio, readable_on};
use crate::theme::Theme;
use ratatui::style::Color;

pub(crate) const ROLES: &[(&str, [Color; 7])] = &[
    (
        "andromeda",
        [
            Color::Rgb(5, 188, 121),
            Color::Rgb(229, 229, 18),
            Color::Rgb(15, 168, 205),
            Color::Rgb(90, 92, 98),
            Color::Rgb(102, 102, 102),
            Color::Rgb(229, 229, 229),
            Color::Rgb(5, 188, 121),
        ],
    ),
    (
        "atom-one-light",
        [
            Color::Rgb(47, 90, 243),
            Color::Rgb(210, 182, 124),
            Color::Rgb(47, 90, 243),
            Color::Rgb(237, 237, 237),
            Color::Rgb(118, 118, 118),
            Color::Rgb(42, 44, 51),
            Color::Rgb(63, 149, 58),
        ],
    ),
    (
        "ayu-mirage",
        [
            Color::Rgb(115, 208, 255),
            Color::Rgb(250, 204, 110),
            Color::Rgb(115, 208, 255),
            Color::Rgb(64, 159, 255),
            Color::Rgb(104, 104, 104),
            Color::Rgb(204, 202, 194),
            Color::Rgb(135, 217, 108),
        ],
    ),
    (
        "carbonfox",
        [
            Color::Rgb(120, 169, 255),
            Color::Rgb(8, 189, 186),
            Color::Rgb(120, 169, 255),
            Color::Rgb(42, 42, 42),
            Color::Rgb(100, 100, 110),
            Color::Rgb(242, 244, 248),
            Color::Rgb(37, 190, 106),
        ],
    ),
    (
        "catppuccin-frappe",
        [
            Color::Rgb(202, 158, 230),
            Color::Rgb(229, 200, 144),
            Color::Rgb(140, 170, 238),
            Color::Rgb(65, 69, 89),
            Color::Rgb(115, 121, 148),
            Color::Rgb(198, 208, 245),
            Color::Rgb(166, 209, 137),
        ],
    ),
    (
        "catppuccin-latte",
        [
            Color::Rgb(136, 57, 239),
            Color::Rgb(254, 100, 11),
            Color::Rgb(30, 102, 245),
            Color::Rgb(204, 208, 218),
            Color::Rgb(156, 160, 176),
            Color::Rgb(76, 79, 105),
            Color::Rgb(64, 160, 43),
        ],
    ),
    (
        "catppuccin-macchiato",
        [
            Color::Rgb(198, 160, 246),
            Color::Rgb(238, 212, 159),
            Color::Rgb(138, 173, 244),
            Color::Rgb(54, 58, 79),
            Color::Rgb(110, 115, 141),
            Color::Rgb(202, 211, 245),
            Color::Rgb(166, 218, 149),
        ],
    ),
    (
        "catppuccin-mocha",
        [
            Color::Rgb(203, 166, 247),
            Color::Rgb(249, 226, 175),
            Color::Rgb(137, 180, 250),
            Color::Rgb(49, 50, 68),
            Color::Rgb(108, 112, 134),
            Color::Rgb(205, 214, 244),
            Color::Rgb(166, 227, 161),
        ],
    ),
    (
        "cyberpunk",
        [
            Color::Rgb(252, 238, 10),
            Color::Rgb(255, 150, 0),
            Color::Rgb(0, 210, 235),
            Color::Rgb(50, 48, 20),
            Color::Rgb(90, 90, 100),
            Color::Rgb(230, 230, 220),
            Color::Rgb(0, 220, 180),
        ],
    ),
    (
        "default-light",
        [
            Color::Rgb(0, 140, 220),
            Color::Rgb(200, 140, 0),
            Color::Rgb(255, 100, 30),
            Color::Rgb(200, 220, 255),
            Color::Rgb(130, 140, 160),
            Color::Rgb(30, 35, 50),
            Color::Rgb(30, 160, 80),
        ],
    ),
    (
        "dracula",
        [
            Color::Rgb(255, 121, 198),
            Color::Rgb(241, 250, 140),
            Color::Rgb(139, 233, 253),
            Color::Rgb(68, 71, 90),
            Color::Rgb(98, 114, 164),
            Color::Rgb(248, 248, 242),
            Color::Rgb(80, 250, 123),
        ],
    ),
    (
        "everforest-dark",
        [
            Color::Rgb(167, 192, 128),
            Color::Rgb(219, 188, 127),
            Color::Rgb(127, 187, 179),
            Color::Rgb(76, 55, 67),
            Color::Rgb(122, 132, 120),
            Color::Rgb(211, 198, 170),
            Color::Rgb(167, 192, 128),
        ],
    ),
    (
        "flexoki-dark",
        [
            Color::Rgb(67, 133, 190),
            Color::Rgb(208, 162, 21),
            Color::Rgb(67, 133, 190),
            Color::Rgb(64, 62, 60),
            Color::Rgb(87, 86, 83),
            Color::Rgb(206, 205, 195),
            Color::Rgb(135, 154, 57),
        ],
    ),
    (
        "flexoki-light",
        [
            Color::Rgb(32, 94, 166),
            Color::Rgb(173, 131, 1),
            Color::Rgb(32, 94, 166),
            Color::Rgb(206, 205, 195),
            Color::Rgb(111, 110, 105),
            Color::Rgb(16, 15, 15),
            Color::Rgb(102, 128, 11),
        ],
    ),
    (
        "forest",
        [
            Color::Rgb(100, 200, 80),
            Color::Rgb(170, 220, 100),
            Color::Rgb(80, 160, 80),
            Color::Rgb(20, 50, 20),
            Color::Rgb(90, 120, 80),
            Color::Rgb(210, 235, 200),
            Color::Rgb(120, 210, 90),
        ],
    ),
    (
        "grape",
        [
            Color::Rgb(200, 120, 255),
            Color::Rgb(200, 160, 255),
            Color::Rgb(130, 180, 255),
            Color::Rgb(50, 35, 80),
            Color::Rgb(110, 100, 130),
            Color::White,
            Color::Rgb(160, 110, 255),
        ],
    ),
    (
        "gruvbox-light",
        [
            Color::Rgb(214, 93, 14),
            Color::Rgb(181, 118, 20),
            Color::Rgb(215, 153, 33),
            Color::Rgb(213, 196, 161),
            Color::Rgb(146, 131, 116),
            Color::Rgb(60, 56, 54),
            Color::Rgb(121, 116, 14),
        ],
    ),
    (
        "gruvbox",
        [
            Color::Rgb(254, 128, 25),
            Color::Rgb(250, 189, 47),
            Color::Rgb(250, 189, 47),
            Color::Rgb(60, 56, 54),
            Color::Rgb(146, 131, 116),
            Color::Rgb(235, 219, 178),
            Color::Rgb(184, 187, 38),
        ],
    ),
    (
        "kanagawa-dragon",
        [
            Color::Rgb(210, 126, 153),
            Color::Rgb(200, 170, 109),
            Color::Rgb(139, 164, 176),
            Color::Rgb(40, 39, 39),
            Color::Rgb(166, 166, 156),
            Color::Rgb(197, 201, 197),
            Color::Rgb(135, 169, 135),
        ],
    ),
    (
        "kanagawa-lotus",
        [
            Color::Rgb(160, 154, 190),
            Color::Rgb(119, 113, 63),
            Color::Rgb(77, 105, 155),
            Color::Rgb(231, 219, 160),
            Color::Rgb(196, 178, 138),
            Color::Rgb(84, 84, 100),
            Color::Rgb(111, 137, 78),
        ],
    ),
    (
        "kanagawa-wave",
        [
            Color::Rgb(210, 126, 153),
            Color::Rgb(220, 165, 97),
            Color::Rgb(126, 156, 216),
            Color::Rgb(42, 42, 55),
            Color::Rgb(114, 113, 105),
            Color::Rgb(220, 215, 186),
            Color::Rgb(118, 148, 106),
        ],
    ),
    (
        "moonfly",
        [
            Color::Rgb(174, 129, 255),
            Color::Rgb(227, 199, 138),
            Color::Rgb(128, 160, 255),
            Color::Rgb(28, 28, 28),
            Color::Rgb(78, 78, 78),
            Color::Rgb(178, 178, 178),
            Color::Rgb(140, 200, 95),
        ],
    ),
    (
        "neon",
        [
            Color::Rgb(255, 0, 200),
            Color::Rgb(255, 220, 0),
            Color::Rgb(0, 255, 200),
            Color::Rgb(30, 0, 50),
            Color::Rgb(100, 80, 120),
            Color::Rgb(230, 230, 255),
            Color::Rgb(0, 255, 130),
        ],
    ),
    (
        "night-owl",
        [
            Color::Rgb(130, 170, 255),
            Color::Rgb(173, 219, 103),
            Color::Rgb(130, 170, 255),
            Color::Rgb(95, 126, 151),
            Color::Rgb(87, 86, 86),
            Color::Rgb(214, 222, 235),
            Color::Rgb(34, 218, 110),
        ],
    ),
    (
        "nightfly",
        [
            Color::Rgb(199, 146, 234),
            Color::Rgb(255, 202, 40),
            Color::Rgb(130, 170, 255),
            Color::Rgb(11, 41, 66),
            Color::Rgb(75, 100, 121),
            Color::Rgb(172, 187, 203),
            Color::Rgb(161, 205, 94),
        ],
    ),
    (
        "nord",
        [
            Color::Rgb(136, 192, 208),
            Color::Rgb(235, 203, 139),
            Color::Rgb(129, 161, 193),
            Color::Rgb(59, 66, 82),
            Color::Rgb(76, 86, 106),
            Color::Rgb(216, 222, 233),
            Color::Rgb(163, 190, 140),
        ],
    ),
    (
        "ocean",
        [
            Color::Rgb(0, 200, 180),
            Color::Rgb(100, 220, 210),
            Color::Rgb(0, 175, 210),
            Color::Rgb(0, 50, 70),
            Color::Rgb(80, 120, 130),
            Color::Rgb(200, 240, 245),
            Color::Rgb(80, 230, 200),
        ],
    ),
    (
        "one-dark",
        [
            Color::Rgb(97, 175, 239),
            Color::Rgb(229, 192, 123),
            Color::Rgb(97, 175, 239),
            Color::Rgb(50, 56, 68),
            Color::Rgb(118, 118, 118),
            Color::Rgb(171, 178, 191),
            Color::Rgb(152, 195, 121),
        ],
    ),
    (
        "oxocarbon",
        [
            Color::Rgb(255, 126, 182),
            Color::Rgb(255, 213, 0),
            Color::Rgb(120, 169, 255),
            Color::Rgb(38, 38, 38),
            Color::Rgb(82, 82, 82),
            Color::Rgb(242, 244, 248),
            Color::Rgb(66, 190, 101),
        ],
    ),
    (
        "poimandres",
        [
            Color::Rgb(93, 228, 199),
            Color::Rgb(255, 250, 194),
            Color::Rgb(137, 221, 255),
            Color::Rgb(50, 55, 75),
            Color::Rgb(100, 106, 130),
            Color::Rgb(166, 172, 205),
            Color::Rgb(93, 228, 199),
        ],
    ),
    (
        "rose-pine-dawn",
        [
            Color::Rgb(144, 122, 169),
            Color::Rgb(234, 157, 52),
            Color::Rgb(86, 148, 159),
            Color::Rgb(223, 218, 217),
            Color::Rgb(152, 147, 165),
            Color::Rgb(87, 82, 121),
            Color::Rgb(40, 105, 131),
        ],
    ),
    (
        "rose-pine-moon",
        [
            Color::Rgb(196, 167, 231),
            Color::Rgb(246, 193, 119),
            Color::Rgb(156, 207, 216),
            Color::Rgb(68, 65, 90),
            Color::Rgb(110, 106, 134),
            Color::Rgb(224, 222, 244),
            Color::Rgb(62, 143, 176),
        ],
    ),
    (
        "rose-pine",
        [
            Color::Rgb(196, 167, 231),
            Color::Rgb(246, 193, 119),
            Color::Rgb(156, 207, 216),
            Color::Rgb(64, 61, 82),
            Color::Rgb(110, 106, 134),
            Color::Rgb(224, 222, 244),
            Color::Rgb(49, 116, 143),
        ],
    ),
    (
        "rose",
        [
            Color::Rgb(255, 100, 150),
            Color::Rgb(255, 180, 200),
            Color::Rgb(255, 140, 180),
            Color::Rgb(80, 20, 40),
            Color::Rgb(140, 90, 110),
            Color::Rgb(255, 230, 235),
            Color::Rgb(255, 160, 190),
        ],
    ),
    (
        "solarized-dark",
        [
            Color::Rgb(38, 139, 210),
            Color::Rgb(181, 137, 0),
            Color::Rgb(42, 161, 152),
            Color::Rgb(7, 54, 66),
            Color::Rgb(88, 110, 117),
            Color::Rgb(131, 148, 150),
            Color::Rgb(133, 153, 0),
        ],
    ),
    (
        "solarized-light",
        [
            Color::Rgb(38, 139, 210),
            Color::Rgb(181, 137, 0),
            Color::Rgb(42, 161, 152),
            Color::Rgb(238, 232, 213),
            Color::Rgb(147, 161, 161),
            Color::Rgb(101, 123, 131),
            Color::Rgb(133, 153, 0),
        ],
    ),
    (
        "sunset",
        [
            Color::Rgb(255, 80, 80),
            Color::Rgb(255, 200, 60),
            Color::Rgb(255, 150, 50),
            Color::Rgb(80, 30, 20),
            Color::Rgb(140, 100, 80),
            Color::Rgb(255, 235, 210),
            Color::Rgb(255, 180, 80),
        ],
    ),
    (
        "synthwave",
        [
            Color::Rgb(246, 24, 143),
            Color::Rgb(253, 248, 52),
            Color::Rgb(18, 195, 226),
            Color::Rgb(25, 50, 60),
            Color::Rgb(127, 112, 148),
            Color::Rgb(218, 217, 199),
            Color::Rgb(30, 187, 43),
        ],
    ),
    (
        "tokyo-night-light",
        [
            Color::Rgb(90, 74, 120),
            Color::Rgb(140, 108, 62),
            Color::Rgb(46, 126, 233),
            Color::Rgb(208, 213, 227),
            Color::Rgb(132, 140, 176),
            Color::Rgb(52, 59, 88),
            Color::Rgb(72, 94, 48),
        ],
    ),
    (
        "tokyo-night-storm",
        [
            Color::Rgb(187, 154, 247),
            Color::Rgb(224, 175, 104),
            Color::Rgb(122, 162, 247),
            Color::Rgb(45, 49, 75),
            Color::Rgb(86, 95, 137),
            Color::Rgb(192, 202, 245),
            Color::Rgb(158, 206, 106),
        ],
    ),
    (
        "tokyo-night",
        [
            Color::Rgb(187, 154, 247),
            Color::Rgb(224, 175, 104),
            Color::Rgb(122, 162, 247),
            Color::Rgb(41, 46, 66),
            Color::Rgb(86, 95, 137),
            Color::Rgb(192, 202, 245),
            Color::Rgb(158, 206, 106),
        ],
    ),
];

/// Apply the role table entry for `t.name`, if any.
pub(crate) fn apply(t: &mut Theme) {
    if let Some((_, r)) = ROLES.iter().find(|(n, _)| *n == t.name) {
        t.title = r[0];
        t.primary = r[1];
        t.border_focused = r[2];
        t.selection_bg = r[3];
        t.selection_fg = readable_on(r[3]);
        t.muted = r[4];
        // Keep the base foreground when the override would fail WCAG AA.
        let bg = t.background.unwrap_or(Color::Rgb(0, 0, 0));
        if contrast_ratio(r[5], bg) >= 4.5 {
            t.foreground = r[5];
        }
        t.success = r[6];
        t.bull = r[6];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_role_entry_is_applied_to_its_theme() {
        for (name, r) in ROLES {
            let t = Theme::by_name(name).unwrap_or_else(|| panic!("unknown theme {name}"));
            assert_eq!(t.title, r[0], "{name} title");
            assert_eq!(t.primary, r[1], "{name} primary");
            assert_eq!(t.border_focused, r[2], "{name} border_focused");
            assert_eq!(t.selection_bg, r[3], "{name} selection_bg");
            assert_eq!(t.muted, r[4], "{name} muted");
            assert_eq!(t.success, r[6], "{name} success");
        }
    }

    #[test]
    fn role_names_are_unique() {
        let mut n: Vec<_> = ROLES.iter().map(|(n, _)| *n).collect();
        n.sort_unstable();
        n.dedup();
        assert_eq!(n.len(), ROLES.len());
    }
}
