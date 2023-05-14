use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::mercurygraphics::mercuryimagebuffer::MercuryImageBuffer;
use core::sync::atomic::{AtomicU32, Ordering};

pub struct GfxDemoGame {
    buffer: MercuryImageBuffer,
    frame: AtomicU32
}

impl GfxDemoGame {
    pub fn new() -> Self {
        GfxDemoGame {
            buffer: MercuryImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            frame: AtomicU32::new(0)
        }
    }
 
    // below will become GameObject trait
    pub fn render(&self) -> [u32; SCREEN_WIDTH * SCREEN_HEIGHT] {
        self.buffer.to_wasm_compatible_buffer()
    }

    //fn position(&self) -> Option<Rect> {
        //return Some(Rect{x: 0.0, y: 0.0, w: SCREEN_WIDTH as f64, h: SCREEN_HEIGHT as f64});
    //}

    pub fn update(&mut self) {
        let f = self.frame.fetch_add(1, Ordering::Relaxed);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
                let sep = u32_to_u8_array(pixel);
                self.buffer.set_pixel(x as u32, y as u32, sep[3], sep[2], sep[1], sep[0]);
            }
        }
    }
}

// todo: move somewhere more appropriate
fn u32_to_u8_array(n: u32) -> [u8; 4] {
    let b0 = (n >> 24) as u8;
    let b1 = (n >> 16) as u8;
    let b2 = (n >> 8) as u8;
    let b3 = n as u8;
    [b0, b1, b2, b3]
}