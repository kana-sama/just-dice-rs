use crate::prelude::*;

pub struct ThemeManager {
    dark_theme: bool,
    theme: ThemeRef,
}

const DARK_THEME: Theme = Theme {
    background: LCDSolidColor::BLACK,
    foreground: LCDSolidColor::WHITE,
};

const LIGHT_THEME: Theme = Theme {
    background: LCDSolidColor::WHITE,
    foreground: LCDSolidColor::BLACK,
};

impl ThemeManager {
    pub fn new(dark_theme: bool) -> Self {
        Self {
            dark_theme,
            theme: Rc::new(RefCell::new(ThemeManager::lookup_theme(dark_theme))),
        }
    }

    fn lookup_theme(dark_theme: bool) -> Theme {
        if dark_theme {
            DARK_THEME
        } else {
            LIGHT_THEME
        }
    }

    pub fn theme(&self) -> ThemeRef {
        ThemeRef::clone(&self.theme)
    }

    pub fn set_dark_theme(&mut self, value: bool) {
        if self.dark_theme != value {
            self.dark_theme = value;
            self.theme.replace(ThemeManager::lookup_theme(value));

            pd::graphics::set_background_color(self.theme.borrow().background);
            pd::graphics::clear(Color::CLEAR);
        }
    }
}
