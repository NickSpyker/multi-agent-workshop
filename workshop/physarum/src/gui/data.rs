use crate::simulation::SpawnMode;

#[derive(Clone, Debug)]
pub struct PhysarumConfig {
    // Simulation control
    pub paused: bool,
    pub steps_per_frame: u32,

    // World size
    pub width: usize,
    pub height: usize,

    // Agent settings
    pub agent_count: usize,
    pub spawn_mode: SpawnMode,

    // Movement
    pub move_speed: f32,
    pub turn_speed: f32, // Degrees per second

    // Sensor settings
    pub sensor_angle: f32,  // Degrees from forward direction
    pub sensor_offset: f32, // Distance from agent to sensor
    pub sensor_size: i32,   // Sensor sample radius

    // Trail settings
    pub deposit_amount: f32,
    pub diffuse_rate: f32,
    pub decay_rate: f32,

    // Boundary behavior
    pub wrap_edges: bool,

    // Visual settings
    pub show_agents: bool,
    pub trail_color: [f32; 3], // RGB 0-1
}

impl Default for PhysarumConfig {
    fn default() -> Self {
        Self {
            paused: false,
            steps_per_frame: 1,

            width: 800,
            height: 600,

            agent_count: 5000,
            spawn_mode: SpawnMode::Circle,

            move_speed: 100.0,
            turn_speed: 180.0,

            sensor_angle: 30.0,
            sensor_offset: 20.0,
            sensor_size: 1,

            deposit_amount: 5.0,
            diffuse_rate: 3.0,
            decay_rate: 0.5,

            wrap_edges: true,

            show_agents: false,
            trail_color: [0.2, 0.8, 0.4], // Green
        }
    }
}
