#[cfg(target_family = "wasm")]
use {
    crate::collision::Rect,
    crate::{SCREEN_WIDTH, SCREEN_HEIGHT},
    crate::mercurygraphics::mercuryimagebuffer::MercuryImageBuffer,
    crate::gameobject::GameObject,
};

#[cfg(target_family = "unix")]
use {
    mercurylib::collision::Rect,
    mercurylib::{SCREEN_WIDTH, SCREEN_HEIGHT},
    mercurylib::mercurygraphics::mercuryimagebuffer::MercuryImageBuffer,
    mercurylib::gameobject::GameObject,
    mercurylib::desktopplatform::DesktopPlatform as CurrentPlatform
};

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
}

impl GameObject for GfxDemoGame {
    fn render(&self) -> Option<MercuryImageBuffer> {
        Some(self.buffer.clone())
    }

    fn position(&self) -> Option<Rect> {
        return Some(Rect{x: 0.0, y: 0.0, w: SCREEN_WIDTH as f64, h: SCREEN_HEIGHT as f64});
    }

    fn update(&mut self) {
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

#[cfg(target_family = "unix")]
fn main() {
    use mercurylib::platform::Platform;

    let mut game = GfxDemoGame::new();
    let mut pf = CurrentPlatform::new();
    pf.gameloop(&mut game);
}