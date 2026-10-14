#![cfg(feature = "serde")]
//! Serde roundtrips and flat-format checks.

use ratatui::style::Color;
use tui_nuances::Theme;

#[test]
fn every_theme_roundtrips() {
    for t in Theme::all() {
        let json = serde_json::to_string(&t).unwrap();
        let back: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(back, t, "{}", t.name);
    }
}

#[test]
fn pretty_json_roundtrips() {
    let t = Theme::tokyo_night();
    let back: Theme = serde_json::from_str(&serde_json::to_string_pretty(&t).unwrap()).unwrap();
    assert_eq!(back, t);
}

#[test]
fn format_is_flat_with_hex_strings() {
    let v = serde_json::to_value(Theme::dracula()).unwrap();
    let o = v.as_object().unwrap();
    assert_eq!(o["name"], "dracula");
    assert_eq!(o["background"], "#282A36");
    assert!(o["foreground"].is_string());
    assert!(o["palette"].is_array());
    assert!(o.values().all(|x| !x.is_object()), "must stay flat");
}

#[test]
fn none_background_is_omitted() {
    let v = serde_json::to_value(Theme::default()).unwrap();
    assert!(v.get("background").is_none());
    assert!(v.get("surface").is_none());
}

#[test]
fn partial_input_falls_back_to_default() {
    let t: Theme = serde_json::from_str(r##"{"name":"mine","accent":"#ff0000"}"##).unwrap();
    assert_eq!(t.name, "mine");
    assert_eq!(t.accent, Color::Rgb(255, 0, 0));
    assert_eq!(t.foreground, Theme::default().foreground);
    assert_eq!(t.palette, Theme::default().palette);
}

#[test]
fn named_colors_parse() {
    let t: Theme = serde_json::from_str(r#"{"error":"red","muted":"dark gray"}"#).unwrap();
    assert_eq!(t.error, Color::Red);
    assert_eq!(t.muted, Color::DarkGray);
}

#[test]
fn invalid_color_is_an_error() {
    assert!(serde_json::from_str::<Theme>(r#"{"accent":"not-a-color"}"#).is_err());
}

#[test]
fn empty_object_is_default() {
    assert_eq!(
        serde_json::from_str::<Theme>("{}").unwrap(),
        Theme::default()
    );
}
