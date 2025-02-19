#![allow(unused)]

pub struct Toggle {
    value: bool,
    progress: u32,
    duration: u32,
    changed: bool,
}

impl Toggle {
    pub fn new(duration: u32, value: bool) -> Self {
        Self {
            value,
            progress: if value { duration } else { 0 },
            duration,
            changed: false,
        }
    }

    pub fn update(&mut self) {
        self.changed = false;

        if self.value {
            if self.progress < self.duration - 1 {
                self.progress += 1;
                self.changed = true;
            }
        } else {
            if self.progress > 0 {
                self.progress -= 1;
                self.changed = true;
            }
        }
    }

    pub fn toggle(&mut self) {
        self.value = !self.value;
    }

    pub fn on(&mut self) {
        self.value = true;
    }

    pub fn off(&mut self) {
        self.value = false;
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    pub fn get(&self) -> bool {
        self.value
    }

    pub fn progress(&self) -> f32 {
        self.progress as f32 / self.duration as f32
    }
}
