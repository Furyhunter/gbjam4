use std::rc::Rc;
use std::cell::RefCell;

use ::math::size::Size;
use ::math::rect::Rect;
use ::math::Position;

use ::gfx::blit::Blit;

pub enum ImageDelegate {
    ImageBuf(Rc<Image>),
    ImageRef(SubImage)
}

/// Pixels are stored row first in the buffer.
pub struct Image {
    pub buffer: Vec<u8>,
    size: Size
}

pub struct SubImage {
    pub image: Rc<Image>,
    rect: Rect
}

impl Clone for SubImage {
    fn clone(&self) -> Self {
        SubImage {
            image: self.image.clone(),
            rect: self.rect
        }
    }
}

impl Clone for ImageDelegate {
    fn clone(&self) -> Self {
        use self::ImageDelegate::*;
        match *self {
            ImageBuf(ref buf) => ImageBuf(buf.clone()),
            ImageRef(ref sub) => ImageRef(sub.clone())
        }
    }
}

impl Image {
    pub fn new<S: Into<Size>>(size: S, color: u8) -> Self {
        let s = size.into();
        let buffer = s.buffer(color);
        Image {
            buffer: buffer,
            size: s
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, position: (u32, u32), color: u8) -> Result<(), String> {
        let i = try!(self.index_of_position(position));

        self.buffer[i as usize] = color;
        Ok(())
    }

    #[inline]
    pub fn set_index(&mut self, index: usize, color: u8) -> Result<(), String> {
        if let Some(p) = self.buffer.get_mut(index) {
            *p = color;
            Ok(())
        } else {
            Err("Invalid index".to_string())
        }
    }

    #[inline]
    pub fn get_pixel(&self, position: (u32, u32)) -> Result<u8, String> {
        let i = try!(self.index_of_position(position));

        Ok(self.buffer[i as usize])
    }

    #[inline]
    fn index_of_position(&self, position: (u32, u32)) -> Result<u32, String> {
        if position.0 > self.size.width || position.1 > self.size.height {
            return Err("error out of range".to_string());
        }
        Ok(position.1 * self.size.width + position.0)
    }
}

impl Blit for Image {
    fn blit_to(&self, src: Option<Rect>, target: &mut Image, dst: Option<Rect>) -> () {

        let src_rect: Rect = match src {
            Some(s) => s.into(),
            None => Rect::new(0, 0, self.size.width, self.size.height)
        };

        let dest_rect: Rect = match dst {
            Some(s) => s.into(),
            None => Rect::new(0, 0, target.size.width, target.size.height)
        };

        let clip_rect = src_rect.clip(&dest_rect);

        if clip_rect.is_zero() {
            return;
        }

        // TODO optimize
        for iy in clip_rect.y()..clip_rect.max_y() {
            for ix in clip_rect.x()..clip_rect.max_x() {
                let color = match self.get_pixel((ix as u32, iy as u32)) {
                    Ok(c) => c,
                    Err(s) => panic!("This shouldn't happen: {}", s)
                };
                if color > 3 { continue; }

                match target.set_pixel((ix as u32, iy as u32), color) {
                    Err(s) => panic!("This shouldn't happen: {}", s),
                    _ => ()
                };
            }
        }
    }
}

impl Blit for ImageDelegate {
    fn blit_to(&self, src: Option<Rect>, target: &mut Image, dst: Option<Rect>) -> () {
        match *self {
            ImageDelegate::ImageBuf(ref i) => i.blit_to(src, target, dst),
            ImageDelegate::ImageRef(ref i) => {
                warn!("unimplemented, fall back to image blit");
                i.image.blit_to(src, target, dst)
            }
        }
    }
}
