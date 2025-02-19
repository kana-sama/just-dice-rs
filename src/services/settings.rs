use crate::prelude::*;

struct Data {
    draw_fps: bool,
    dark_theme: bool,
}

impl Data {
    fn decode(data: &[u8]) -> Self {
        let draw_fps = data.get(0).copied().unwrap_or(0) == 1;
        let dark_theme = data.get(1).copied().unwrap_or(1) == 1;
        Data { draw_fps, dark_theme }
    }

    fn encode(&self) -> Vec<u8> {
        vec![
            if self.draw_fps { 1 } else { 0 },
            if self.dark_theme { 1 } else { 0 },
        ]
    }
}

pub struct Settings {
    data: Data,
    dirty: bool,

    draw_fps_menu: CheckMenuItem,
    dark_theme_menu: CheckMenuItem,
}

impl Settings {
    pub fn load() -> Self {
        let data = pd::fs::read("settings.bin", true).unwrap_or_default();
        let data = Data::decode(&data);

        Self {
            draw_fps_menu: CheckMenuItem::new("draw fps", data.draw_fps, None, ()).unwrap(),
            dark_theme_menu: CheckMenuItem::new("dark theme", data.dark_theme, None, ()).unwrap(),

            data,
            dirty: false,
        }
    }

    pub fn update(&mut self) {
        if self.draw_fps_menu.is_checked() != self.draw_fps() {
            self.set_draw_fps(self.draw_fps_menu.is_checked());
        }

        if self.dark_theme_menu.is_checked() != self.dark_theme() {
            self.set_dark_theme(self.dark_theme_menu.is_checked());
        }

        if self.dirty {
            self.save();
            self.dirty = false;
        }
    }

    fn save(&self) {
        pd::fs::write("settings.bin", self.data.encode()).unwrap();
    }

    pub fn draw_fps(&self) -> bool {
        self.data.draw_fps
    }

    pub fn set_draw_fps(&mut self, value: bool) {
        self.data.draw_fps = value;
        self.dirty = true;
    }

    pub fn dark_theme(&self) -> bool {
        self.data.dark_theme
    }

    pub fn set_dark_theme(&mut self, value: bool) {
        self.data.dark_theme = value;
        self.dirty = true;
    }
}
