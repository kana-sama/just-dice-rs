use crate::prelude::*;

const CURSOR_HEIGHT: i32 = 24;
const CURSOR_OFFSET: i32 = 7;

pub struct Cursor {
    position: Smooth<Vector2D>,
    sprite: Sprite,
    floating: Animated<f32>,
    theme: ThemeRef,
}

impl Cursor {
    pub fn new(theme: ThemeRef) -> Self {
        let position = Smooth::new(Vector2D::zero());

        let image: &_ = unsafe {
            static mut IMAGE: Option<Bitmap> = None;
            IMAGE.get_or_insert_with(|| Bitmap::load("assets/images/cursor").unwrap())
        };

        let sprite = Sprite::new();
        sprite.set_image(&image, BitmapFlip::Unflipped);
        sprite.set_z_index(ZIndex::Cursor as _);

        let mut floating = Animated::new(
            easing::IN_OUT_CIRC, 700,
            -CURSOR_OFFSET as f32 / 2.,
            CURSOR_OFFSET as f32 / 2.,
        );

        floating.set_looping(Looping::LoopAndReverse);
        floating.start();

        Self { position, sprite, floating, theme }
    }

    pub fn update(&mut self) {
        if self.sprite.is_visible() {
            self.position.update();
            self.sprite.move_to(
                self.position.get().x.floor(),
                (self.position.get().y + self.floating.value()).floor(),
            );
            self.sprite.set_draw_mode(self.theme.borrow().image_draw_mode());
        }
    }

    pub fn show(&mut self, target: PDRect) {
        let (position, flip) = Self::position_for_target(target);

        self.position = Smooth::new(position);
        self.sprite.set_image_flip(flip);

        self.sprite.add();
    }

    pub fn hide(&mut self) {
        self.sprite.remove();
    }

    pub fn move_to(&mut self, target: PDRect) {
        let (position, flip) = Self::position_for_target(target);
        self.position.set(position);
        self.sprite.set_image_flip(flip);
    }

    pub fn immediately_move_to(&mut self, target: PDRect) {
        let (position, flip) = Self::position_for_target(target);
        self.position.set_immediately(position);
        self.sprite.set_image_flip(flip);
    }

    fn position_for_target(target: PDRect) -> (Vector2D, BitmapFlip) {
        let mut flip = BitmapFlip::Unflipped;
        let mut position = Vector2D::new(
            target.x + target.width / 2.,
            target.y - CURSOR_HEIGHT as f32 / 2. - CURSOR_OFFSET as f32,
        );

        if position.y < CURSOR_HEIGHT as f32 {
            flip = BitmapFlip::FlippedXY;
            position.y = target.y + target.height + CURSOR_HEIGHT as f32 / 2. + CURSOR_OFFSET as f32;
        }

        return (position, flip);
    }
}
