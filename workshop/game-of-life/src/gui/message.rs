#[derive(Clone)]
pub enum MessageFromGuiToSimulator {
    Reset,
    SpawnCells(Vec<(i64, i64)>),
    RemoveCells(Vec<(i64, i64)>),
    PlacePattern(Vec<(i64, i64)>),
}
