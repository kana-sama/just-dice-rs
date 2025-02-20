use core::ops::{Add, Mul, Sub};

use crate::utils::math_traits::Norm;
use crate::utils::float_ext::FloatExt as _;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0., y: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.sqr() + self.y.sqr()).sqrt()
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f32) -> Vector2D {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Norm for Vector2D {
    fn norm(&self) -> f32 {
        self.magnitude()
    }
}
