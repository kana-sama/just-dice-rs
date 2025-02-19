pub use alloc::vec::Vec;
pub use alloc::string::ToString;
pub use alloc::rc::Rc;

pub use core::ops::*;
pub use core::cell::RefCell;

pub use itertools::Itertools as _;

pub use pd::controls::buttons::PDButtonsExt as _;
pub use pd::graphics::bitmap::table::BitmapTable;
pub use pd::graphics::bitmap::Bitmap;
pub use pd::graphics::bitmap::{BitmapFlip, BitmapFlipExt as _};
pub use pd::graphics::bitmap::{BitmapDrawMode, BitmapDrawModeExt as _};
pub use pd::graphics::color::{Color, LCDColorConst as _};
pub use pd::graphics::text::Font;
pub use pd::graphics::text::{TextAlignment, TextAlignmentExt as _};
pub use pd::graphics::text::{TextWrappingMode, TextWrappingModeExt as _};
pub use pd::sprite::Sprite;
pub use pd::system::menu::CheckMenuItem;

pub use pd::sys::ffi::LCDSolidColor;
pub use pd::sys::ffi::PDRect;

pub use crate::utils::easing;

pub use crate::utils::animated::{Animated, Looping};
pub use crate::utils::smooth::Smooth;
pub use crate::utils::cached::Cached;
pub use crate::utils::toggle::Toggle;

pub use crate::utils::vector_2d::Vector2D;
pub use crate::utils::vector_3d::Vector3D;

pub use crate::utils::bitmap_ext::BitmapExt as _;
pub use crate::utils::float_ext::FloatExt as _;
pub use crate::utils::rect_ext::PDRectExt as _;

pub use crate::utils::bytes_decoder::{BytesDecode, BytesDecoder};

#[repr(i16)]
pub enum ZIndex {
    DieShadow,
    Die,
    Cursor,
    Fade,
    Lock,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Theme {
    pub foreground: LCDSolidColor,
    pub background: LCDSolidColor,
}

pub type ThemeRef = Rc<RefCell<Theme>>;

impl Theme {
    pub fn image_draw_mode(&self) -> BitmapDrawMode {
        match self.foreground {
            LCDSolidColor::WHITE => BitmapDrawMode::Copy,
            LCDSolidColor::BLACK => BitmapDrawMode::Inverted,
            _ => BitmapDrawMode::Copy,
        }
    }

    pub fn text_draw_mode(&self) -> BitmapDrawMode {
        match self.foreground {
            LCDSolidColor::WHITE => BitmapDrawMode::FillWhite,
            LCDSolidColor::BLACK => BitmapDrawMode::FillBlack,
            _ => BitmapDrawMode::Copy,
        }
    }
}
