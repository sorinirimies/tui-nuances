//! Palette gallery: every built-in theme as a row of swatches, plus the slots
//! of the selected theme.  Keys: `t`/→ next theme, `T`/← previous, `q` quit.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_nuances::ThemeSwitcher;

fn swatch(color: ratatui::style::Color) -> Span<'static> {
    Span::styled("   ", Style::default().bg(color))
}

fn draw(f: &mut Frame, sw: &ThemeSwitcher) {
    let theme = sw.current();
    f.render_widget(Block::default().style(theme.style()), f.area());
    let [body, footer] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(f.area());
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(body);

    let rows: Vec<Line> = sw
        .themes()
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let marker = if i == sw.index() { "▶ " } else { "  " };
            let mut spans = vec![Span::styled(
                format!("{marker}{:<18}", t.name),
                if i == sw.index() {
                    theme.selected_style()
                } else {
                    theme.style()
                },
            )];
            spans.extend(t.palette.iter().take(8).map(|c| swatch(*c)));
            Line::from(spans)
        })
        .collect();
    f.render_widget(
        Paragraph::new(rows).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style(true))
                .title(" Themes ")
                .title_style(theme.title_style()),
        ),
        left,
    );

    let slots = [
        ("foreground", theme.foreground),
        ("muted", theme.muted),
        ("disabled", theme.disabled),
        ("border", theme.border),
        ("border_focused", theme.border_focused),
        ("primary", theme.primary),
        ("secondary", theme.secondary),
        ("accent", theme.accent),
        ("success", theme.success),
        ("warning", theme.warning),
        ("error", theme.error),
        ("info", theme.info),
        ("selection_bg", theme.selection_bg),
        ("highlight", theme.highlight),
        ("bull", theme.bull),
        ("bear", theme.bear),
        ("neutral", theme.neutral),
        ("grid", theme.grid),
    ];
    let lines: Vec<Line> = slots
        .iter()
        .map(|(n, c)| {
            Line::from(vec![
                Span::styled(format!("{n:<16}"), theme.muted_style()),
                swatch(*c),
                Span::styled(format!(" {c}"), theme.style()),
            ])
        })
        .collect();
    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style(false))
                .title(format!(" {} ", theme.name))
                .title_style(theme.title_style()),
        ),
        right,
    );
    f.render_widget(
        Paragraph::new("t/→ next theme   T/← previous   q quit").style(theme.muted_style()),
        footer,
    );
}

fn main() -> std::io::Result<()> {
    let mut sw = ThemeSwitcher::new();
    ratatui::run(|terminal| loop {
        terminal.draw(|f| draw(f, &sw))?;
        if let Event::Key(k) = event::read()? {
            if k.kind != KeyEventKind::Press {
                continue;
            }
            match k.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('t') | KeyCode::Right => {
                    sw.next();
                }
                KeyCode::Char('T') | KeyCode::Left => {
                    sw.prev();
                }
                _ => {}
            }
        }
    })
}
