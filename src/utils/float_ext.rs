#![allow(unused)]

use core::ops::RangeInclusive;

pub trait FloatExt: Sized {
    fn sqr(self) -> Self;
    fn sqrt(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;

    fn rezoom_interval(self, from: RangeInclusive<Self>, to: RangeInclusive<Self>) -> Self;
}

impl FloatExt for f32 {
    #[inline(always)]
    fn sqr(self) -> Self {
        self * self
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        unsafe { core::intrinsics::sqrtf32(self) }
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        unsafe { core::intrinsics::ceilf32(self) }
    }

    #[inline(always)]
    fn floor(self) -> Self {
        unsafe { core::intrinsics::floorf32(self) }
    }

    fn rezoom_interval(self, from: RangeInclusive<Self>, to: RangeInclusive<Self>) -> Self {
        let progress = ((self - *from.start()) / (*from.end() - *from.start()))
            .max(0.0).min(1.0);

        to.start() + (to.end() - to.start()) * progress
    }
}
