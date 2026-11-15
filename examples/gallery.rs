//! Palette gallery: every built-in theme as a tile painted in its own colors,
//! on several pages. Dark themes come first, then the light ones, so paging
//! never flashes between light and dark.  Keys: `n`/→/space next page,
//! `p`/← previous page, `q` quit.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use tui_nuances::Theme;

const TILE_W: u16 = 38;

fn is_light(t: &Theme) -> bool {
    match t.background {
        Some(Color::Rgb(r, g, b)) => {
            (u32::from(r) * 299 + u32::from(g) * 587 + u32::from(b) * 114) / 1000 > 140
        }
        _ => false,
    }
}

/// All built-in themes: dark first, then light (each group keeps cycle order).
fn ordered() -> Vec<Theme> {
    let (light, dark): (Vec<Theme>, Vec<Theme>) = Theme::all().into_iter().partition(is_light);
    dark.into_iter().chain(light).collect()
}

fn tile(t: &Theme, width: u16) -> Line<'static> {
    let mut spans = vec![Span::styled(format!(" {:<20}", t.name), t.style())];
    spans.extend(t.palette.iter().take(8).map(|c| {
        Span::styled(
            "██",
            Style::default()
                .fg(*c)
                .bg(t.background.unwrap_or(Color::Reset)),
        )
    }));
    let used: usize = spans.iter().map(Span::width).sum();
    spans.push(Span::styled(
        " ".repeat(usize::from(width).saturating_sub(used)),
        t.style(),
    ));
    Line::from(spans)
}

fn grid(area: Rect) -> (usize, usize) {
    let cols = usize::from((area.width / TILE_W).max(1));
    (cols, usize::from(area.height.max(1)))
}

/// Split into pages of `per_page`; light themes always start a fresh page.
fn paginate(themes: &[Theme], per_page: usize) -> Vec<&[Theme]> {
    let split = themes.iter().position(is_light).unwrap_or(themes.len());
    let (dark, light) = themes.split_at(split);
    dark.chunks(per_page)
        .chain(light.chunks(per_page))
        .collect()
}

fn body_area(full: Rect) -> Rect {
    Rect::new(0, 0, full.width, full.height.saturating_sub(2))
}

fn page_count(themes: &[Theme], full: Rect) -> usize {
    let (cols, rows) = grid(body_area(full));
    paginate(themes, cols * rows).len()
}

fn draw(f: &mut Frame, themes: &[Theme], page: usize) {
    let base = Theme::default();
    let [head, body, foot] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(f.area());
    let (cols, rows) = grid(body);
    let pages = paginate(themes, cols * rows);
    let page = page.min(pages.len() - 1);
    let slice = pages[page];
    let kind = if slice.iter().any(is_light) {
        "light"
    } else {
        "dark"
    };

    f.render_widget(
        Paragraph::new(format!(
            " tui-nuances: {} themes   page {}/{}   ({kind})",
            themes.len(),
            page + 1,
            pages.len()
        ))
        .style(base.title_style()),
        head,
    );
    let col_w = body.width / cols as u16;
    for (c, chunk) in slice
        .chunks(slice.len().div_ceil(cols).min(rows))
        .enumerate()
    {
        let r = Rect::new(body.x + c as u16 * col_w, body.y, col_w, body.height);
        let lines: Vec<Line> = chunk.iter().map(|t| tile(t, col_w)).collect();
        f.render_widget(Paragraph::new(lines), r);
    }
    f.render_widget(
        Paragraph::new("n/→ next page   p/← previous page   q quit").style(base.muted_style()),
        foot,
    );
}

fn main() -> std::io::Result<()> {
    let themes = ordered();
    let mut page = 0usize;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|f| draw(f, &themes, page))?;
            if let Event::Key(k) = event::read()? {
                if k.kind != KeyEventKind::Press {
                    continue;
                }
                let pages = page_count(&themes, terminal.get_frame().area());
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('n' | ' ') | KeyCode::Right => page = (page + 1) % pages,
                    KeyCode::Char('p') | KeyCode::Left => page = (page + pages - 1) % pages,
                    _ => {}
                }
            }
        }
    })
}
