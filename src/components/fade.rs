use crate::prelude::*;

use crate::utils::easing;
use crate::utils::animated::Animated;

const FADE_DURATION: u32 = 200;

pub struct Fade {
    progress: Animated<f32>,
    prev_progress: f32,

    image: Bitmap,
    sprite: Sprite,
    theme: ThemeRef,
}

impl Fade {
    pub fn new(theme: ThemeRef) -> Self {
        Self {
            progress: Animated::new(easing::LINEAR, FADE_DURATION, 0.0, 1.0),
            prev_progress: 0.0,

            image: Bitmap::empty(),
            sprite: {
                let sprite = Sprite::new();
                sprite.move_to(pd::display::width() as f32 / 2.0, pd::display::height() as f32 / 2.0);
                sprite.set_z_index(ZIndex::Fade as _);
                sprite.add();
                sprite.set_draw_mode(theme.borrow().image_draw_mode());
                sprite
            },
            theme,
        }
    }

    pub fn update(&mut self) {
        let progress = self.progress.value();
        let changed = progress != self.prev_progress;
        self.prev_progress = progress;

        if changed {
            self.image = Bitmap::new(pd::display::width(), pd::display::width(), Color::CLEAR).unwrap().render(|_| {
                let diameter = pd::display::width() as f32 * progress * 1.5;
                let offset = (pd::display::width() as f32 - diameter) / 2.0;

                pd::graphics::fill_ellipse(
                    offset as i32, offset as i32,
                    diameter as i32, diameter as i32,
                    0.0, 0.0,
                    self.theme.borrow().foreground as _,
                );
            });

            self.sprite.set_image(&self.image, BitmapFlip::Unflipped);
        }

    }

    pub fn enter(&mut self) {
        self.progress = Animated::new(easing::LINEAR, 200, 0.0, 1.0);
        self.progress.start();
    }

    pub fn leave(&mut self) {
        self.progress = Animated::new(easing::LINEAR, FADE_DURATION - self.progress.remaining_duration(), self.progress.value(), 0.0);
        self.progress.start();
    }
}
