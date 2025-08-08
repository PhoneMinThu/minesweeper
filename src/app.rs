use crate::board::Board;
use crate::difficulty::Difficulty;
use crate::error::Result;
use std::time::Instant;

/// High-level commands the UI can react to after handling an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Nothing significant changed; a redraw is optional.
    None,
    /// The board changed; request a redraw.
    Redraw,
    /// The player won; end the game and show win UI.
    GameWon,
    /// The player lost; end the game and show lose UI.
    GameLost,
}

/// Runtime game status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Playing,
    Win,
    Lose,
}

/// Logical cursor within the board grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Player input intents. The higher-level input layer should map keys/mouse to these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Reveal,
    ToggleFlag,
    Chord,
    Restart,
    SetDifficulty(Difficulty),
}

/// AppState encapsulates a single game session.
pub struct AppState {
    pub board: Board,
    pub cursor: Cursor,
    pub difficulty: Difficulty,
    pub first_click_done: bool,
    pub start_time: Option<Instant>,
    pub status: Status,
}

impl AppState {
    pub fn new(difficulty: Difficulty) -> Self {
        let (w, h, m) = difficulty.parameters();
        let board = Board::new(w, h, m);
        Self {
            board,
            cursor: Cursor::new(0, 0),
            difficulty,
            first_click_done: false,
            start_time: None,
            status: Status::Playing,
        }
    }

    /// Reset the current game while keeping the current difficulty.
    pub fn restart(&mut self) {
        let (w, h, m) = self.difficulty.parameters();
        self.board = Board::new(w, h, m);
        self.cursor = Cursor::new(0, 0);
        self.first_click_done = false;
        self.start_time = None;
        self.status = Status::Playing;
    }

    /// Handle a high-level action and return a command the UI can respond to.
    pub fn handle_action(&mut self, action: Action) -> Command {
        // If game is over, only allow restart or difficulty change.
        if !matches!(self.status, Status::Playing) {
            return match action {
                Action::Restart => {
                    self.restart();
                    Command::Redraw
                }
                Action::SetDifficulty(d) => {
                    self.difficulty = d;
                    self.restart();
                    Command::Redraw
                }
                _ => Command::None,
            };
        }

        match action {
            Action::MoveLeft => self.try_move(-1, 0),
            Action::MoveRight => self.try_move(1, 0),
            Action::MoveUp => self.try_move(0, -1),
            Action::MoveDown => self.try_move(0, 1),
            Action::Reveal => self.reveal_at_cursor(),
            Action::ToggleFlag => {
                self.board.toggle_flag(self.cursor.x, self.cursor.y);
                Command::Redraw
            }
            Action::Chord => self.chord_at_cursor(),
            Action::Restart => {
                self.restart();
                Command::Redraw
            }
            Action::SetDifficulty(d) => {
                self.difficulty = d;
                self.restart();
                Command::Redraw
            }
        }
    }

    fn try_move(&mut self, dx: isize, dy: isize) -> Command {
        let nx = self.cursor.x as isize + dx;
        let ny = self.cursor.y as isize + dy;
        if self.board.in_bounds(nx, ny) {
            self.cursor.x = nx as usize;
            self.cursor.y = ny as usize;
            Command::Redraw
        } else {
            Command::None
        }
    }

    fn ensure_timer_started(&mut self) {
        if !self.first_click_done {
            self.first_click_done = true;
            self.start_time = Some(Instant::now());
        }
    }

    fn reveal_at_cursor(&mut self) -> Command {
        self.ensure_timer_started();
        let safe = self.board.reveal(self.cursor.x, self.cursor.y);
        if !safe {
            self.status = Status::Lose;
            return Command::GameLost;
        }
        if self.board.is_win() {
            self.status = Status::Win;
            return Command::GameWon;
        }
        Command::Redraw
    }

    fn chord_at_cursor(&mut self) -> Command {
        self.ensure_timer_started();
        let safe = self.board.chord(self.cursor.x, self.cursor.y);
        if !safe {
            self.status = Status::Lose;
            return Command::GameLost;
        }
        if self.board.is_win() {
            self.status = Status::Win;
            return Command::GameWon;
        }
        Command::Redraw
    }
}

pub struct App {
    board: Board,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        let (w, h, m) = difficulty.parameters();
        let board = Board::new(w, h, m);
        Self { board }
    }

    pub fn run(&mut self) -> Result<()> {
        // Placeholder run loop
        self.board.render();
        Ok(())
    }
}

