pub mod observer;
pub mod sprite;
pub mod mercurygraphics;
pub mod player;
pub mod gameobject;
pub mod collision;
pub mod background;
pub mod platform;

#[cfg(target_family = "wasm")]
extern crate web_sys;
#[cfg(target_family = "wasm")]
pub mod games;
#[cfg(target_family = "wasm")]
use games::light_gfx_demo;

#[cfg(target_family = "unix")]
pub mod input;
#[cfg(target_family = "unix")]
extern crate piston;
#[cfg(target_family = "unix")]
extern crate graphics;
#[cfg(target_family = "unix")]
extern crate opengl_graphics;
#[cfg(target_family = "unix")]
extern crate sdl2_window;

extern crate image as im;

// todo: what types should these be?
pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 480;
pub const FRAME_RATE: u32 = 45;

