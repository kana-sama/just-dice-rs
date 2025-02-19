use core::mem::MaybeUninit;
use crate::utils::vector_2d::Vector2D;

pub struct BytesDecoder<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> BytesDecoder<'a> {
    const fn take_byte(&mut self) -> u8 {
        let byte = self.data[self.index];
        self.index += 1;
        byte
    }

    const fn require_eof(&self) {
        if self.index != self.data.len() {
            panic!("unexpected data left");
        }
    }

    pub const fn take<T: ~const BytesDecode>(&mut self) -> T {
        T::decode(self)
    }

    pub const fn decode<T: ~const BytesDecode>(data: &'a [u8]) -> T {
        let mut decoder = Self { data, index: 0 };
        let value = T::decode(&mut decoder);
        decoder.require_eof();
        value
    }
}

#[const_trait]
pub trait BytesDecode {
    fn decode(decoder: &mut BytesDecoder) -> Self;
}

impl const BytesDecode for u8 {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        decoder.take_byte()
    }
}

impl const BytesDecode for u16 {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        u16::from_ne_bytes(decoder.take())
    }
}

// kek
impl const BytesDecode for f32 {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        decoder.take::<u16>() as f32
    }
}

impl const BytesDecode for Vector2D {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        Vector2D::new(
            decoder.take(),
            decoder.take(),
        )
    }
}

impl<T: ~const BytesDecode, const N: usize> const BytesDecode for [T; N] {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        let mut array = MaybeUninit::uninit_array();

        let mut i = 0;
        while i < N {
            array[i] = MaybeUninit::new(decoder.take());
            i += 1;
        }

        unsafe { MaybeUninit::array_assume_init(array) }
    }
}
