#[derive(Clone, Debug)]
pub struct BoidsConfig {
    pub paused: bool,
    pub tick_rate_per_second: f32,

    // Boid count
    pub boid_count: usize,

    // Movement parameters
    pub max_speed: f32,
    pub min_speed: f32,

    // Behavioral weights
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,

    // Perception radii
    pub separation_radius: f32,
    pub alignment_radius: f32,
    pub cohesion_radius: f32,

    // Field of view (in degrees)
    pub field_of_view: f32,

    // Visual settings
    pub boid_size: f32,
    pub show_vision_radius: bool,
}

impl Default for BoidsConfig {
    fn default() -> Self {
        Self {
            paused: false,
            tick_rate_per_second: 60.0,

            boid_count: 200,

            max_speed: 150.0,
            min_speed: 50.0,

            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,

            separation_radius: 25.0,
            alignment_radius: 50.0,
            cohesion_radius: 75.0,

            field_of_view: 270.0,

            boid_size: 6.0,
            show_vision_radius: false,
        }
    }
}
