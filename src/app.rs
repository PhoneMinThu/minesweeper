use crate::board::Board;
use crate::difficulty::Difficulty;
use crate::error::Result;

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

