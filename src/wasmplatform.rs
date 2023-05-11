use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::light_gfx_demo::GfxDemoGame;

#[no_mangle]
static mut BUFFER: [u32; SCREEN_WIDTH * SCREEN_HEIGHT] = [0; SCREEN_WIDTH * SCREEN_HEIGHT];

#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be called from
    // JavaScript. If you maintain that condition, then we know that the &mut
    // we're about to produce is unique, and therefore safe.
    let mut wasm_game: GfxDemoGame = GfxDemoGame::new();
    loop {
        render_frame_safe(&mut BUFFER, &mut wasm_game)
    }
}

// We split this out so that we can escape 'unsafe' as quickly as possible.
fn render_frame_safe(buffer: &mut [u32; SCREEN_WIDTH * SCREEN_HEIGHT], wasm_game: &mut GfxDemoGame) {
    wasm_game.update();
    *buffer = wasm_game.render();
}