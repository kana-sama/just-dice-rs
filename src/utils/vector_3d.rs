#![allow(unused)]

use core::ops::{Add, AddAssign, Div, Sub, SubAssign};
use crate::utils::float_ext::FloatExt as _;

#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0., z: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl From<(f32, f32, f32)> for Vector3D {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}


fn f() -> Result<(), ()> { Ok(()) }
type X = Result<(), ()>;
fn g() -> X { Ok(()) }

fn h() {
    f();
    g();
}
