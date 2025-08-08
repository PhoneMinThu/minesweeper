#![deny(clippy::all, clippy::pedantic)]

mod app;
mod board;
mod difficulty;
mod error;
mod input;
mod ui;

use crate::app::{Action, AppState, Command, Status};
use crate::difficulty::Difficulty;
use crate::input::{translate_event, Dir, InputAction};
use crate::ui::draw_app;
use crossterm::event::{poll, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

fn main() {
    // 1) Initialize terminal backend and enable raw mode
    let mut stdout = stdout();
    if let Err(e) = enable_raw_mode() {
        eprintln!("Failed to enable raw mode: {e}");
        return;
    }
    if let Err(e) = stdout.execute(EnterAlternateScreen) {
        eprintln!("Failed to enter alternate screen: {e}");
        let _ = disable_raw_mode();
        return;
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(e) => {
            let mut s = std::io::stdout();
            let _ = s.execute(LeaveAlternateScreen);
            let _ = disable_raw_mode();
            eprintln!("Failed to create terminal: {e}");
            return;
        }
    };

    // 2) Instantiate AppState with default difficulty
    let mut app = AppState::new(Difficulty::Easy);

    // 3) Event loop
    let tick = Duration::from_millis(50);
    let mut running = true;
    while running {
        // Redraw UI each tick
        let elapsed_secs = app
            .start_time
            .map(|t| t.elapsed().as_secs())
            .unwrap_or(0);
        let width = app.board.width();
        let height = app.board.height();
        let mines_total = app.board.mines();
        let flags = app.board.flags_count();
        let cursor = Some((app.cursor.x, app.cursor.y));
        let status = app.status;

        if let Err(e) = terminal.draw(|f| {
            draw_app(
                f,
                mines_total,
                flags,
                elapsed_secs,
                width,
                height,
                |x, y| app.board.cell_at(x, y),
                cursor,
                status,
            );
        }) {
            eprintln!("UI draw error: {e}");
            break;
        }

        // Poll for events, handle inputs, and update app state
        if let Ok(true) = poll(tick) {
            if let Ok(event) = read() {
                if let Some(input_action) = translate_event(event) {
                    match input_action_to_action(input_action, &app) {
                        Some(AppOrSys::Action(a)) => {
                            let cmd = app.handle_action(a);
                            match cmd {
                                Command::GameWon | Command::GameLost => {
                                    // Status already updated inside handle_action; we just redraw next tick
                                }
                                Command::Redraw | Command::None => {}
                            }
                        }
                        Some(AppOrSys::Quit) => {
                            running = false;
                        }
                        None => {}
                    }
                }
            }
        }
    }

    // 4) Restore terminal on exit
    // Drop terminal first to release the backend writer
    drop(terminal);
    let mut s: Stdout = std::io::stdout();
    let _ = s.execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
}

/// Represents either an app action to be handled or a request to quit the app
enum AppOrSys {
    Action(Action),
    Quit,
}

/// Map high-level InputAction (from crossterm) into App Action or Quit.
fn input_action_to_action(input: InputAction, app: &AppState) -> Option<AppOrSys> {
    match input {
        InputAction::Move(dir) => Some(AppOrSys::Action(match dir {
            Dir::Left => Action::MoveLeft,
            Dir::Right => Action::MoveRight,
            Dir::Up => Action::MoveUp,
            Dir::Down => Action::MoveDown,
        })),
        InputAction::Reveal => Some(AppOrSys::Action(Action::Reveal)),
        InputAction::Flag => Some(AppOrSys::Action(Action::ToggleFlag)),
        InputAction::Chord => Some(AppOrSys::Action(Action::Chord)),
        InputAction::Restart => Some(AppOrSys::Action(Action::Restart)),
        InputAction::ChangeDifficulty => {
            let next = app.difficulty.cycle();
            Some(AppOrSys::Action(Action::SetDifficulty(next)))
        }
        InputAction::Quit => Some(AppOrSys::Quit),
    }
}
