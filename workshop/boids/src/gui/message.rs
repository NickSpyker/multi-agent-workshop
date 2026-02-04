#[derive(Clone)]
pub enum MessageFromGuiToSimulator {
    Reset,
    SpawnBoids(usize),
    SetBoidCount(usize),
    ResizeWorld(f32, f32),
}
