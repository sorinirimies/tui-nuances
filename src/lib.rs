//! # tui-nuances
//!
//! Shared theming for [ratatui](https://ratatui.rs) widget libraries
//! (`tui-chart`, `tui-slider`, `tui-spinner`, `tui-rating`, `tui-piechart`,
//! `tui-checkbox`, `tui-file-explorer`, …): build **one** [`Theme`] and style
//! every component of your app from it.
//!
//! * [`Theme`] – ~30 named color slots (UI, status, selection, data-viz) with
//!   104 built-ins (see [`Theme::names`]), among them 29 light themes
//!   (`default-light`, `catppuccin-latte`, `solarized-light`, `github-light`, `gruvbox-light`, `tokyo-night-day`, `rose-pine-dawn`, `atom-one-light`, `ayu-light`, `everforest-light`, `dayfox`, `kanagawa-lotus`, `flexoki-light`, `modus-operandi`, `alabaster`, `edge-light`, `selenized-light`, `bluloco-light`, `papercolor-light`, `synthwave-dawn`, `tokyo-night-light`, `light-owl`, `gruvbox-material-light`, `dawnfox`, `material-lighter`, `iceberg-light`, `clair-obscur-canvas`, `mario-overworld`, `meadow`),
//!   a synthwave family (`synthwave`, `synthwave-outrun`, `synthwave-dawn`, `retrowave`, `vaporwave`, `laserwave`, `fairyfloss`, `neon`, `cyberpunk`) and the accessibility
//!   themes `colorblind`, `high-contrast` and `mono`.
//! * [`Theme::from_base`] / [`Theme::from_palette`] – derive a full theme from a
//!   few colors your app already has.
//! * Semantic styles – [`Theme::style`], [`Theme::selected_style`],
//!   [`Theme::border_style`], [`Theme::status_style`] …
//! * [`color`] – mix / lighten / darken, WCAG contrast, [`color::readable_on`],
//!   256/16-color quantisation.
//! * [`ColorSupport`] + [`Theme::downgrade`] – adapt to the terminal
//!   (`COLORTERM`, `TERM`, `NO_COLOR`).
//! * [`Themed`] / [`ThemeProvider`] – glue for widget crates and app theme types.
//! * [`ThemeSwitcher`] – cycle themes in demos.
//!
//! ```
//! use ratatui::{style::Color, widgets::{Block, Borders, Paragraph}};
//! use tui_nuances::{Status, Theme};
//!
//! let theme = Theme::by_name("tokyo-night").unwrap();
//! let block = Block::default()
//!     .borders(Borders::ALL)
//!     .border_style(theme.border_style(true))
//!     .title_style(theme.title_style())
//!     .style(theme.style());
//! let ok = Paragraph::new("all good").style(theme.status_style(Status::Success));
//! # let _ = (block, ok);
//! assert!(theme.palette.len() >= 8);
//! ```
//!
//! # Features
//!
//! * `serde` – `Serialize`/`Deserialize` for [`Theme`] (flat map of color strings).
//!
//! # Depending on it
//!
//! ```toml
//! tui-nuances = { git = "https://github.com/sorinirimies/tui-nuances" }
//! ```

#![warn(missing_docs)]

#[macro_use]
mod macros;

mod builtin;
pub mod color;
mod provider;
mod roles;
mod style;
mod support;
mod switcher;
mod theme;

pub use provider::{ThemeProvider, Themed};
pub use style::Status;
pub use support::ColorSupport;
pub use switcher::ThemeSwitcher;
pub use theme::{DEFAULT_PALETTE, Theme};
