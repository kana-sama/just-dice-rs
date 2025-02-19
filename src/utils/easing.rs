#![allow(unused)]

use crate::utils::float_ext::FloatExt as _;

pub type EasingFunction = fn(x: f32) -> f32;

pub static LINEAR: EasingFunction = |x| x;

pub static OUT_CIRC: EasingFunction = |x| {
    (1. - (x - 1.).sqr()).sqrt()
};

pub static IN_OUT_CIRC: EasingFunction = |x| {
    if x < 0.5 {
        (1. - (1. - (2. * x).sqr()).sqrt()) / 2.
    } else {
        ((1. - (-2. * x + 2.).sqr()).sqrt() + 1.) / 2.
    }
};

pub static IN_OUT_QUAD: EasingFunction = |x| {
    if x < 0.5 {
        2. * x.sqr()
    } else {
        1. - (-2. * x + 2.).sqr() / 2.
    }
};
