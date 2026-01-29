use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct GameOfLife {
    pub cells: HashSet<(i64, i64)>,
    pub generation: u64,
}

impl Default for GameOfLife {
    fn default() -> Self {
        let mut cells: HashSet<(i64, i64)> = HashSet::new();

        cells.insert((1, -3));
        cells.insert((2, -2));
        cells.insert((2, -3));
        cells.insert((2, -4));
        cells.insert((3, -4));

        Self {
            cells,
            generation: 0,
        }
    }
}

impl GameOfLife {
    #[inline]
    pub fn spawn(&mut self, cells: Vec<(i64, i64)>) {
        self.cells.extend(cells);
    }

    #[inline]
    pub fn remove(&mut self, cells: Vec<(i64, i64)>) {
        for cell in cells {
            self.cells.remove(&cell);
        }
    }
}
