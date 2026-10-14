//! Semantic [`Style`] helpers on [`Theme`]: ask for *meaning*, not colors.

use crate::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
};

/// Status severity, each mapped to a theme color by [`Theme::status_color`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Operation succeeded / healthy.
    Success,
    /// Needs attention.
    Warning,
    /// Failed / unhealthy.
    Error,
    /// Neutral information.
    Info,
}

impl Status {
    /// All statuses, in severity-agnostic display order.
    pub const ALL: [Status; 4] = [
        Status::Success,
        Status::Warning,
        Status::Error,
        Status::Info,
    ];

    /// Lower-case label (`"success"`, `"warning"`, `"error"`, `"info"`).
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Status::Success => "success",
            Status::Warning => "warning",
            Status::Error => "error",
            Status::Info => "info",
        }
    }
}

impl Theme {
    /// Base style: foreground (+ background when set).
    #[must_use]
    pub fn style(&self) -> Style {
        let s = Style::default().fg(self.foreground);
        match self.background {
            Some(bg) => s.bg(bg),
            None => s,
        }
    }

    /// Alias of [`Theme::style`] (the name `tui-chart` used).
    #[must_use]
    pub fn base_style(&self) -> Style {
        self.style()
    }

    /// Style for panels / cards: foreground on `surface` (falls back to the background).
    #[must_use]
    pub fn surface_style(&self) -> Style {
        let s = Style::default().fg(self.foreground);
        match self.surface.or(self.background) {
            Some(bg) => s.bg(bg),
            None => s,
        }
    }

    /// De-emphasised text style.
    #[must_use]
    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Border style; the focused variant uses `border_focused` and bold.
    #[must_use]
    pub fn border_style(&self, focused: bool) -> Style {
        if focused {
            Style::default()
                .fg(self.border_focused)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.border)
        }
    }

    /// Title style (bold).
    #[must_use]
    pub fn title_style(&self) -> Style {
        Style::default().fg(self.title).add_modifier(Modifier::BOLD)
    }

    /// Style of the selected item: `selection_fg` on `selection_bg`.
    #[must_use]
    pub fn selected_style(&self) -> Style {
        Style::default().fg(self.selection_fg).bg(self.selection_bg)
    }

    /// Style of disabled elements.
    #[must_use]
    pub fn disabled_style(&self) -> Style {
        Style::default().fg(self.disabled)
    }

    /// Style of search matches / markers: `highlight` color, bold.
    #[must_use]
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }

    /// Color for a [`Status`].
    #[must_use]
    pub fn status_color(&self, status: Status) -> Color {
        match status {
            Status::Success => self.success,
            Status::Warning => self.warning,
            Status::Error => self.error,
            Status::Info => self.info,
        }
    }

    /// Style for a [`Status`] (its color as foreground).
    #[must_use]
    pub fn status_style(&self, status: Status) -> Style {
        Style::default().fg(self.status_color(status))
    }

    /// Axis line style.
    #[must_use]
    pub fn axis_style(&self) -> Style {
        Style::default().fg(self.axis)
    }

    /// Tick / legend label style (same color as `muted`).
    #[must_use]
    pub fn label_style(&self) -> Style {
        self.muted_style()
    }

    /// Grid line style.
    #[must_use]
    pub fn grid_style(&self) -> Style {
        Style::default().fg(self.grid)
    }

    /// Tooltip style: `tooltip_fg` on `tooltip_bg`.
    #[must_use]
    pub fn tooltip_style(&self) -> Style {
        Style::default().fg(self.tooltip_fg).bg(self.tooltip_bg)
    }

    /// Fill `area` with the theme background (no-op when there is none).
    pub fn fill(&self, area: Rect, buf: &mut Buffer) {
        if let Some(bg) = self.background {
            buf.set_style(area.intersection(buf.area), Style::default().bg(bg));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_has_fg_and_bg() {
        let t = Theme::dracula();
        let s = t.style();
        assert_eq!(s.fg, Some(t.foreground));
        assert_eq!(s.bg, t.background);
        assert_eq!(Theme::default().style().bg, None);
        assert_eq!(t.base_style(), s);
    }

    #[test]
    fn surface_style_falls_back() {
        let t = Theme::dracula();
        assert_eq!(t.surface_style().bg, t.surface);
        let t = t.surface(None);
        assert_eq!(t.surface_style().bg, t.background);
    }

    #[test]
    fn border_focus_differs() {
        let t = Theme::nord();
        assert_eq!(t.border_style(false).fg, Some(t.border));
        assert_eq!(t.border_style(true).fg, Some(t.border_focused));
        assert!(t.border_style(true).add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn selected_uses_selection_colors() {
        let t = Theme::gruvbox();
        let s = t.selected_style();
        assert_eq!((s.fg, s.bg), (Some(t.selection_fg), Some(t.selection_bg)));
    }

    #[test]
    fn status_maps_each_variant() {
        let t = Theme::one_dark();
        assert_eq!(t.status_color(Status::Success), t.success);
        assert_eq!(t.status_color(Status::Warning), t.warning);
        assert_eq!(t.status_color(Status::Error), t.error);
        assert_eq!(t.status_color(Status::Info), t.info);
        assert_eq!(t.status_style(Status::Error).fg, Some(t.error));
    }

    #[test]
    fn status_labels_unique() {
        let mut l: Vec<_> = Status::ALL.iter().map(|s| s.label()).collect();
        l.sort_unstable();
        l.dedup();
        assert_eq!(l.len(), 4);
    }

    #[test]
    fn misc_styles() {
        let t = Theme::monokai();
        assert_eq!(t.muted_style().fg, Some(t.muted));
        assert_eq!(t.label_style(), t.muted_style());
        assert_eq!(t.disabled_style().fg, Some(t.disabled));
        assert_eq!(t.axis_style().fg, Some(t.axis));
        assert_eq!(t.grid_style().fg, Some(t.grid));
        assert_eq!(t.highlight_style().fg, Some(t.highlight));
        assert_eq!(t.tooltip_style().bg, Some(t.tooltip_bg));
        assert!(t.title_style().add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn fill_paints_background_within_buffer() {
        let t = Theme::dracula();
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 2));
        t.fill(Rect::new(0, 0, 10, 10), &mut buf);
        assert_eq!(buf[(3, 1)].bg, t.background.unwrap());
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 2));
        Theme::default().fill(buf.area, &mut buf);
        assert_eq!(buf[(0, 0)].bg, Color::Reset);
    }
}
