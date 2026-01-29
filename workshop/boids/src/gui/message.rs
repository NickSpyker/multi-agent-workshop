#[derive(Clone)]
pub enum MessageFromGuiToSimulator {
    Reset,
    SpawnBoids(usize),
    ResizeWorld(f32, f32),
}
