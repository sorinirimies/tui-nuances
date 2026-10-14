//! [`ThemeSwitcher`]: cycle through themes in demos and settings screens.

use crate::theme::Theme;

/// A wrapping cursor over a list of themes.
///
/// ```
/// use tui_nuances::ThemeSwitcher;
///
/// let mut s = ThemeSwitcher::new();
/// let first = s.current().name.clone();
/// s.next();
/// assert_ne!(s.current().name, first);
/// s.prev();
/// assert_eq!(s.current().name, first);
/// assert!(s.set_by_name("nord"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeSwitcher {
    themes: Vec<Theme>,
    index: usize,
}

impl Default for ThemeSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeSwitcher {
    /// Switcher over all built-in themes, starting at the first.
    #[must_use]
    pub fn new() -> Self {
        Self {
            themes: Theme::all(),
            index: 0,
        }
    }

    /// Switcher over custom themes. An empty list falls back to the built-ins.
    #[must_use]
    pub fn with_themes(themes: Vec<Theme>) -> Self {
        if themes.is_empty() {
            Self::new()
        } else {
            Self { themes, index: 0 }
        }
    }

    /// The active theme.
    #[must_use]
    pub fn current(&self) -> &Theme {
        &self.themes[self.index]
    }

    /// Index of the active theme.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Number of themes (always at least 1).
    #[must_use]
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    /// Always `false`; a switcher holds at least one theme.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// All themes in order.
    #[must_use]
    pub fn themes(&self) -> &[Theme] {
        &self.themes
    }

    /// Names of all themes in order.
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        self.themes.iter().map(|t| t.name.as_str()).collect()
    }

    /// Advance (wrapping) and return the new active theme.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> &Theme {
        self.index = (self.index + 1) % self.themes.len();
        self.current()
    }

    /// Step back (wrapping) and return the new active theme.
    pub fn prev(&mut self) -> &Theme {
        self.index = (self.index + self.themes.len() - 1) % self.themes.len();
        self.current()
    }

    /// Jump to `index`, wrapping modulo the number of themes.
    pub fn set_index(&mut self, index: usize) -> &Theme {
        self.index = index % self.themes.len();
        self.current()
    }

    /// Select a theme by name (same normalisation as [`Theme::by_name`]).
    /// Returns `false` and keeps the selection when not found.
    pub fn set_by_name(&mut self, name: &str) -> bool {
        let key = name.trim().to_ascii_lowercase().replace(['_', ' '], "-");
        match self.themes.iter().position(|t| t.name == key) {
            Some(i) => {
                self.index = i;
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_first() {
        let s = ThemeSwitcher::new();
        assert_eq!(s.index(), 0);
        assert_eq!(s.len(), Theme::all().len());
        assert!(!s.is_empty());
    }

    #[test]
    fn next_wraps() {
        let mut s = ThemeSwitcher::new();
        for _ in 0..s.len() {
            s.next();
        }
        assert_eq!(s.index(), 0);
    }

    #[test]
    fn prev_wraps() {
        let mut s = ThemeSwitcher::new();
        s.prev();
        assert_eq!(s.index(), s.len() - 1);
    }

    #[test]
    fn set_index_wraps() {
        let mut s = ThemeSwitcher::new();
        s.set_index(s.len() + 2);
        assert_eq!(s.index(), 2);
    }

    #[test]
    fn set_by_name_found_and_missing() {
        let mut s = ThemeSwitcher::new();
        assert!(s.set_by_name("Tokyo Night"));
        assert_eq!(s.current().name, "tokyo-night");
        assert!(!s.set_by_name("zzz"));
        assert_eq!(s.current().name, "tokyo-night");
    }

    #[test]
    fn custom_and_empty() {
        let s = ThemeSwitcher::with_themes(vec![Theme::default().name("a")]);
        assert_eq!(s.names(), vec!["a"]);
        assert_eq!(ThemeSwitcher::with_themes(vec![]).len(), Theme::all().len());
        let mut one = s;
        one.next();
        assert_eq!(one.index(), 0);
    }
}
