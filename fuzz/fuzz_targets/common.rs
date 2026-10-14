//! Shared `Arbitrary` helpers for the fuzz targets.
#![allow(dead_code)]

use arbitrary::Arbitrary;
use ratatui::style::Color;

/// Any ratatui color, including every named variant.
#[derive(Debug, Clone, Copy, Arbitrary)]
pub enum FuzzColor {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl From<FuzzColor> for Color {
    fn from(c: FuzzColor) -> Color {
        match c {
            FuzzColor::Reset => Color::Reset,
            FuzzColor::Black => Color::Black,
            FuzzColor::Red => Color::Red,
            FuzzColor::Green => Color::Green,
            FuzzColor::Yellow => Color::Yellow,
            FuzzColor::Blue => Color::Blue,
            FuzzColor::Magenta => Color::Magenta,
            FuzzColor::Cyan => Color::Cyan,
            FuzzColor::Gray => Color::Gray,
            FuzzColor::DarkGray => Color::DarkGray,
            FuzzColor::LightRed => Color::LightRed,
            FuzzColor::LightGreen => Color::LightGreen,
            FuzzColor::LightYellow => Color::LightYellow,
            FuzzColor::LightBlue => Color::LightBlue,
            FuzzColor::LightMagenta => Color::LightMagenta,
            FuzzColor::LightCyan => Color::LightCyan,
            FuzzColor::White => Color::White,
            FuzzColor::Rgb(r, g, b) => Color::Rgb(r, g, b),
            FuzzColor::Indexed(i) => Color::Indexed(i),
        }
    }
}
