#![no_main]
use libfuzzer_sys::fuzz_target;
use tui_nuances::*;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    if let Some(t) = Theme::by_name(&s) {
        assert!(Theme::names().contains(&t.name.as_str()));
        assert_eq!(Theme::by_name(&t.name), Some(t));
    }
    let mut sw = ThemeSwitcher::new();
    let found = sw.set_by_name(&s);
    assert_eq!(found, Theme::by_name(&s).is_some());
    for (i, b) in data.iter().enumerate().take(64) {
        match b % 3 {
            0 => {
                sw.next();
            }
            1 => {
                sw.prev();
            }
            _ => {
                sw.set_index(i * usize::from(*b));
            }
        }
        assert!(sw.index() < sw.len());
    }
    // A theme with an arbitrary name still cycles.
    let custom = Theme::default().name(s.into_owned());
    let _ = (custom.next(), custom.prev());
});
