#![no_main]
mod common;
use common::FuzzColor;
use libfuzzer_sys::fuzz_target;
use tui_nuances::color::*;

fuzz_target!(|input: (FuzzColor, FuzzColor, f64, f64)| {
    let (a, b, t, min) = input;
    let (a, b) = (a.into(), b.into());
    let _ = (mix(a, b, t), lighten(a, t), darken(a, t), with_alpha(a, b, t));
    let c = contrast_ratio(a, b);
    assert!((1.0..=21.0 + 1e-9).contains(&c), "contrast {c}");
    assert!((c - contrast_ratio(b, a)).abs() < 1e-9);
    let l = relative_luminance(a);
    assert!((0.0..=1.0 + 1e-9).contains(&l));
    assert!(contrast_ratio(readable_on(a), a) >= 4.5);
    let _ = ensure_contrast(a, b, min);
    let _ = (to_ansi16(a), to_ansi256(a), is_light(a), to_rgb(a));
});
