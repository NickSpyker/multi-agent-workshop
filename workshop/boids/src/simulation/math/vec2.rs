use std::{
    f32::consts::TAU,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let len: f32 = self.length();

        if len > 0.0 { self / len } else { Self::ZERO }
    }

    #[inline]
    pub fn clamp_length(self, max_length: f32) -> Self {
        let len_sq: f32 = self.length_squared();

        if len_sq > max_length * max_length {
            self.normalized() * max_length
        } else {
            self
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    #[inline]
    pub fn random_direction(rng: &mut fastrand::Rng) -> Self {
        let angle: f32 = rng.f32() * TAU;

        Self::from_angle(angle)
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}
