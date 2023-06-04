extern crate mercurylib;
extern crate image as im;

use mercurylib::background::Background;
use mercurylib::player::Player;
//use crate::input::Input;
use mercurylib::gameobject::GameObject;
use mercurylib::collision::Rect;
use mercurylib::mercurygraphics::mercuryimagebuffer::MercuryImageBuffer;
use mercurylib::platform::desktopplatform::run_desktop_platform;
use mercurylib::{SCREEN_WIDTH, SCREEN_HEIGHT};
use std::time::Instant;

pub struct TownDemoGame {
    //pub input: Input,
    gameobjects: Vec<Box<dyn GameObject>>
}

impl TownDemoGame {
    pub fn new() -> Self {
        let player = Box::new(Player::new());

        //let mytilemap = Box::new(TileMap::new(2));
        let bg = Box::new(Background::new("map.jpg"));

        // setup subscriptions
        //let mut input = Input::new();
        //input.subscribe(player.observer.clone(), vec!["up", "down", "left", "right"]);
        //input.subscribe(bg.observer.clone(), vec!["up", "down", "left", "right"]);

        TownDemoGame {
            //input: input,
            gameobjects: vec![bg, player]
        }
    }
}

impl GameObject for TownDemoGame {
    // create the screen image
    fn render(&self) -> Option<MercuryImageBuffer> {
        let mut screen_img = im::RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

        for g in &self.gameobjects {
            
            if let Some(img) = g.render() {
                if let Some(pos) = g.position() {
                    let start = Instant::now();

                    #[cfg(target_family = "unix")]
                    im::imageops::overlay(&mut screen_img, &img.to_lib_rgba_image(), pos.x as i64, pos.y as i64);
                    #[cfg(target_family = "wasm")]
                    im::imageops::overlay(&mut screen_img, &img.to_lib_rgba_image(), pos.x as u32, pos.y as u32);
                    
                    let duration = start.elapsed();
                    println!("overlay: {:?}", duration);
                } else {
                    continue;
                }
            } else {
                continue;
            }
            
        }

        return Some(MercuryImageBuffer::from_lib_rgba_image(&screen_img));
    }

    fn position(&self) -> Option<Rect> {
        return Some(Rect{x: 0.0, y: 0.0, w: SCREEN_WIDTH as f64, h: SCREEN_HEIGHT as f64});
    }

    fn update(&mut self) {
        for u in self.gameobjects.iter_mut() {
            u.update();
        }
    }
}


#[cfg(target_family = "unix")]
fn main() {
    run_desktop_platform(&mut TownDemoGame::new());
}

#[cfg(target_family = "wasm")]
fn main() {}

// some notes:
//
// Just remember that 32, Some(32), and None can all be passed into a function whose type implements Into<Option<i32>>.
// This pattern is a relatively easy way to implement optional/default arguments in Rust.
//
// copy_ex() is like copy() but with some more options
// mem::replace() can swap values of same type
//
// input example: https://github.com/PistonDevelopers/piston-examples/blob/master/examples/user_input/src/main.rs