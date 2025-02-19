use crate::prelude::*;

pub struct Lock {
    theme: ThemeRef,
    locked: Toggle,
    bitmap: Cached<(i32, BitmapDrawMode), Bitmap>,
    sprite: Sprite,
}

impl Lock {
    pub fn new(locked: bool, theme: ThemeRef) -> Self {
        Self {
            theme,
            locked: Toggle::new(15, locked),
            bitmap: Cached::new(),
            sprite: {
                let sprite = Sprite::new();
                sprite.set_z_index(ZIndex::Lock as _);
                sprite.add();
                sprite
            }
        }
    }

    pub fn set(&mut self, locked: bool) {
        self.locked.set(locked);
    }

    pub fn update(&mut self) {
        self.locked.update();

        let lock_offset = self.locked.progress().rezoom_interval(0. ..= 0.6, 0. ..= -50.).floor();
        let shank_offset = self.locked.progress().rezoom_interval(0.7 ..= 1., -3. ..= 0.).floor() as i32;

        let (ref body, ref shank) = unsafe {
            static mut BODY: Option<(Bitmap, Bitmap)> = None;
            BODY.get_or_insert_with(|| (
                Bitmap::load("assets/images/lock-body").unwrap(),
                Bitmap::load("assets/images/lock-shank").unwrap()
            ))
        };

        let bitmap = self.bitmap.get((shank_offset, self.theme.borrow().image_draw_mode()), |(shank_offset, draw_mode)| {
            Bitmap::new(24, 34, Color::CLEAR).unwrap().render(|_| {
                let _ = pd::graphics::set_draw_mode(*draw_mode);
                shank.draw(0, *shank_offset, BitmapFlip::Unflipped);
                body.draw(0, 0, BitmapFlip::Unflipped);
            })
        });

        self.sprite.set_image(bitmap, BitmapFlip::Unflipped);
        self.sprite.move_to(playdate::display::width() as f32 + lock_offset + 25., playdate::display::height() as f32 - 25.);
    }
}
