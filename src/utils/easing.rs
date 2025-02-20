#![allow(unused)]

use crate::utils::float_ext::FloatExt as _;

pub type EasingFunction = fn(x: f32) -> f32;

pub static LINEAR: EasingFunction = |x| x;

pub static OUT_CIRC: EasingFunction = |x| {
    (1.0 - (x - 1.0).sqr()).sqrt()
};

pub static IN_OUT_CIRC: EasingFunction = |x| {
    if x < 0.5 {
        (1.0 - (1.0 - (2.0 * x).sqr()).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * x + 2.0).sqr()).sqrt() + 1.0) / 2.0
    }
};

pub static IN_OUT_QUAD: EasingFunction = |x| {
    if x < 0.5 {
        2.0 * x.sqr()
    } else {
        1.0 - (-2.0 * x + 2.0).sqr() / 2.
    }
};
