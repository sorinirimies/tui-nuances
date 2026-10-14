//! Mock UI built only from semantic style helpers.
//! Keys: `j`/`k` select, Tab focus other panel, `t`/`T` theme, `q` quit.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tui_nuances::{Status, Theme};

const JOBS: [(&str, Status); 6] = [
    ("build        passed", Status::Success),
    ("lint         3 warnings", Status::Warning),
    ("deploy       failed", Status::Error),
    ("docs         queued", Status::Info),
    ("test         passed", Status::Success),
    ("bench        failed", Status::Error),
];

struct App {
    theme: Theme,
    list: ListState,
    focus_list: bool,
}

fn draw(f: &mut Frame, app: &mut App) {
    let t = &app.theme;
    f.render_widget(Block::default().style(t.style()), f.area());
    let [body, footer] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(f.area());
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)]).areas(body);

    let items: Vec<ListItem> = JOBS
        .iter()
        .map(|(n, s)| ListItem::new(Line::styled(*n, t.status_style(*s))))
        .collect();
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(t.border_style(app.focus_list))
                .title(" Jobs ")
                .title_style(t.title_style()),
        )
        .highlight_style(t.selected_style())
        .highlight_symbol("▌ ");
    f.render_stateful_widget(list, left, &mut app.list);

    let mut lines = vec![
        Line::styled(format!("theme: {}", t.name), t.title_style()),
        Line::styled("muted hint text", t.muted_style()),
        Line::styled("disabled option", t.disabled_style()),
        Line::styled("search match", t.highlight_style()),
        Line::raw(""),
    ];
    lines.extend(Status::ALL.iter().map(|s| {
        Line::from(vec![
            Span::styled("● ", t.status_style(*s)),
            Span::styled(s.label(), t.status_style(*s)),
        ])
    }));
    f.render_widget(
        Paragraph::new(lines).style(t.surface_style()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(t.border_style(!app.focus_list))
                .title(" Details ")
                .title_style(t.title_style()),
        ),
        right,
    );
    f.render_widget(
        Paragraph::new("j/k select  Tab focus  t/T theme  q quit").style(t.muted_style()),
        footer,
    );
}

fn main() -> std::io::Result<()> {
    let mut app = App {
        theme: Theme::catppuccin_mocha(),
        list: ListState::default().with_selected(Some(0)),
        focus_list: true,
    };
    ratatui::run(|terminal| loop {
        terminal.draw(|f| draw(f, &mut app))?;
        if let Event::Key(k) = event::read()? {
            if k.kind != KeyEventKind::Press {
                continue;
            }
            match k.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => app.list.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.list.select_previous(),
                KeyCode::Tab => app.focus_list = !app.focus_list,
                KeyCode::Char('t') => app.theme = app.theme.next(),
                KeyCode::Char('T') => app.theme = app.theme.prev(),
                _ => {}
            }
        }
    })
}
