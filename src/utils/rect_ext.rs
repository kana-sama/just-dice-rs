use pd::sys::ffi::PDRect;

use crate::utils::vector_2d::Vector2D;

pub trait PDRectExt {
    fn set_center(&mut self, center: Vector2D);
}

impl PDRectExt for PDRect {
    fn set_center(&mut self, center: Vector2D) {
        self.x = center.x - self.width / 2.;
        self.y = center.y - self.height / 2.;
    }
}
