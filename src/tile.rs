use crate::gameobject::GameObject;
use crate::collision::Rect;

pub const TILE_SIZE: u32 = 32;       // all tiles are square

/*
A key question is whether Tile should know if it is permeable.
Given that a Tile will typically represent some sort of material in
game, I think the answer is yes at this stage.
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    size: u32,                      // size of one side of square
    image: im::RgbaImage,
    permeable: bool
}

impl Tile {
    pub fn new<F: Fn() -> im::RgbaImage>(create_image: F, is_permeable: bool) -> Self {
        Tile {
            size: TILE_SIZE,
            image: create_image(),
            permeable: is_permeable
        }
    }

    pub fn is_permeable(&self) -> bool {
        self.permeable
    }
}

impl GameObject for Tile {
    fn render(&self) -> Option<im::RgbaImage> {
        return Some(self.image.clone());
    }

    fn position(&self) -> Option<Rect> {
        return None;
    }

    fn update(&mut self) {}
}

