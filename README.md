# tui-nuances

[![Crates.io](https://img.shields.io/crates/v/tui-nuances.svg)](https://crates.io/crates/tui-nuances)
[![docs.rs](https://docs.rs/tui-nuances/badge.svg)](https://docs.rs/tui-nuances)
[![tui-nuances installs](https://img.shields.io/crates/d/tui-nuances?label=tui-nuances%20installs&logo=rust&color=orange)](https://crates.io/crates/tui-nuances)
[![CI](https://github.com/sorinirimies/tui-nuances/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-nuances/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Shared **theming for [ratatui](https://ratatui.rs) widget libraries**. Build one
`Theme`, style every component of your app from it: `tui-chart`, `tui-slider`,
`tui-spinner`, `tui-rating`, `tui-piechart`, `tui-checkbox`, `tui-file-explorer`
and your own widgets.

![gallery](examples/vhs/output/gallery.gif)

## Features

- **One `Theme`** with ~30 named slots: UI (`foreground`, `muted`, `border`,
  `border_focused`, `primary`, `selection_bg/fg` …), status (`success`, `warning`,
  `error`, `info`) and data-viz (`palette`, `bull`, `bear`, `axis`, `grid`, `crosshair`, tooltip).
- **16 built-in themes**: `default`, `catppuccin-mocha`, `catppuccin-latte`, `dracula`,
  `nord`, `gruvbox`, `tokyo-night`, `solarized-dark`, `solarized-light`, `one-dark`,
  `rose-pine`, `monokai`, `porsche`, `colorblind`, `high-contrast`, `mono`.
  Every theme (except the terminal-native `default`/`mono`) passes WCAG AA for text.
- **Derive a theme** from what you have: `Theme::from_base(..)`, `Theme::from_palette(..)`.
- **Semantic styles**: `style()`, `muted_style()`, `border_style(focused)`,
  `title_style()`, `selected_style()`, `disabled_style()`, `status_style(Status)`.
- **Color math** (`tui_nuances::color`): mix / lighten / darken, relative luminance,
  contrast ratio, `readable_on`, `ensure_contrast`, 256/16-color quantisation.
- **Terminal aware**: `ColorSupport::detect()` (`COLORTERM`, `TERM`, `NO_COLOR`) and
  `Theme::downgrade(support)`.
- **Glue traits**: `Themed` (widgets) and `ThemeProvider` (your app theme type).
- **`ThemeSwitcher`** for demos and settings screens.
- Optional **`serde`** feature: themes load from flat JSON/TOML-style maps of color strings.

## Installation

```sh
cargo add tui-nuances
# with serde support:
cargo add tui-nuances --features serde
```

or in `Cargo.toml`:

```toml
[dependencies]
tui-nuances = "0.1"
# with serde support:
# tui-nuances = { version = "0.1", features = ["serde"] }
```

## Quick start

```rust
use ratatui::widgets::{Block, Borders, Paragraph};
use tui_nuances::{Status, Theme};

let theme = Theme::by_name("tokyo-night").unwrap_or_default();

let panel = Block::default()
    .borders(Borders::ALL)
    .border_style(theme.border_style(true))      // focused border
    .title_style(theme.title_style())
    .style(theme.style());                       // fg + bg
let status = Paragraph::new("deploy ok").style(theme.status_style(Status::Success));
```

### Derive from your own colors

```rust
use ratatui::style::Color;
use tui_nuances::Theme;

let theme = Theme::from_base(
    "app",
    Some(Color::Rgb(16, 16, 24)),   // background
    Color::Rgb(230, 230, 240),      // foreground
    Color::Rgb(213, 0, 28),         // accent
    Color::Green,                   // bull / success
    Color::Red,                     // bear / error
)
.warning(Color::Yellow);            // every slot has a builder setter
```

### Plug in your app's theme type

```rust
use tui_nuances::{Theme, ThemeProvider};

struct AppTheme { /* ... */ }
impl ThemeProvider for AppTheme {
    fn nuance_theme(&self) -> Theme { Theme::nord() }
}
```

Widget crates implement `Themed { fn apply_theme(self, &Theme) -> Self }`; then
`widget.themed(&app_theme)` or `widget.apply_theme(&theme)` works everywhere.

### Adapt to the terminal

```rust
use tui_nuances::Theme;

let theme = Theme::dracula().downgrade_for_terminal(); // honours NO_COLOR / COLORTERM / TERM
```

## Examples

| Example | Run | Demo |
|---|---|---|
| `gallery` – swatches of every theme, `t`/`T` cycle | `cargo run --example gallery` | ![gallery](examples/vhs/output/gallery.gif) |
| `semantic` – mock UI using only semantic styles | `cargo run --example semantic` | ![semantic](examples/vhs/output/semantic.gif) |
| `switcher` – `ThemeSwitcher` + color downgrade | `cargo run --example switcher` | ![switcher](examples/vhs/output/switcher.gif) |

Regenerate the GIFs with `nu scripts/generate_all_tapes.nu` ([vhs](https://github.com/charmbracelet/vhs) required).

## Migrating from `tui-chart`'s `Theme`

All chart fields keep their names (`background`, `foreground`, `muted`, `axis`, `grid`,
`border`, `title`, `palette`, `bull`, `bear`, `neutral`, `accent`, `crosshair`,
`tooltip_fg`, `tooltip_bg`) and the helpers `base_style`, `axis_style`, `label_style`,
`grid_style`, `fill`, `color(i)`, `next`/`prev`, `all` are available.

## Development

```sh
just test-all   # cargo test --all-features + nushell script tests
just fuzz-build # needs nightly + cargo-fuzz
```

See [CONTRIBUTING.md](CONTRIBUTING.md). Licensed under [MIT](LICENSE).
