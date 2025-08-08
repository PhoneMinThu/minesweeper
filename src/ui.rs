use crate::app::Status;
use crate::board::CellState;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

/// Draw the entire app UI composed of header, board, optional overlay, and footer.
///
/// Parameters:
/// - mines_total: total mines for the board
/// - flags: current number of placed flags
/// - elapsed_secs: elapsed seconds since first reveal
/// - width/height: board dimensions in cells
/// - cell_at: returns the current CellState at (x, y)
/// - cursor: optional (x, y) cursor position to highlight
/// - status: current game status to decide on overlay text
pub fn draw_app<FGet>(
    f: &mut Frame<'_>,
    mines_total: usize,
    flags: usize,
    elapsed_secs: u64,
    width: usize,
    height: usize,
    mut cell_at: FGet,
    cursor: Option<(usize, usize)>,
    status: Status,
) where
    FGet: FnMut(usize, usize) -> CellState,
{
    let area = f.size();

    // Vertical layout: header (3), board (auto), footer (3)
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(area);

    draw_header(f, layout[0], mines_total, flags, elapsed_secs);
    draw_board(f, layout[1], width, height, &mut cell_at, cursor);
    draw_footer(f, layout[2]);

    // Overlay for game end
    match status {
        Status::Win => draw_overlay(f, area, "You win! Press R to restart or D to change difficulty"),
        Status::Lose => draw_overlay(f, area, "Boom! You lost. Press R to restart or D to change difficulty"),
        Status::Playing => {}
    }
}

/// Draw header showing remaining mines and timer.
pub fn draw_header(f: &mut Frame<'_>, area: Rect, mines_total: usize, flags: usize, elapsed_secs: u64) {
    let mines_left = mines_total.saturating_sub(flags);
    let (mm, ss) = (elapsed_secs / 60, elapsed_secs % 60);
    let time_text = format!("{:02}:{:02}", mm, ss);

    let spans = vec![
        Span::styled(
            format!(" Mines: {mines_left} "),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" Time: {time_text} "),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ];

    let para = Paragraph::new(Line::from(spans))
        .block(Block::default().title(" Minesweeper ").borders(Borders::ALL));
    f.render_widget(para, area);
}

/// Draw the footer with key legend.
pub fn draw_footer(f: &mut Frame<'_>, area: Rect) {
    let legend = concat!(
        "Move: [1mArrows[0m/WASD  ",
        "Reveal: [1mEnter[0m/Space  ",
        "Flag: [1mF[0m  ",
        "Chord: [1mC[0m  ",
        "Restart: [1mR[0m  ",
        "Difficulty: [1mD[0m  ",
        "Quit: [1mQ[0m",
    );

    let para = Paragraph::new(legend)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(para, area);
}

/// Draw the central game board as a grid of Unicode glyphs with colors per number.
pub fn draw_board<FGet>(
    f: &mut Frame<'_>,
    area: Rect,
    width: usize,
    height: usize,
    cell_at: &mut FGet,
    cursor: Option<(usize, usize)>,
) where
    FGet: FnMut(usize, usize) -> CellState,
{
    // Build content line by line. Each cell is 2-character wide for spacing.
    let mut lines: Vec<Line> = Vec::with_capacity(height);
    for y in 0..height {
        let mut spans: Vec<Span> = Vec::with_capacity(width);
        for x in 0..width {
            let cell = cell_at(x, y);
            let (symbol, style) = cell_symbol_and_style(cell);
            let mut style = style;
            if let Some((cx, cy)) = cursor {
                if cx == x && cy == y {
                    style = style.bg(Color::Gray).add_modifier(Modifier::REVERSED);
                }
            }
            // Add a space after each glyph to improve readability
            spans.push(Span::styled(symbol, style));
            spans.push(Span::raw(" "));
        }
        lines.push(Line::from(spans));
    }

    let block = Block::default().borders(Borders::ALL).title(" Board ");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

/// Map a cell to a printable unicode symbol and color style.
fn cell_symbol_and_style(cell: CellState) -> (String, Style) {
    match cell {
        CellState::Hidden => ("■".to_string(), Style::default().fg(Color::DarkGray)),
        CellState::Flagged => ("⚑".to_string(), Style::default().fg(Color::Red)),
        CellState::Revealed(0) => ("·".to_string(), Style::default().fg(Color::Gray)),
        CellState::Revealed(n) => {
            let color = match n {
                1 => Color::Blue,
                2 => Color::Green,
                3 => Color::Red,
                4 => Color::Magenta,
                5 => Color::LightRed,
                6 => Color::Cyan,
                7 => Color::Yellow,
                _ => Color::LightMagenta,
            };
            (format!("{n}"), Style::default().fg(color).add_modifier(Modifier::BOLD))
        }
    }
}

/// Draw a centered overlay with a message.
pub fn draw_overlay(f: &mut Frame<'_>, area: Rect, message: &str) {
    let overlay_area = centered_rect(60, 25, area);
    let block = Block::default()
        .title(" Game Over ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let para = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(block);

    // Clear behind the overlay, then render
    f.render_widget(Clear, overlay_area);
    f.render_widget(para, overlay_area);
}

/// Helper to create a centered rect with a given percentage size.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vert[1]);

    horz[1]
}

