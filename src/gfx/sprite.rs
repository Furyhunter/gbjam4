use ::gfx::image::{ImageDelegate, Image, SubImage};
use ::gfx::blit::Blit;

use ::math::Position;
use ::math::rect::Rect;

pub struct Sprite {
    image: ImageDelegate,
    offset: Position
}

impl Sprite {
    pub fn new(im: ImageDelegate, offset: Position) -> Self {
        Sprite {
            image: im,
            offset: offset
        }
    }
}

impl Blit for Sprite {
    fn blit_to(&self, src: Option<Rect>, target: &mut Image, dst: Option<Rect>) -> () {
        // Offset the dst by our own offset
        let dst: Rect = if let Some(r) = dst {
            Rect::new(r.x() + self.offset.x, r.y() + self.offset.y, r.w(), r.h())
        } else {
            Rect::new(self.offset.x, self.offset.y, 0, 0)
        };

        self.image.blit_to(src, target, Some(dst));
    }
}
