use crate::sprite::Sprite;
use crate::sprite::Direction;
use crate::observer::Listener;
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::rc::Rc;

const PLAYER_SPEED: f64 = 1.0;

// handles player, receives input signals
pub struct Player {
    spr: Sprite,
    pub observer: Rc<Listener>
}

impl Player {
    pub fn new() -> Player {
        Self {
            spr: Sprite::new("reaper.png", PLAYER_SPEED, Rect {
                x: (SCREEN_WIDTH / 2) as f64,
                y: (SCREEN_HEIGHT / 2) as f64,
                w: 100.0,
                h: 100.0
            }),         // todo: width and height are placeholders
            observer: Rc::new(Listener::new())
        }
    }
}

impl GameObject for Player {
    fn render(&self) -> Option<im::RgbaImage> {
        return self.spr.render();
    }

    fn position(&self) -> Option<Rect> {
        return self.spr.position();
    }

    fn update(&mut self) {
        for e in self.observer.poll_evt() {
            match e.as_str() {
                "up" => {
                    self.spr.set_facing(Direction::Up);
                },
                "down" => {
                    self.spr.set_facing(Direction::Down);
                },
                "left" => {
                    self.spr.set_facing(Direction::Left);
                },
                "right" => {
                    self.spr.set_facing(Direction::Right);
                },
                _ => ()
            }
        }

        self.spr.update();
    }
}