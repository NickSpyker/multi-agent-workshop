use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct GameOfLife {
    pub cells: HashSet<(i64, i64)>,
    pub generation: u64,
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
