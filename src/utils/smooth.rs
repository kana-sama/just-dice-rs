use core::ops::{Add, Mul, Sub};
use crate::utils::math_traits::Norm;

pub struct Smooth<T> {
    current_value: T,
    target_value: T,
}

pub trait Smoothable = Copy + Norm + Sub<Output = Self> + Add<Output = Self> + Mul<f32, Output = Self>;

impl<T: Smoothable> Smooth<T> {
    pub fn new(value: T) -> Self {
        Self {
            current_value: value,
            target_value: value,
        }
    }

    pub fn get(&self) -> T {
        self.current_value
    }

    pub fn set(&mut self, value: T) {
        self.target_value = value;
    }

    pub fn set_immediately(&mut self, value: T) {
        self.current_value = value;
        self.target_value = value;
    }

    pub fn update(&mut self) {
        let diff = self.target_value - self.current_value;

        if diff.norm() < 1. {
            self.current_value = self.target_value;
        } else {
            self.current_value = self.current_value + diff * 0.1;
        }
    }
}
