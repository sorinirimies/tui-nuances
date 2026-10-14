//! Color math on [`ratatui::style::Color`]: blending, luminance, WCAG contrast
//! and terminal palette quantisation.
//!
//! Every function is total: it never panics, whatever the input colors or
//! factors (NaN and out-of-range factors are clamped). Colors that cannot be
//! resolved to RGB ([`Color::Reset`]) are handled as documented per function.
//!
//! ```
//! use ratatui::style::Color;
//! use tui_nuances::color::{contrast_ratio, mix, readable_on};
//!
//! let bg = Color::Rgb(30, 30, 46);
//! let fg = readable_on(bg);
//! assert!(contrast_ratio(fg, bg) >= 4.5);
//! assert_eq!(mix(Color::Rgb(0, 0, 0), Color::Rgb(200, 100, 50), 0.5), Color::Rgb(100, 50, 25));
//! ```

use ratatui::style::Color;

/// RGB values of the 16 ANSI colors (xterm defaults), indexed by ANSI number.
const ANSI16: [(u8, u8, u8); 16] = [
    (0, 0, 0),
    (205, 0, 0),
    (0, 205, 0),
    (205, 205, 0),
    (0, 0, 238),
    (205, 0, 205),
    (0, 205, 205),
    (229, 229, 229),
    (127, 127, 127),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (92, 92, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

/// The 16 basic colors in ANSI order, matching [`ANSI16`].
const BASIC: [Color; 16] = [
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::Gray,
    Color::DarkGray,
    Color::LightRed,
    Color::LightGreen,
    Color::LightYellow,
    Color::LightBlue,
    Color::LightMagenta,
    Color::LightCyan,
    Color::White,
];

const CUBE: [u8; 6] = [0, 95, 135, 175, 215, 255];

/// Resolve a color to RGB (xterm defaults for named/indexed colors).
///
/// Returns `None` for [`Color::Reset`], whose value depends on the terminal.
#[must_use]
pub fn to_rgb(color: Color) -> Option<(u8, u8, u8)> {
    let named = |i: usize| Some(ANSI16[i]);
    match color {
        Color::Reset => None,
        Color::Black => named(0),
        Color::Red => named(1),
        Color::Green => named(2),
        Color::Yellow => named(3),
        Color::Blue => named(4),
        Color::Magenta => named(5),
        Color::Cyan => named(6),
        Color::Gray => named(7),
        Color::DarkGray => named(8),
        Color::LightRed => named(9),
        Color::LightGreen => named(10),
        Color::LightYellow => named(11),
        Color::LightBlue => named(12),
        Color::LightMagenta => named(13),
        Color::LightCyan => named(14),
        Color::White => named(15),
        Color::Rgb(r, g, b) => Some((r, g, b)),
        Color::Indexed(i) => Some(indexed_rgb(i)),
    }
}

fn indexed_rgb(i: u8) -> (u8, u8, u8) {
    match i {
        0..=15 => ANSI16[i as usize],
        16..=231 => {
            let n = (i - 16) as usize;
            (CUBE[n / 36], CUBE[(n / 6) % 6], CUBE[n % 6])
        }
        232..=255 => {
            let v = 8 + 10 * (i - 232);
            (v, v, v)
        }
    }
}

fn lerp(a: u8, b: u8, t: f64) -> u8 {
    (f64::from(a) + (f64::from(b) - f64::from(a)) * t).round() as u8
}

fn clamp01(t: f64) -> f64 {
    if t.is_nan() {
        0.0
    } else {
        t.clamp(0.0, 1.0)
    }
}

/// Linear blend from `a` (`t = 0`) to `b` (`t = 1`), clamped to `0..=1`.
///
/// Returns [`Color::Rgb`]. If either color is [`Color::Reset`] the nearer
/// endpoint is returned unchanged.
#[must_use]
pub fn mix(a: Color, b: Color, t: f64) -> Color {
    let t = clamp01(t);
    match (to_rgb(a), to_rgb(b)) {
        (Some(x), Some(y)) => Color::Rgb(lerp(x.0, y.0, t), lerp(x.1, y.1, t), lerp(x.2, y.2, t)),
        _ => {
            if t < 0.5 {
                a
            } else {
                b
            }
        }
    }
}

/// Blend toward white by `amount` (`0..=1`).
#[must_use]
pub fn lighten(color: Color, amount: f64) -> Color {
    mix(color, Color::Rgb(255, 255, 255), amount)
}

/// Blend toward black by `amount` (`0..=1`).
#[must_use]
pub fn darken(color: Color, amount: f64) -> Color {
    mix(color, Color::Rgb(0, 0, 0), amount)
}

/// Composite `fg` over `bg` with the given opacity (`0` = `bg`, `1` = `fg`).
#[must_use]
pub fn with_alpha(fg: Color, bg: Color, alpha: f64) -> Color {
    mix(bg, fg, alpha)
}

/// WCAG relative luminance in `0.0..=1.0`. [`Color::Reset`] counts as black.
#[must_use]
pub fn relative_luminance(color: Color) -> f64 {
    let (r, g, b) = to_rgb(color).unwrap_or((0, 0, 0));
    let lin = |c: u8| {
        let c = f64::from(c) / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}

/// WCAG contrast ratio in `1.0..=21.0` (symmetric in its arguments).
#[must_use]
pub fn contrast_ratio(a: Color, b: Color) -> f64 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// Black or white, whichever has the higher contrast against `bg`.
#[must_use]
pub fn readable_on(bg: Color) -> Color {
    let white = Color::Rgb(255, 255, 255);
    let black = Color::Rgb(0, 0, 0);
    if contrast_ratio(white, bg) >= contrast_ratio(black, bg) {
        white
    } else {
        black
    }
}

/// Move `fg` toward the readable extreme until it reaches `min_ratio` against
/// `bg`; returns `fg` untouched when it already qualifies.
#[must_use]
pub fn ensure_contrast(fg: Color, bg: Color, min_ratio: f64) -> Color {
    if contrast_ratio(fg, bg) >= min_ratio {
        return fg;
    }
    let target = readable_on(bg);
    (1..=20)
        .map(|i| mix(fg, target, f64::from(i) / 20.0))
        .find(|c| contrast_ratio(*c, bg) >= min_ratio)
        .unwrap_or(target)
}

/// `true` when the color's luminance is above the mid-point (a "light" color).
#[must_use]
pub fn is_light(color: Color) -> bool {
    relative_luminance(color) > 0.179
}

fn dist2(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let d = |x: u8, y: u8| {
        let d = i32::from(x) - i32::from(y);
        (d * d) as u32
    };
    d(a.0, b.0) + d(a.1, b.1) + d(a.2, b.2)
}

/// Nearest of the 16 basic named colors. [`Color::Reset`] is returned as is.
#[must_use]
pub fn to_ansi16(color: Color) -> Color {
    let Some(rgb) = to_rgb(color) else {
        return color;
    };
    if BASIC.contains(&color) {
        return color;
    }
    let best = (0..16).min_by_key(|&i| dist2(ANSI16[i], rgb)).unwrap_or(0);
    BASIC[best]
}

/// Nearest xterm-256 palette entry as [`Color::Indexed`]; named colors and
/// [`Color::Reset`] are kept, existing indexed colors are unchanged.
#[must_use]
pub fn to_ansi256(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Indexed(nearest_256((r, g, b))),
        other => other,
    }
}

fn nearest_256(rgb: (u8, u8, u8)) -> u8 {
    let level = |v: u8| {
        (0..6)
            .min_by_key(|&i| (i32::from(CUBE[i]) - i32::from(v)).abs())
            .unwrap_or(0)
    };
    let (ri, gi, bi) = (level(rgb.0), level(rgb.1), level(rgb.2));
    let cube_idx = 16 + 36 * ri + 6 * gi + bi;
    let cube_rgb = (CUBE[ri], CUBE[gi], CUBE[bi]);
    let avg = ((u32::from(rgb.0) + u32::from(rgb.1) + u32::from(rgb.2)) / 3) as i32;
    let gi = ((avg - 8 + 5) / 10).clamp(0, 23);
    let gray = (8 + 10 * gi) as u8;
    let gray_idx = 232 + gi as usize;
    if dist2((gray, gray, gray), rgb) < dist2(cube_rgb, rgb) {
        gray_idx as u8
    } else {
        cube_idx as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: Color = Color::Rgb(255, 255, 255);
    const K: Color = Color::Rgb(0, 0, 0);

    /// Tiny deterministic xorshift PRNG for property tests.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn color(&mut self) -> Color {
            let v = self.next();
            Color::Rgb(v as u8, (v >> 8) as u8, (v >> 16) as u8)
        }
        fn unit(&mut self) -> f64 {
            (self.next() % 10_001) as f64 / 10_000.0
        }
    }

    #[test]
    fn to_rgb_variants() {
        assert_eq!(to_rgb(Color::Reset), None);
        assert_eq!(to_rgb(Color::Red), Some((205, 0, 0)));
        assert_eq!(to_rgb(Color::Rgb(1, 2, 3)), Some((1, 2, 3)));
        assert_eq!(to_rgb(Color::Indexed(15)), Some((255, 255, 255)));
        assert_eq!(to_rgb(Color::Indexed(16)), Some((0, 0, 0)));
        assert_eq!(to_rgb(Color::Indexed(231)), Some((255, 255, 255)));
        assert_eq!(to_rgb(Color::Indexed(232)), Some((8, 8, 8)));
        assert_eq!(to_rgb(Color::Indexed(255)), Some((238, 238, 238)));
    }

    #[test]
    fn mix_endpoints_and_midpoint() {
        assert_eq!(mix(K, W, 0.0), K);
        assert_eq!(mix(K, W, 1.0), W);
        assert_eq!(mix(K, W, 0.5), Color::Rgb(128, 128, 128));
    }

    #[test]
    fn mix_clamps_and_handles_nan() {
        assert_eq!(mix(K, W, -3.0), K);
        assert_eq!(mix(K, W, 9.0), W);
        assert_eq!(mix(K, W, f64::NAN), K);
        assert_eq!(mix(K, W, f64::INFINITY), W);
    }

    #[test]
    fn mix_reset_returns_nearest_endpoint() {
        assert_eq!(mix(Color::Reset, W, 0.2), Color::Reset);
        assert_eq!(mix(Color::Reset, W, 0.8), W);
    }

    #[test]
    fn lighten_darken_bounds() {
        assert_eq!(lighten(K, 1.0), W);
        assert_eq!(darken(W, 1.0), K);
        assert_eq!(lighten(Color::Rgb(10, 20, 30), 0.0), Color::Rgb(10, 20, 30));
    }

    #[test]
    fn with_alpha_is_bg_to_fg() {
        assert_eq!(with_alpha(W, K, 0.0), K);
        assert_eq!(with_alpha(W, K, 1.0), W);
    }

    #[test]
    fn luminance_extremes() {
        assert!((relative_luminance(K)).abs() < 1e-9);
        assert!((relative_luminance(W) - 1.0).abs() < 1e-9);
        assert_eq!(relative_luminance(Color::Reset), 0.0);
    }

    #[test]
    fn contrast_black_white_is_21() {
        assert!((contrast_ratio(K, W) - 21.0).abs() < 1e-9);
        assert!((contrast_ratio(W, W) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn readable_on_picks_extreme() {
        assert_eq!(readable_on(K), W);
        assert_eq!(readable_on(W), K);
        assert_eq!(readable_on(Color::Rgb(30, 30, 46)), W);
        assert_eq!(readable_on(Color::Rgb(239, 241, 245)), K);
    }

    #[test]
    fn ensure_contrast_reaches_target() {
        let bg = Color::Rgb(40, 40, 40);
        let fg = ensure_contrast(Color::Rgb(60, 60, 60), bg, 4.5);
        assert!(contrast_ratio(fg, bg) >= 4.5);
        let ok = Color::Rgb(250, 250, 250);
        assert_eq!(ensure_contrast(ok, bg, 4.5), ok);
    }

    #[test]
    fn is_light_matches_luminance() {
        assert!(is_light(W));
        assert!(!is_light(K));
    }

    #[test]
    fn ansi16_exact_and_nearest() {
        assert_eq!(to_ansi16(Color::Red), Color::Red);
        assert_eq!(to_ansi16(Color::Reset), Color::Reset);
        assert_eq!(to_ansi16(Color::Rgb(250, 5, 5)), Color::LightRed);
        assert_eq!(to_ansi16(Color::Rgb(1, 1, 1)), Color::Black);
        assert_eq!(to_ansi16(Color::Indexed(15)), Color::White);
    }

    #[test]
    fn ansi256_cube_and_gray() {
        assert_eq!(to_ansi256(Color::Rgb(255, 0, 0)), Color::Indexed(196));
        assert_eq!(to_ansi256(Color::Rgb(255, 255, 255)), Color::Indexed(231));
        assert_eq!(to_ansi256(Color::Rgb(128, 128, 128)), Color::Indexed(244));
        assert_eq!(to_ansi256(Color::Red), Color::Red);
        assert_eq!(to_ansi256(Color::Indexed(7)), Color::Indexed(7));
    }

    #[test]
    fn prop_mix_stays_within_channel_bounds() {
        let mut r = Rng(0x9E37_79B9_7F4A_7C15);
        for _ in 0..2000 {
            let (a, b, t) = (r.color(), r.color(), r.unit());
            let (x, y) = (to_rgb(a).unwrap(), to_rgb(b).unwrap());
            let m = to_rgb(mix(a, b, t)).unwrap();
            assert!(m.0 >= x.0.min(y.0) && m.0 <= x.0.max(y.0));
            assert!(m.1 >= x.1.min(y.1) && m.1 <= x.1.max(y.1));
            assert!(m.2 >= x.2.min(y.2) && m.2 <= x.2.max(y.2));
        }
    }

    #[test]
    fn prop_contrast_symmetric_and_bounded() {
        let mut r = Rng(42);
        for _ in 0..2000 {
            let (a, b) = (r.color(), r.color());
            let c = contrast_ratio(a, b);
            assert!((1.0..=21.0 + 1e-9).contains(&c), "{c}");
            assert!((c - contrast_ratio(b, a)).abs() < 1e-12);
        }
    }

    #[test]
    fn prop_lighten_never_darker_darken_never_lighter() {
        let mut r = Rng(7);
        for _ in 0..2000 {
            let (c, t) = (r.color(), r.unit());
            let l = relative_luminance(c);
            assert!(relative_luminance(lighten(c, t)) >= l - 1e-9);
            assert!(relative_luminance(darken(c, t)) <= l + 1e-9);
        }
    }

    #[test]
    fn prop_readable_on_always_meets_aa_large() {
        let mut r = Rng(99);
        for _ in 0..2000 {
            let bg = r.color();
            assert!(contrast_ratio(readable_on(bg), bg) >= 4.5);
        }
    }

    #[test]
    fn prop_ensure_contrast_postcondition() {
        let mut r = Rng(1234);
        for _ in 0..1000 {
            let (fg, bg) = (r.color(), r.color());
            assert!(contrast_ratio(ensure_contrast(fg, bg, 4.5), bg) >= 4.5);
        }
    }

    #[test]
    fn prop_downgrades_yield_expected_kinds() {
        let mut r = Rng(5);
        for _ in 0..2000 {
            let c = r.color();
            assert!(matches!(to_ansi256(c), Color::Indexed(i) if i >= 16));
            assert!(BASIC.contains(&to_ansi16(c)));
        }
    }

    #[test]
    fn prop_ansi256_is_idempotent_through_rgb() {
        let mut r = Rng(77);
        for _ in 0..500 {
            let Color::Indexed(i) = to_ansi256(r.color()) else {
                panic!()
            };
            let back = Color::Rgb(indexed_rgb(i).0, indexed_rgb(i).1, indexed_rgb(i).2);
            assert_eq!(to_ansi256(back), Color::Indexed(i));
        }
    }
}
