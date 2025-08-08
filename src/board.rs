use rand::rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pjb enum CellState {
    Hidden,
    Revealed(u8),
    Flagged,
}

#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    mines: usize,
    mines_placed: bool,
    minefield: Vec<bool>,
    state: Vec<CellState>,
}

impl Board {
    /// Create an empty board with all cells hidden and no mines placed yet.
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        assert!(width > 0 && height > 0, "Board dimensions must be > 0");
        assert!(mines < width * height, "Mines must be less than cell count");
        let len = width * height;
        Self {
            width,
            height,
            mines,
            mines_placed: false,
            minefield: vec![false; len],
            state: vec![CellState::Hidden; len],
        }
    }

    /// Board width in cells.
    pub const fn width(&self) -> usize { self.width }
    /// Board height in cells.
    pub const fn height(&self) -> usize { self.height }
    /// Total number of mines.
    pub const fn mines(&self) -> usize { self.mines }

    /// Return the current state of a cell at (x, y).
    pub fn cell_at(&self, x: usize, y: usize) -> CellState {
        self.state[self.idx(x, y)]
    }

    /// Count how many flags are currently placed on the board.
    pub fn flags_count(&self) -> usize {
        self.state
            .iter()
            .filter(|c| matches!(c, CellState::Flagged))
            .count()
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        (-1isize..=1)
            .flat_map(move |dy| (-1isize..=1).map(move |dx| (dx, dy)))
            .filter(move |&(dx, dy)| !(dx == 0 && dy == 0))
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|&(nx, ny)| self.in_bounds(nx, ny))
            .map(|(nx, ny)| (nx as usize, ny as usize))
    }

    pub fn adjacent_mine_count(&self, x: usize, y: usize) -> u8 {
        self.neighbors(x, y)
            .filter(|&(nx, ny)| self.minefield[self.idx(nx, ny)])
            .count() as u8
    }

    /// Lazily place mines on the first reveal, excluding a specific coordinate.
    /// Ensures the excluded position is never mined.
    pub fn place_mines_excluding(&mut self, exclude: (usize, usize)) {
        if self.mines_placed {
            return;
        }
        let total = self.width * self.height;
        let exclude_idx = self.idx(exclude.0, exclude.1);
        let mut candidates: Vec<usize> = (0..total).filter(|&i| i != exclude_idx).collect();
        let mut rng = rng();
        candidates.shuffle(&mut rng);
        for &i in candidates.iter().take(self.mines) {
            self.minefield[i] = true;
        }
        self.mines_placed = true;

        // After placing mines, precompute numbers for any already revealed cells (none in lazy start)
    }

    /// Reveal a cell. Returns true if safe, false if a mine was revealed.
    pub fn reveal(&mut self, x: usize, y: usize) -> bool {
        if !self.in_bounds(x as isize, y as isize) {
            return true; // Out of bounds treated as no-op
        }
        if !self.mines_placed {
            self.place_mines_excluding((x, y));
        }
        let i = self.idx(x, y);
        match self.state[i] {
            CellState::Hidden => {
                if self.minefield[i] {
                    // Hit a mine
                    return false;
                }
                let count = self.adjacent_mine_count(x, y);
                self.state[i] = CellState::Revealed(count);
                if count == 0 {
                    // flood fill
                    self.flood_fill_zeroes(x, y);
                }
                true
            }
            _ => true, // No-op for Revealed/Flagged
        }
    }

    fn flood_fill_zeroes(&mut self, x: usize, y: usize) {
        let mut stack = vec![(x, y)];
        while let Some((cx, cy)) = stack.pop() {
            let neighbors: Vec<(usize, usize)> = self.neighbors(cx, cy).collect();
            for (nx, ny) in neighbors {
                let idx = self.idx(nx, ny);
                if matches!(self.state[idx], CellState::Hidden) && !self.minefield[idx] {
                    let count = self.adjacent_mine_count(nx, ny);
                    self.state[idx] = CellState::Revealed(count);
                    if count == 0 {
                        stack.push((nx, ny));
                    }
                }
            }
        }
    }

    /// Toggle flag on a cell. Hidden <-> Flagged. No-op if Revealed.
    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if !self.in_bounds(x as isize, y as isize) {
            return;
        }
        let i = self.idx(x, y);
        self.state[i] = match self.state[i] {
            CellState::Hidden => CellState::Flagged,
            CellState::Flagged => CellState::Hidden,
            r @ CellState::Revealed(_) => r,
        };
    }

    /// Chord a revealed numbered cell: if number equals adjacent flag count,
    /// reveal all unflagged hidden neighbors. Returns true if safe, false if a mine was revealed.
    pub fn chord(&mut self, x: usize, y: usize) -> bool {
        if !self.in_bounds(x as isize, y as isize) {
            return true;
        }
        let i = self.idx(x, y);
        let number = match self.state[i] {
            CellState::Revealed(n) if n > 0 => n,
            _ => return true, // Only chording on revealed number cells makes sense
        };
        let mut flag_count = 0u8;
        for (nx, ny) in self.neighbors(x, y) {
            if matches!(self.state[self.idx(nx, ny)], CellState::Flagged) {
                flag_count += 1;
            }
        }
        if flag_count != number {
            return true; // Do nothing if flags don't match
        }
        let mut safe = true;
        let neighbors: Vec<(usize, usize)> = self.neighbors(x, y).collect();
        for (nx, ny) in neighbors {
            let idx = self.idx(nx, ny);
            if matches!(self.state[idx], CellState::Hidden) {
                if self.minefield[idx] {
                    // Incorrect flagging, stepped on a mine while chording
                    safe = false;
                } else {
                    let count = self.adjacent_mine_count(nx, ny);
                    self.state[idx] = CellState::Revealed(count);
                    if count == 0 {
                        self.flood_fill_zeroes(nx, ny);
                    }
                }
            }
        }
        safe
    }

    /// Check if all non-mine cells are revealed.
    pub fn is_win(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.idx(x, y);
                if !self.minefield[i] && !matches!(self.state[i], CellState::Revealed(_)) {
                    return false;
                }
            }
        }
        true
    }

    /// Render helper used by placeholder app.
    pub fn render(&self) {
        println!("Board: {}x{}, mines {}", self.width, self.height, self.mines);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.idx(x, y);
                let ch = match self.state[i] {
                    CellState::Hidden => '#',
                    CellState::Flagged => 'F',
                    CellState::Revealed(n) => char::from(b'0' + n),
                };
                print!("{} ", ch);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_with(width: usize, height: usize, mines: usize) -> Board {
        Board::new(width, height, mines)
    }

    #[test]
    fn in_bounds_works() {
        let b = board_with(3, 2, 1);
        assert!(b.in_bounds(0, 0));
        assert!(b.in_bounds(2, 1));
        assert!(!b.in_bounds(-1, 0));
        assert!(!b.in_bounds(3, 0));
        assert!(!b.in_bounds(0, 2));
    }

    #[test]
    fn neighbors_center_has_8() {
        let b = board_with(3, 3, 1);
        let ns: Vec<_> = b.neighbors(1, 1).collect();
        assert_eq!(ns.len(), 8);
    }

    #[test]
    fn neighbors_corner_has_3() {
        let b = board_with(3, 3, 1);
        let ns: Vec<_> = b.neighbors(0, 0).collect();
        assert_eq!(ns.len(), 3);
    }

    #[test]
    fn first_click_safety() {
        let mut b = board_with(5, 5, 5);
        assert!(!b.mines_placed);
        let safe = b.reveal(2, 2);
        assert!(safe);
        assert!(b.mines_placed);
        // The first clicked cell cannot be a mine
        assert!(!b.minefield[b.idx(2, 2)]);
        // The revealed cell should be Revealed
        match b.state[b.idx(2, 2)] {
            CellState::Revealed(_) => {}
            _ => panic!("first click should reveal a number"),
        }
    }

    #[test]
    fn flood_fill_reveals_zero_region() {
        // Construct a board with no mines to force flood fill of entire board
        let mut b = board_with(4, 3, 0);
        let safe = b.reveal(0, 0);
        assert!(safe);
        for y in 0..b.height {
            for x in 0..b.width {
                assert!(matches!(b.state[b.idx(x, y)], CellState::Revealed(0)));
            }
        }
        assert!(b.is_win());
    }

    #[test]
    fn toggle_flag_cycles_hidden_and_flagged() {
        let mut b = board_with(2, 2, 1);
        assert!(matches!(b.state[b.idx(0, 0)], CellState::Hidden));
        b.toggle_flag(0, 0);
        assert!(matches!(b.state[b.idx(0, 0)], CellState::Flagged));
        b.toggle_flag(0, 0);
        assert!(matches!(b.state[b.idx(0, 0)], CellState::Hidden));
    }

    #[test]
    fn chord_opens_neighbors_when_flags_match() {
        // Create a deterministic setup: place 1 mine and ensure counts
        let mut b = board_with(3, 3, 1);
        // Manually place mine to control layout
        b.mines_placed = true;
        let mine_idx = b.idx(2, 2);
        b.minefield[mine_idx] = true; // bottom-right is a mine
        // Reveal center (1,1) which should have 1 adjacent mine
        let safe = b.reveal(1, 1);
        assert!(safe);
        assert!(matches!(b.state[b.idx(1, 1)], CellState::Revealed(1)));
        // Flag the mine
        b.toggle_flag(2, 2);
        // Now chord center; should open remaining neighbors safely
        let chord_safe = b.chord(1, 1);
        assert!(chord_safe);
        for (nx, ny) in b.neighbors(1, 1) {
            if (nx, ny) != (2, 2) {
                assert!(matches!(b.state[b.idx(nx, ny)], CellState::Revealed(_)));
            }
        }
    }

    #[test]
    fn chord_incorrect_flags_can_hit_mine() {
        let mut b = board_with(2, 2, 1);
        // Place a mine at (0,0)
        b.mines_placed = true;
        let mine_idx = b.idx(0, 0);
        b.minefield[mine_idx] = true;
        // Reveal (1,1) which should have 1 adjacent mine
        assert!(b.reveal(1, 1));
        assert!(matches!(b.state[b.idx(1, 1)], CellState::Revealed(1)));
        // Incorrectly flag (1,0) instead of (0,0)
        b.toggle_flag(1, 0);
        // Chord should now attempt to open (0,0) and hit a mine
        let safe = b.chord(1, 1);
        assert!(!safe);
    }

    // New edge-case tests
    #[test]
    fn chord_noop_when_flag_count_does_not_match() {
        let mut b = board_with(3, 3, 1);
        // Place a mine at (2,2), reveal center shows 1
        b.mines_placed = true;
        b.minefield[b.idx(2, 2)] = true;
        assert!(b.reveal(1, 1));
        assert!(matches!(b.state[b.idx(1, 1)], CellState::Revealed(1)));
        // Do NOT place any flags, chording should be a no-op
        let before_hidden: usize = b
            .neighbors(1, 1)
            .filter(|&(nx, ny)| matches!(b.state[b.idx(nx, ny)], CellState::Hidden))
            .count();
        let safe = b.chord(1, 1);
        assert!(safe);
        let after_hidden: usize = b
            .neighbors(1, 1)
            .filter(|&(nx, ny)| matches!(b.state[b.idx(nx, ny)], CellState::Hidden))
            .count();
        assert_eq!(before_hidden, after_hidden, "chord should not reveal when flags don't match");
    }

    #[test]
    fn win_detection_after_revealing_all_non_mines() {
        let mut b = board_with(2, 2, 1);
        // Deterministic mine at (0,0)
        b.mines_placed = true;
        b.minefield[b.idx(0, 0)] = true;
        // Reveal all safe cells
        assert!(b.reveal(1, 0));
        assert!(b.reveal(0, 1));
        assert!(b.reveal(1, 1));
        assert!(b.is_win());
    }

    #[test]
    fn flood_fill_handles_board_edges_without_overflow() {
        // Place a single mine far from corner to create zeros near (0,0)
        let mut b = board_with(3, 3, 1);
        b.mines_placed = true;
        b.minefield[b.idx(2, 2)] = true;
        // Revealing (0,0) should not panic and should reveal a region up to numbers at the boundary
        assert!(b.reveal(0, 0));
        // Ensure all non-mine cells except those adjacent to the mine are revealed
        for y in 0..3 {
            for x in 0..3 {
                if (x, y) == (2, 2) { continue; }
                assert!(matches!(b.state[b.idx(x, y)], CellState::Revealed(_)));
            }
        }
        assert!(!matches!(b.state[b.idx(2, 2)], CellState::Revealed(_)));
    }
}

