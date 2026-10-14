//! Property-style tests with a deterministic xorshift generator.

use ratatui::style::Color;
use tui_nuances::color::{
    contrast_ratio, darken, ensure_contrast, lighten, mix, readable_on, relative_luminance, to_rgb,
};
use tui_nuances::{ColorSupport, Theme};

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
        match v % 4 {
            0 => Color::Indexed((v >> 8) as u8),
            _ => Color::Rgb((v >> 8) as u8, (v >> 16) as u8, (v >> 24) as u8),
        }
    }
    fn factor(&mut self) -> f64 {
        (self.next() % 3001) as f64 / 1000.0 - 1.0 // −1.0 ..= 2.0, exercises clamping
    }
}

#[test]
fn mix_is_total_and_endpoint_exact() {
    let mut r = Rng(0xDEADBEEF);
    for _ in 0..3000 {
        let (a, b) = (r.color(), r.color());
        let t = r.factor();
        let _ = mix(a, b, t);
        let (ra, rb) = (to_rgb(a).unwrap(), to_rgb(b).unwrap());
        assert_eq!(to_rgb(mix(a, b, 0.0)).unwrap(), ra);
        assert_eq!(to_rgb(mix(a, b, 1.0)).unwrap(), rb);
    }
}

#[test]
fn mix_with_self_is_identity() {
    let mut r = Rng(3);
    for _ in 0..1000 {
        let c = r.color();
        assert_eq!(to_rgb(mix(c, c, r.factor())), to_rgb(c));
    }
}

#[test]
fn luminance_monotonic_in_lighten() {
    let mut r = Rng(11);
    for _ in 0..1000 {
        let c = r.color();
        let (t1, t2) = (r.factor().abs().min(1.0), r.factor().abs().min(1.0));
        let (lo, hi) = (t1.min(t2), t1.max(t2));
        assert!(relative_luminance(lighten(c, hi)) >= relative_luminance(lighten(c, lo)) - 1e-9);
        assert!(relative_luminance(darken(c, hi)) <= relative_luminance(darken(c, lo)) + 1e-9);
    }
}

#[test]
fn contrast_with_self_is_one() {
    let mut r = Rng(13);
    for _ in 0..500 {
        let c = r.color();
        assert!((contrast_ratio(c, c) - 1.0).abs() < 1e-12);
    }
}

#[test]
fn readable_on_beats_aa_and_ensure_contrast_holds() {
    let mut r = Rng(17);
    for _ in 0..2000 {
        let (fg, bg) = (r.color(), r.color());
        assert!(contrast_ratio(readable_on(bg), bg) >= 4.5);
        assert!(contrast_ratio(ensure_contrast(fg, bg, 3.0), bg) >= 3.0);
    }
}

#[test]
fn from_base_never_panics_and_is_complete() {
    let mut r = Rng(19);
    for _ in 0..1000 {
        let bg = if r.next() % 3 == 1 {
            None
        } else {
            Some(r.color())
        };
        let t = Theme::from_base("p", bg, r.color(), r.color(), r.color(), r.color());
        assert!(t.palette.len() >= 8);
        assert_eq!(t.background, bg);
        let _ = (t.next(), t.prev(), t.downgrade(ColorSupport::Ansi16));
    }
}

#[test]
fn from_palette_handles_any_length() {
    let mut r = Rng(23);
    for n in 0..30 {
        let colors: Vec<Color> = (0..n).map(|_| r.color()).collect();
        let t = Theme::from_palette("q", &colors);
        assert!(t.palette.len() >= 8);
        assert!(!t.palette.is_empty());
    }
}

#[test]
fn downgrade_is_idempotent() {
    let mut r = Rng(29);
    for _ in 0..200 {
        let t = Theme::from_base(
            "d",
            Some(r.color()),
            r.color(),
            r.color(),
            r.color(),
            r.color(),
        );
        for s in [
            ColorSupport::Ansi256,
            ColorSupport::Ansi16,
            ColorSupport::None,
        ] {
            let once = t.downgrade(s);
            assert_eq!(once.downgrade(s), once);
        }
    }
}

#[test]
fn palette_cycle_is_periodic() {
    let t = Theme::nord();
    let n = t.palette.len();
    for i in 0..100 {
        assert_eq!(t.color(i), t.color(i + n));
    }
}
