use alloc::vec::Vec;

use crate::prelude::{BytesDecode, BytesDecoder};

struct BitImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    alpha: Vec<u8>,
}

impl BytesDecode for BitImage {
    fn decode(decoder: &mut BytesDecoder) -> Self {

    }
}

impl BitImage {
    fn from_image(image: DynamicImage) -> Self {
        let row_len = (image.width() as f32 / 32.0).ceil() as usize * 4;

        let mut bit_image = BitImage {
            width: image.width(),
            height: image.height(),
            data: vec![0; row_len * image.height() as usize],
            alpha: vec![0; row_len * image.height() as usize],
        };

        for (x, y, color) in image.pixels() {
            let index = y as usize * row_len + x as usize / 8;
            let bit = 7 - x as u8 % 8;

            bit_image.data[index] |= ((color[0] > 128) as u8) << bit;
            bit_image.alpha[index] |= ((color[3] > 128) as u8) << bit;
        }

        bit_image
    }

    fn to_image(&self) -> DynamicImage {
        let mut image = DynamicImage::new(self.width, self.height, ColorType::Rgba8);

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * (self.width as f32 / 32.0).ceil() as usize * 4 + x as usize / 8;
                let bit = 7 - x as u8 % 8;

                let color = if self.alpha[index] & (1 << bit) != 0 {
                    if self.data[index] & (1 << bit) != 0 {
                        [255, 255, 255, 255]
                    } else {
                        [0, 0, 0, 255]
                    }
                } else {
                    [0, 0, 0, 0]
                };

                image.put_pixel(x, y, image::Rgba(color));
            }
        }

        image
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.width.to_le_bytes());
        bytes.extend_from_slice(&self.height.to_le_bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.alpha);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let width = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let height = u32::from_le_bytes(bytes[4..8].try_into().unwrap());

        let bytes = &bytes[8..];

        let chunk_size = bytes.len() / 2;

        let data = bytes[0 .. chunk_size].to_vec();
        let alpha = bytes[chunk_size ..].to_vec();

        BitImage { width, height, data, alpha }
    }

    fn to_compressed_bytes(&self) -> Vec<u8> {
        deflate::compress_to_vec_zlib(&self.to_bytes(), 6)
    }

    fn from_compressed_bytes(bytes: &[u8]) -> Self {
        Self::from_bytes(&inflate::decompress_to_vec_zlib(bytes).unwrap())
    }
}
