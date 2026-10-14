//! Integration tests over every built-in theme.

use ratatui::style::Color;
use tui_nuances::color::{contrast_ratio, readable_on, to_rgb};
use tui_nuances::{ColorSupport, Status, Theme, ThemeSwitcher};

/// Themes exempt from the WCAG check: they use terminal-native named colors,
/// whose real RGB depends on the user's terminal palette.
const CONTRAST_EXEMPT: [&str; 2] = ["default", "mono"];

#[test]
fn every_theme_has_at_least_eight_palette_colors() {
    for t in Theme::all() {
        assert!(t.palette.len() >= 8, "{}", t.name);
    }
}

#[test]
fn palette_colors_are_distinct_enough() {
    for t in Theme::all().iter().filter(|t| t.name != "mono") {
        let mut p = t.palette.clone();
        p.dedup();
        assert!(p.len() >= 6, "{} palette too repetitive", t.name);
    }
}

#[test]
fn foreground_on_background_meets_wcag_aa() {
    for t in Theme::all() {
        if CONTRAST_EXEMPT.contains(&t.name.as_str()) {
            continue;
        }
        let bg = t.background.unwrap_or(Color::Rgb(0, 0, 0));
        let ratio = contrast_ratio(t.foreground, bg);
        assert!(ratio >= 4.5, "{}: {ratio:.2}", t.name);
    }
}

#[test]
fn selection_text_is_readable() {
    for t in Theme::all() {
        if CONTRAST_EXEMPT.contains(&t.name.as_str()) {
            continue;
        }
        let r = contrast_ratio(t.selection_fg, t.selection_bg);
        assert!(r >= 4.5, "{}: {r:.2}", t.name);
    }
}

#[test]
fn rgb_themes_resolve_every_color() {
    for t in Theme::all() {
        for c in t.colors() {
            assert!(to_rgb(c).is_some() || c == Color::Reset, "{}", t.name);
        }
    }
}

#[test]
fn by_name_roundtrip() {
    for t in Theme::all() {
        assert_eq!(Theme::by_name(&t.name), Some(t.clone()));
        assert_eq!(Theme::by_name(&t.name.to_uppercase()), Some(t));
    }
    assert!(Theme::by_name("does-not-exist").is_none());
}

#[test]
fn names_are_kebab_case_and_unique() {
    let names = Theme::names();
    for n in &names {
        assert!(n.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "{n}");
    }
    let mut s = names.clone();
    s.sort_unstable();
    s.dedup();
    assert_eq!(s.len(), names.len());
}

#[test]
fn next_visits_every_theme_once_then_wraps() {
    let all = Theme::all();
    let mut t = all[0].clone();
    let mut seen = vec![t.name.clone()];
    for _ in 1..all.len() {
        t = t.next();
        seen.push(t.name.clone());
    }
    assert_eq!(seen, Theme::names());
    assert_eq!(t.next(), all[0]);
}

#[test]
fn prev_visits_every_theme_in_reverse() {
    let all = Theme::all();
    let mut t = all[0].clone();
    for expected in all.iter().rev().take(all.len() - 1) {
        t = t.prev();
        assert_eq!(&t, expected);
    }
    assert_eq!(t.prev(), all[0]);
}

#[test]
fn switcher_matches_theme_cycle() {
    let mut s = ThemeSwitcher::new();
    let mut t = Theme::default();
    for _ in 0..40 {
        assert_eq!(s.current(), &t);
        s.next();
        t = t.next();
    }
}

#[test]
fn light_themes_have_light_backgrounds() {
    for n in ["catppuccin-latte", "solarized-light"] {
        let t = Theme::by_name(n).unwrap();
        assert!(tui_nuances::color::is_light(t.background.unwrap()), "{n}");
    }
    for n in ["dracula", "nord", "monokai"] {
        let t = Theme::by_name(n).unwrap();
        assert!(!tui_nuances::color::is_light(t.background.unwrap()), "{n}");
    }
}

#[test]
fn from_base_derivations_across_inputs() {
    let bgs = [
        None,
        Some(Color::Rgb(10, 10, 10)),
        Some(Color::Rgb(250, 250, 250)),
    ];
    for bg in bgs {
        let fg = readable_on(bg.unwrap_or(Color::Rgb(0, 0, 0)));
        let t = Theme::from_base(
            "x",
            bg,
            fg,
            Color::Rgb(200, 30, 60),
            Color::Green,
            Color::Red,
        );
        assert_eq!(t.background, bg);
        assert_eq!(t.foreground, fg);
        assert_eq!(t.border_focused, t.accent);
        assert_eq!(t.status_color(Status::Success), Color::Green);
        assert_eq!(t.status_color(Status::Error), Color::Red);
        assert!(t.palette.len() >= 8);
        let base = bg.unwrap_or(Color::Rgb(0, 0, 0));
        // muted is between bg and fg: less contrast than fg, more than bg alone.
        let c = contrast_ratio(t.muted, base);
        assert!(c > 1.0 && c < contrast_ratio(fg, base), "{c}");
    }
}

#[test]
fn downgrade_ansi256_only_indexed_or_basic() {
    for t in Theme::all() {
        for c in t.downgrade(ColorSupport::Ansi256).colors() {
            assert!(!matches!(c, Color::Rgb(..)), "{}: {c:?}", t.name);
        }
    }
}

#[test]
fn downgrade_ansi16_only_basic() {
    for t in Theme::all() {
        for c in t.downgrade(ColorSupport::Ansi16).colors() {
            assert!(
                !matches!(c, Color::Rgb(..) | Color::Indexed(..)),
                "{}: {c:?}",
                t.name
            );
        }
    }
}

#[test]
fn downgrade_none_resets_everything() {
    for t in Theme::all() {
        let d = t.downgrade(ColorSupport::None);
        assert!(d.colors().iter().all(|c| *c == Color::Reset), "{}", t.name);
        assert_eq!(d.name, t.name);
    }
}

#[test]
fn downgrade_preserves_palette_length_and_name() {
    for t in Theme::all() {
        for s in [ColorSupport::Ansi256, ColorSupport::Ansi16] {
            let d = t.downgrade(s);
            assert_eq!(d.palette.len(), t.palette.len());
            assert_eq!(d.name, t.name);
        }
    }
}

#[test]
fn builder_chain_overrides_builtin() {
    let t = Theme::nord()
        .name("nord-red")
        .primary(Color::Red)
        .border_focused(Color::Red)
        .palette(vec![Color::Red; 8]);
    assert_eq!(t.name, "nord-red");
    assert_eq!(t.border_style(true).fg, Some(Color::Red));
    assert_eq!(t.color(13), Color::Red);
    assert_eq!(t.foreground, Theme::nord().foreground);
}
