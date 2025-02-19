use pd::graphics::{bitmap::Bitmap, color::Color};

pub trait BitmapExt {
    fn render(self, draw: impl Fn(&Bitmap)) -> Self;

    fn empty() -> Self;
}

impl BitmapExt for Bitmap {
    #[inline(always)]
    fn render(self, draw: impl Fn(&Bitmap)) -> Self {
        pd::graphics::push_context(&self);
        draw(&self);
        pd::graphics::pop_context();
        return self;
    }

    #[inline(always)]
    fn empty() -> Self {
        Self::new(0, 0, Color::CLEAR).unwrap()
    }
}
