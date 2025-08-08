use crate::difficulty::Difficulty;

#[derive(Debug)]
pub struct Board {
    pub difficulty: Difficulty,
}

impl Board {
    pub fn new(difficulty: Difficulty) -> Self {
        Self { difficulty }
    }

    pub fn render(&self) {
        println!("Rendering board with difficulty: {:?}", self.difficulty);
    }
}

