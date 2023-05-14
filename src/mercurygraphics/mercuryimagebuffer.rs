use im::{Pixel, Rgba};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

/**
 * Platform independent image buffer.
 * 
 * For the moment, the underlying representation is image::RgbaImage, but
 * this will change when we go nostd.
 */
pub struct MercuryImageBuffer {
    buffer: im::RgbaImage
}

impl MercuryImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: im::RgbaImage::new(width as u32, height as u32)
        }
    }

    /// load buffer from Image crate ImageBuffer
    fn from_lib_rgba_image(img: &im::RgbaImage) -> Self {
        Self {
            buffer: img.clone()
        }
    }

    /// return Image crate ImageBuffer representation
    fn to_lib_rgba_image(&self) -> im::RgbaImage {
        self.buffer.clone()
    }

    /// return a buffer compatible with Javascript ImageData class (little endian)
    pub fn to_wasm_compatible_buffer(&self) -> [u32; SCREEN_WIDTH * SCREEN_HEIGHT] {
        let mut wasm_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT] = [0; SCREEN_WIDTH * SCREEN_HEIGHT];

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let im_pixel = self.buffer.get_pixel(x as u32, y as u32);
                let channels = im_pixel.channels();

                // invert the bytes to little endian for Javascript ImageData class
                //wasm_buffer[y * SCREEN_WIDTH + x] = ((channels[3] << 24) | (channels[2] << 16) | (channels[1] << 8) | (channels[0])) as u32;
                let bytes: [u8; 4] = [channels[0], channels[1], channels[2], channels[3]];
                wasm_buffer[y * SCREEN_WIDTH + x] = u32::from_le_bytes(bytes);
            }
        }

        wasm_buffer
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        self.buffer.put_pixel(x, y, Rgba([r, g, b, a]));
    }
}

// todo
#[cfg(test)]
mod tests {
    use super::*;
}