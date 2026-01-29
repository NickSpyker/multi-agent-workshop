#[derive(Clone)]
pub enum MessageFromGuiToSimulator {
    SpawnCells(Vec<(i64, i64)>),
    RemoveCells(Vec<(i64, i64)>),
    Reset,
    /// Place a pattern at a specific position (cells are already offset to position)
    PlacePattern(Vec<(i64, i64)>),
}
