pub mod observer;
pub mod sprite;
pub mod imgload;
pub mod player;
pub mod gameobject;
pub mod collision;
pub mod background;
pub mod platform;
pub mod game;

//#[cfg(target_family = "wasm")]
//pub mod wasmplatform;

#[cfg(target_family = "unix")]
pub mod input;
#[cfg(target_family = "unix")]
pub mod desktopplatform;
#[cfg(target_family = "unix")]
extern crate piston;
#[cfg(target_family = "unix")]
extern crate graphics;
#[cfg(target_family = "unix")]
extern crate opengl_graphics;
#[cfg(target_family = "unix")]
extern crate sdl2_window;

extern crate image as im;

/*
 * The following is what runs in wasm
 * todo: encapsulate into another "game"
 */

use core::sync::atomic::{AtomicU32, Ordering};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

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
fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT]) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
        }
    }
}