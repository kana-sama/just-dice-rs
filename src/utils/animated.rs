#![allow(unused)]

use core::ops::{Add, Mul, Sub};

use super::easing::{self, EasingFunction};

enum State {
    NotStarted,
    Finished,
    InProgress { start: u32 },
}

pub enum Looping {
    None,
    Loop,
    LoopAndReverse,
}

pub struct Animated<T> {
    easing_fn: EasingFunction,
    duration: u32,

    state: State,

    start_value: T,
    end_value: T,

    looping: Looping,
}

pub trait Animatable = Copy + Sub<Output = Self> + Add<Output = Self> + Mul<f32, Output = Self>;

impl<T: Default + Animatable>  Default for Animated<T> {
    fn default() -> Self {
        Animated::new(easing::LINEAR, 0, T::default(), T::default())
    }
}

impl<T: Animatable> Animated<T> {
    pub fn new(easing_fn: EasingFunction, duration: u32, start_value: T, end_value: T) -> Self {
        Self {
            easing_fn,
            duration,
            state: State::NotStarted,
            start_value,
            end_value,
            looping: Looping::None,
        }
    }

    pub fn get_start_value(&self) -> T {
        self.start_value
    }

    pub fn set_start_value(&mut self, start_value: T) {
        self.start_value = start_value;
    }

    pub fn get_end_value(&self) -> T {
        self.end_value
    }

    pub fn set_end_value(&mut self, end_value: T) {
        self.end_value = end_value;
    }

    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    pub fn set_looping(&mut self, looping: Looping) {
        self.looping = looping;
    }

    pub fn invert(&mut self) {
        (self.start_value, self.end_value) = (self.end_value, self.start_value);
    }

    pub fn start(&mut self) {
        let now = pd::system::current_time_ms();
        self.state = State::InProgress { start: now };
    }

    pub fn finish(&mut self) {
        self.state = State::Finished;
    }

    pub fn progress(&mut self) -> f32 {
        match self.state {
            State::NotStarted => 0.,
            State::Finished => 1.,
            State::InProgress { start, .. } => {
                let now = pd::system::current_time_ms();
                let mut elapsed = now - start;

                if elapsed > self.duration {
                    match self.looping {
                        Looping::None => {
                            self.state = State::Finished;
                            return 1.;
                        }
                        Looping::Loop => {
                            elapsed %= self.duration;
                        }
                        Looping::LoopAndReverse => {
                            elapsed %= 2 * self.duration;
                            if elapsed > self.duration {
                                elapsed = 2 * self.duration - elapsed;
                            }
                        }
                    }
                }

                return elapsed as f32 / self.duration as f32;
            }
        }
    }

    pub fn value(&mut self) -> T {
        match self.state {
            State::NotStarted => self.start_value,
            State::Finished => self.end_value,
            State::InProgress { start } => {
                let x = self.progress();
                let y = (self.easing_fn)(x);
                self.start_value + (self.end_value - self.start_value) * y
            },
        }
    }

    pub fn remaining_duration(&self) -> u32 {
        match self.state {
            State::NotStarted => self.duration,
            State::Finished => 0,
            State::InProgress { start } => {
                let now = pd::system::current_time_ms();
                let elapsed = now - start;
                if elapsed > self.duration {
                    return 0;
                } else {
                    return self.duration - elapsed;
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.looping {
            Looping::None => match self.state {
                State::NotStarted => false,
                State::Finished => true,
                State::InProgress { start } => pd::system::current_time_ms() >= start + self.duration,
            },
            Looping::Loop => false,
            Looping::LoopAndReverse => false,
        }
    }
}
