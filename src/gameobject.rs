use crate::collision::Rect;
use crate::mercurygraphics::mercuryimagebuffer::MercuryImageBuffer;

pub trait GameObject {
    // this will be inefficient giving ownership of the image each time
    // and therefore rerendering excessively
    // but we'll see how it goes and then refactor a bit
    fn render(&self) -> Option<MercuryImageBuffer>;

    fn position(&self) -> Option<Rect>;

    fn update(&mut self);
}

// also, it may be unwise to combine renderable and updateable into one trait
// but we'll start here