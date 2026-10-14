# tui-nuances Development Rules

- Rust library: shared theming for ratatui widget libraries. MIT. MSRV 1.88.
- Layout: src/{lib,theme,builtin,color,style,switcher,provider,support,macros}.rs, tests/, examples/, fuzz/, scripts/ (nushell).
- Gates: cargo fmt, clippy -D warnings, cargo test --all-features, cargo doc, cargo deny, `nu scripts/tests/run_all.nu`.
- Every pub item documented (`#![warn(missing_docs)]`).
