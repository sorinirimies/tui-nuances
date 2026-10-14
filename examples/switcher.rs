//! `ThemeSwitcher` + terminal downgrade demo.
//! Keys: `n`/→ next, `p`/← previous, `c` cycle color support, `q` quit.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_nuances::{ColorSupport, ThemeSwitcher};

const LEVELS: [ColorSupport; 4] = [
    ColorSupport::TrueColor,
    ColorSupport::Ansi256,
    ColorSupport::Ansi16,
    ColorSupport::None,
];

fn draw(f: &mut Frame, sw: &ThemeSwitcher, level: usize) {
    let support = LEVELS[level];
    let t = sw.current().downgrade(support);
    f.render_widget(Block::default().style(t.style()), f.area());
    let [body, footer] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(f.area());

    let mut lines = vec![
        Line::styled(
            format!("{} ({}/{})", t.name, sw.index() + 1, sw.len()),
            t.title_style(),
        ),
        Line::styled(format!("color support: {support:?}"), t.muted_style()),
        Line::raw(""),
    ];
    lines.push(Line::from(
        t.palette
            .iter()
            .map(|c| Span::styled("      ", Style::default().bg(*c)))
            .collect::<Vec<_>>(),
    ));
    lines.push(Line::raw(""));
    lines.push(Line::styled(" selected item ", t.selected_style()));
    lines.push(Line::styled(
        "success / error",
        t.status_style(tui_nuances::Status::Success),
    ));
    lines.push(Line::raw(""));
    for (i, name) in sw.names().iter().enumerate() {
        let (mark, st) = if i == sw.index() {
            ("▶ ", t.highlight_style())
        } else {
            ("  ", t.muted_style())
        };
        lines.push(Line::styled(format!("{mark}{name}"), st));
    }
    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(t.border_style(true))
                .title(" ThemeSwitcher ")
                .title_style(t.title_style()),
        ),
        body,
    );
    f.render_widget(
        Paragraph::new("n/→ next  p/← prev  c color support  q quit").style(t.muted_style()),
        footer,
    );
}

fn main() -> std::io::Result<()> {
    let mut sw = ThemeSwitcher::new();
    let mut level = 0;
    ratatui::run(|terminal| loop {
        terminal.draw(|f| draw(f, &sw, level))?;
        if let Event::Key(k) = event::read()? {
            if k.kind != KeyEventKind::Press {
                continue;
            }
            match k.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('n') | KeyCode::Right => {
                    sw.next();
                }
                KeyCode::Char('p') | KeyCode::Left => {
                    sw.prev();
                }
                KeyCode::Char('c') => level = (level + 1) % LEVELS.len(),
                _ => {}
            }
        }
    })
}
