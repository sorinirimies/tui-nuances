#![no_main]
mod common;
use common::FuzzColor;
use libfuzzer_sys::fuzz_target;
use tui_nuances::*;

fuzz_target!(|input: (Option<FuzzColor>, FuzzColor, FuzzColor, FuzzColor, FuzzColor, Vec<FuzzColor>)| {
    let (bg, fg, accent, bull, bear, pal) = input;
    let t = Theme::from_base("fuzz", bg.map(Into::into), fg.into(), accent.into(), bull.into(), bear.into());
    assert!(t.palette.len() >= 8);
    let _ = (t.next(), t.prev(), t.style(), t.border_style(true));
    let d = t.downgrade(ColorSupport::Ansi256);
    assert_eq!(d.palette.len(), t.palette.len());
    let pal: Vec<_> = pal.into_iter().map(Into::into).collect();
    let p = Theme::from_palette("fuzz", &pal);
    assert!(p.palette.len() >= 8);
    let _ = p.color(usize::MAX);
});
