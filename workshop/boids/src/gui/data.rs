#[derive(Clone, Debug)]
pub struct BoidsConfig {
    pub paused: bool,
    pub boid_count: usize,
    pub max_speed: f32,
    pub min_speed: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub separation_radius: f32,
    pub alignment_radius: f32,
    pub cohesion_radius: f32,
    pub field_of_view: f32,
    pub boid_size: f32,
    pub show_vision_radius: bool,
}

impl Default for BoidsConfig {
    fn default() -> Self {
        Self {
            paused: false,
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
