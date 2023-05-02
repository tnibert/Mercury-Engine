use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

use core::sync::atomic::{AtomicU32, Ordering};

#[no_mangle]
static mut BUFFER: [u32; SCREEN_WIDTH * SCREEN_HEIGHT] = [0; SCREEN_WIDTH * SCREEN_HEIGHT];

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be called from
    // JavaScript. If you maintain that condition, then we know that the &mut
    // we're about to produce is unique, and therefore safe.
    render_frame_safe(&mut BUFFER)
}

// We split this out so that we can escape 'unsafe' as quickly as possible.
// todo: need to provide frame from engine game loop
fn render_frame_safe(buffer: &mut [u32; SCREEN_WIDTH * SCREEN_HEIGHT]) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            buffer[y * SCREEN_WIDTH + x] = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
        }
    }
}