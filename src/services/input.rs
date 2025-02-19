use pd::sys::ffi::PDButtons;

pub struct Input {
    pub pressed: PDButtons,
}

impl Input {
    pub fn new() -> Self {
        Self { pressed: PDButtons(0) }
    }

    pub fn update(&mut self) {
        self.pressed = pd::controls::buttons::pushed();
    }
}
