use crate::board::Board;
use crate::difficulty::Difficulty;
use crate::error::Result;

pub struct App {
    board: Board,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        let board = Board::new(difficulty);
        Self { board }
    }

    pub fn run(&mut self) -> Result<()> {
        // Placeholder run loop
        self.board.render();
        Ok(())
    }
}

