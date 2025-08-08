#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Return the board parameters for this difficulty as (width, height, mines)
    /// Classic Minesweeper values:
    /// - Easy/Beginner: 9x9 with 10 mines
    /// - Medium/Intermediate: 16x16 with 40 mines
    /// - Hard/Expert: 30x16 with 99 mines
    pub const fn parameters(self) -> (usize, usize, usize) {
        match self {
            Self::Easy => (9, 9, 10),
            Self::Medium => (16, 16, 40),
            Self::Hard => (30, 16, 99),
        }
    }

    /// Cycle to the next difficulty in order: Easy -> Medium -> Hard -> Easy
    pub const fn cycle(self) -> Self {
        match self {
            Self::Easy => Self::Medium,
            Self::Medium => Self::Hard,
            Self::Hard => Self::Easy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Difficulty;

    #[test]
    fn parameters_match_classic_values() {
        assert_eq!(Difficulty::Easy.parameters(), (9, 9, 10));
        assert_eq!(Difficulty::Medium.parameters(), (16, 16, 40));
        assert_eq!(Difficulty::Hard.parameters(), (30, 16, 99));
    }

    #[test]
    fn cycle_rotates_in_order() {
        assert_eq!(Difficulty::Easy.cycle(), Difficulty::Medium);
        assert_eq!(Difficulty::Medium.cycle(), Difficulty::Hard);
        assert_eq!(Difficulty::Hard.cycle(), Difficulty::Easy);
    }
}
