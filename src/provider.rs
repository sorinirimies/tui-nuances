//! Traits that connect widget crates and application theme types to [`Theme`].
//!
//! * Widget crates implement [`Themed`] on their widgets:
//!
//!   ```
//!   use tui_nuances::{Theme, Themed};
//!   use ratatui::style::{Color, Style};
//!
//!   struct Gauge { style: Style }
//!   impl Themed for Gauge {
//!       fn apply_theme(mut self, theme: &Theme) -> Self {
//!           self.style = Style::default().fg(theme.accent);
//!           self
//!       }
//!   }
//!   let g = Gauge { style: Style::default() }.apply_theme(&Theme::nord());
//!   assert_eq!(g.style.fg, Some(Theme::nord().accent));
//!   # let _ = Color::Red;
//!   ```
//!
//! * Applications implement [`ThemeProvider`] on their own theme type, then
//!   pass it anywhere via [`Themed::themed`]:
//!
//!   ```
//!   use ratatui::style::Color;
//!   use tui_nuances::{Theme, ThemeProvider};
//!
//!   struct AppTheme { bg: Color, text: Color, brand: Color }
//!
//!   impl ThemeProvider for AppTheme {
//!       fn nuance_theme(&self) -> Theme {
//!           Theme::from_base("app", Some(self.bg), self.text, self.brand, Color::Green, Color::Red)
//!       }
//!   }
//!
//!   let app = AppTheme { bg: Color::Rgb(16, 16, 24), text: Color::White, brand: Color::Magenta };
//!   assert_eq!(app.nuance_theme().primary, Color::Magenta);
//!   ```
//!
//! # `From<&T>` guidance
//!
//! Because of the orphan rule a *widget crate* cannot implement
//! `From<&AppTheme> for Theme`, but an *application* (which owns `AppTheme`)
//! can; it is a handy complement when widgets take `impl Into<Theme>`:
//! `impl From<&AppTheme> for tui_nuances::Theme { … }`. Prefer
//! [`ThemeProvider`] when you want one trait that works for every widget.

use crate::theme::Theme;

/// Implemented by widgets that can restyle themselves from a [`Theme`].
pub trait Themed: Sized {
    /// Return `self` styled from `theme`. Explicit per-item colors set on the
    /// widget should keep winning over the theme.
    #[must_use]
    fn apply_theme(self, theme: &Theme) -> Self;

    /// Apply the theme produced by any [`ThemeProvider`].
    #[must_use]
    fn themed(self, provider: &impl ThemeProvider) -> Self {
        self.apply_theme(&provider.nuance_theme())
    }
}

/// Implemented by application theme types to expose them as a [`Theme`].
pub trait ThemeProvider {
    /// Produce the shared [`Theme`] for this application theme.
    fn nuance_theme(&self) -> Theme;
}

impl ThemeProvider for Theme {
    fn nuance_theme(&self) -> Theme {
        self.clone()
    }
}

impl<T: ThemeProvider + ?Sized> ThemeProvider for &T {
    fn nuance_theme(&self) -> Theme {
        (**self).nuance_theme()
    }
}

impl<T: ThemeProvider + ?Sized> ThemeProvider for Box<T> {
    fn nuance_theme(&self) -> Theme {
        (**self).nuance_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    struct W(Color);
    impl Themed for W {
        fn apply_theme(self, t: &Theme) -> Self {
            W(t.accent)
        }
    }

    struct App;
    impl ThemeProvider for App {
        fn nuance_theme(&self) -> Theme {
            Theme::nord()
        }
    }

    #[test]
    fn theme_is_its_own_provider() {
        let t = Theme::dracula();
        assert_eq!(t.nuance_theme(), t);
    }

    #[test]
    fn themed_uses_provider() {
        assert_eq!(W(Color::Reset).themed(&App).0, Theme::nord().accent);
        assert_eq!(W(Color::Reset).themed(&&App).0, Theme::nord().accent);
    }

    #[test]
    fn boxed_dyn_provider() {
        let b: Box<dyn ThemeProvider> = Box::new(App);
        assert_eq!(b.nuance_theme().name, "nord");
        assert_eq!(W(Color::Reset).themed(&b).0, Theme::nord().accent);
    }
}
