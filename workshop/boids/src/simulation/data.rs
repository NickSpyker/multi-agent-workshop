use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[must_use]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[must_use]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    #[must_use]
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self / len
        } else {
            Self::ZERO
        }
    }

    #[must_use]
    pub fn clamp_length(self, max_length: f32) -> Self {
        let len_sq = self.length_squared();
        if len_sq > max_length * max_length {
            self.normalized() * max_length
        } else {
            self
        }
    }

    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[must_use]
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    #[must_use]
    pub fn random_direction(rng: &mut fastrand::Rng) -> Self {
        let angle = rng.f32() * std::f32::consts::TAU;
        Self::from_angle(angle)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

#[derive(Debug, Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Boid {
    #[must_use]
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    #[must_use]
    pub fn random(rng: &mut fastrand::Rng, width: f32, height: f32, max_speed: f32) -> Self {
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
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            boids: Vec::new(),
            width,
            height,
        }
    }

    pub fn spawn_random(&mut self, count: usize, max_speed: f32) {
        let mut rng = fastrand::Rng::new();
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

    #[must_use]
    pub fn count(&self) -> usize {
        self.boids.len()
    }
}
