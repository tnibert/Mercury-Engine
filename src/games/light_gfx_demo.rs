use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use core::sync::atomic::{AtomicU32, Ordering};

pub struct GfxDemoGame {
    buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    frame: AtomicU32
}

impl GfxDemoGame {
    pub fn new() -> Self {
        GfxDemoGame {
            buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            frame: AtomicU32::new(0)
        }
    }
 
    // below will become GameObject trait
    pub fn render(&self) -> [u32; SCREEN_WIDTH * SCREEN_HEIGHT] {
        return self.buffer.clone();
    }

    //fn position(&self) -> Option<Rect> {
        //return Some(Rect{x: 0.0, y: 0.0, w: SCREEN_WIDTH as f64, h: SCREEN_HEIGHT as f64});
    //}

    pub fn update(&mut self) {
        let f = self.frame.fetch_add(1, Ordering::Relaxed);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.buffer[y * SCREEN_WIDTH + x] = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
            }
        }
    }
}