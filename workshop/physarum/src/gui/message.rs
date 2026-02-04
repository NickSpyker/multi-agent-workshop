#[derive(Clone)]
pub enum MessageFromGuiToSimulator {
    Reset,
    SetAgentCount(usize),
    ResizeWorld(usize, usize),
    ClearTrails,
}
