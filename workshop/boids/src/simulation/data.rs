use super::math::Vec2;
use fastrand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Boid {
    #[inline]
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    #[inline]
    pub fn random(rng: &mut Rng, width: f32, height: f32, max_speed: f32) -> Self {
        let position = Vec2::new(rng.f32() * width, rng.f32() * height);
        let velocity = Vec2::random_direction(rng) * (rng.f32() * max_speed);

        Self::new(position, velocity)
    }
}

#[derive(Debug, Clone)]
pub struct Boids {
    pub boids: Vec<Boid>,
    pub width: f32,
    pub height: f32,
}

impl Default for Boids {
    fn default() -> Self {
        Self {
            boids: Vec::new(),
            width: 800.0,
            height: 600.0,
        }
    }
}

impl Boids {
    pub fn spawn_random(&mut self, count: usize, max_speed: f32) {
        let mut rng = Rng::new();

        for _ in 0..count {
            self.boids
                .push(Boid::random(&mut rng, self.width, self.height, max_speed));
        }
    }

    pub fn clear(&mut self) {
        self.boids.clear();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_count(&mut self, target: usize, max_speed: f32) {
        let current: usize = self.boids.len();

        match current.cmp(&target) {
            Ordering::Less => self.spawn_random(target - current, max_speed),
            Ordering::Greater => self.boids.truncate(target),
            Ordering::Equal => {}
        }
    }
}
