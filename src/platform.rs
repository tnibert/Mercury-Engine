use crate::gameobject::GameObject;

/*
 * trait to hide platform implementation behind
 */
pub trait Platform {
    fn gameloop(&mut self, game: &mut dyn GameObject);
}