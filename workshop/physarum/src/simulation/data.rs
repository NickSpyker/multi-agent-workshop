use fastrand::Rng;
use std::cmp::Ordering;

/// A single slime agent with position and direction
#[derive(Debug, Clone)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub angle: f32, // Direction in radians
}

impl Agent {
    #[inline]
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }

    pub fn random(rng: &mut Rng, width: usize, height: usize) -> Self {
        let x = rng.f32() * width as f32;
        let y = rng.f32() * height as f32;
        let angle = rng.f32() * std::f32::consts::TAU;
        Self::new(x, y, angle)
    }

    pub fn random_in_circle(rng: &mut Rng, center_x: f32, center_y: f32, radius: f32) -> Self {
        // Random point in circle using polar coordinates
        let r = radius * rng.f32().sqrt();
        let theta = rng.f32() * std::f32::consts::TAU;
        let x = center_x + r * theta.cos();
        let y = center_y + r * theta.sin();
        // Point outward from center
        let angle = (y - center_y).atan2(x - center_x);
        Self::new(x, y, angle)
    }
}

/// The trail map stores pheromone concentrations
#[derive(Debug, Clone)]
pub struct TrailMap {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl TrailMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0.0; width * height],
            width,
            height,
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0.0
        }
    }

    #[inline]
    pub fn add(&mut self, x: usize, y: usize, value: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.data[idx] = (self.data[idx] + value).min(1.0);
        }
    }

    /// Sample the trail map at a position with sensor size (average of surrounding pixels)
    pub fn sample(&self, x: f32, y: f32, sensor_size: i32) -> f32 {
        let cx = x as i32;
        let cy = y as i32;
        let mut sum = 0.0;
        let mut count = 0;

        for dy in -sensor_size..=sensor_size {
            for dx in -sensor_size..=sensor_size {
                let sx = cx + dx;
                let sy = cy + dy;

                if sx >= 0 && sx < self.width as i32 && sy >= 0 && sy < self.height as i32 {
                    #[allow(clippy::cast_sign_loss)]
                    {
                        sum += self.get(sx as usize, sy as usize);
                    }
                    count += 1;
                }
            }
        }

        if count > 0 {
            sum / count as f32
        } else {
            0.0
        }
    }

    /// Apply diffusion (box blur) and decay to the trail map
    pub fn diffuse_and_decay(&mut self, diffuse_rate: f32, decay_rate: f32, dt: f32) {
        let width = self.width;
        let height = self.height;

        // Create a copy for reading while we write
        let original = self.data.clone();

        for y in 0..height {
            for x in 0..width {
                // 3x3 box blur
                let mut sum = 0.0;
                let mut count = 0;

                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            #[allow(clippy::cast_sign_loss)]
                            {
                                sum += original[ny as usize * width + nx as usize];
                            }
                            count += 1;
                        }
                    }
                }

                let blurred = sum / count as f32;
                let original_val = original[y * width + x];

                // Lerp between original and blurred based on diffuse rate
                let diffused = original_val + (blurred - original_val) * diffuse_rate * dt;

                // Apply decay
                let decayed = diffused - decay_rate * dt;

                self.data[y * width + x] = decayed.max(0.0);
            }
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(0.0);
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        self.width = new_width;
        self.height = new_height;
        self.data = vec![0.0; new_width * new_height];
    }
}

impl Default for TrailMap {
    fn default() -> Self {
        Self::new(800, 600)
    }
}

/// Spawn mode for agents
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SpawnMode {
    #[default]
    Random,
    Center,
    Circle,
}

/// The complete Physarum simulation state
#[derive(Debug, Clone)]
pub struct Physarum {
    pub agents: Vec<Agent>,
    pub trail_map: TrailMap,
    pub width: usize,
    pub height: usize,
}

impl Default for Physarum {
    fn default() -> Self {
        Self {
            agents: Vec::new(),
            trail_map: TrailMap::default(),
            width: 800,
            height: 600,
        }
    }
}

impl Physarum {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            agents: Vec::new(),
            trail_map: TrailMap::new(width, height),
            width,
            height,
        }
    }

    pub fn spawn_agents(&mut self, count: usize, mode: SpawnMode) {
        let mut rng = Rng::new();
        let center_x = self.width as f32 / 2.0;
        let center_y = self.height as f32 / 2.0;
        let radius = self.width.min(self.height) as f32 * 0.4;

        for _ in 0..count {
            let agent = match mode {
                SpawnMode::Random => Agent::random(&mut rng, self.width, self.height),
                SpawnMode::Center => {
                    let angle = rng.f32() * std::f32::consts::TAU;
                    Agent::new(center_x, center_y, angle)
                }
                SpawnMode::Circle => Agent::random_in_circle(&mut rng, center_x, center_y, radius),
            };
            self.agents.push(agent);
        }
    }

    pub fn clear(&mut self) {
        self.agents.clear();
        self.trail_map.clear();
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.trail_map.resize(width, height);
    }

    pub fn set_agent_count(&mut self, target: usize, mode: SpawnMode) {
        let current = self.agents.len();

        match current.cmp(&target) {
            Ordering::Less => self.spawn_agents(target - current, mode),
            Ordering::Greater => self.agents.truncate(target),
            Ordering::Equal => {}
        }
    }
}
