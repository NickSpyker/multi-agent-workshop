#[derive(Clone, Debug)]
pub struct GameOfLifeConfig {
    pub paused: bool,
    pub tick_rate_per_second: f32,
}

impl Default for GameOfLifeConfig {
    fn default() -> Self {
        Self {
            paused: true,
            tick_rate_per_second: 2.0,
        }
    }
}
