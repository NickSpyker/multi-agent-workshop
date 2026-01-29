#[derive(Clone)]
pub enum MessageFromGuiToSimulator {
    SpawnCells(Vec<(i64, i64)>),
    RemoveCells(Vec<(i64, i64)>),
}
