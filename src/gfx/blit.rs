use ::math::rect::Rect;
use ::gfx::image::Image;

pub trait Blit {
    fn blit_to(&self, src: Option<Rect>, target: &mut Image, dst: Option<Rect>) -> ();
}
