use crate::prelude::*;

const HISTORY_SIZE: usize = 10;

const REQUIRED_SHAKE_FORCE: f32 = 0.3;
const SHAKE_DEBOUNCE: i32 = 7;
const EXTREMUM_DEBOUNCE: i32 = 7;

pub struct Shaking {
    measures: [Vector3D; HISTORY_SIZE],
    measures_index: usize,
    measures_total: Vector3D,

    is_shaking_right_now: bool,
    is_shaking_right_now_debounce: i32,

    is_shaking_in_extremum: bool,
    is_shaking_in_extremum_debounce: i32,

    is_shaking_just_started: bool,
    is_shaking_just_stopped: bool,
}

impl Shaking {
    pub fn new() -> Self {
        pd::controls::accelerometer::enable();

        Self {
            measures: [Vector3D::zero(); HISTORY_SIZE],
            measures_index: 0,
            measures_total: Vector3D::zero(),

            is_shaking_right_now: false,
            is_shaking_right_now_debounce: SHAKE_DEBOUNCE,

            is_shaking_in_extremum: false,
            is_shaking_in_extremum_debounce: EXTREMUM_DEBOUNCE,

            is_shaking_just_started: false,
            is_shaking_just_stopped: false,
        }
    }

    pub fn update(&mut self) {
        let measure = Vector3D::from(pd::controls::accelerometer::get());

        self.measures_index = (self.measures_index + 1) % HISTORY_SIZE;
        self.measures_total -= self.measures[self.measures_index];
        self.measures[self.measures_index] = measure;
        self.measures_total += self.measures[self.measures_index];

        let average_len = self.measures_total.magnitude() / HISTORY_SIZE as f32;

        let average_delta_len = self.measures.iter()
            .map(|m| m.magnitude() - average_len)
            .sum::<f32>()
            .div(HISTORY_SIZE as f32);

        let is_required_force = average_delta_len > REQUIRED_SHAKE_FORCE;

        if is_required_force != self.is_shaking_right_now {
            self.is_shaking_right_now_debounce -= 1;
        } else {
            self.is_shaking_right_now_debounce = SHAKE_DEBOUNCE;
        }

        let was_shaking = self.is_shaking_right_now;

        if self.is_shaking_right_now_debounce <= 0 {
            self.is_shaking_right_now = is_required_force;
            self.is_shaking_right_now_debounce = SHAKE_DEBOUNCE;
        }

        self.is_shaking_just_started = !was_shaking && self.is_shaking_right_now;
        self.is_shaking_just_stopped = was_shaking && !self.is_shaking_right_now;

        self.is_shaking_in_extremum = true
            && self.is_shaking_in_extremum_debounce <= 0
            && (self.last_measure() - self.prev_measure()).magnitude() > 0.5;

        if self.is_shaking_in_extremum {
            self.is_shaking_in_extremum_debounce = EXTREMUM_DEBOUNCE;
        } else {
            self.is_shaking_in_extremum_debounce -= 1;
        }
    }

    pub fn just_started(&self) -> bool {
        self.is_shaking_just_started
    }

    pub fn right_now(&self) -> bool {
        self.is_shaking_right_now
    }

    pub fn just_stopped(&self) -> bool {
        self.is_shaking_just_stopped
    }

    pub fn in_extremum(&self) -> bool {
        self.is_shaking_in_extremum
    }

    fn prev_measure(&self) -> Vector3D {
        let prev_index = (self.measures_index + HISTORY_SIZE - 1) % HISTORY_SIZE;
        self.measures[prev_index]
    }

    fn last_measure(&self) -> Vector3D {
        self.measures[self.measures_index]
    }
}
