#![no_main]
use libfuzzer_sys::fuzz_target;
use tui_nuances::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(t) = serde_json::from_slice::<Theme>(data) {
        let s = serde_json::to_string(&t).expect("serialize theme");
        let back: Theme = serde_json::from_str(&s).expect("roundtrip theme");
        assert_eq!(serde_json::to_string(&back).unwrap(), s);
        let _ = (t.color(7), t.style(), t.selected_style(), t.next(), t.prev());
        for s in [
            ColorSupport::TrueColor,
            ColorSupport::Ansi256,
            ColorSupport::Ansi16,
            ColorSupport::None,
        ] {
            let _ = t.downgrade(s);
        }
    }
});
