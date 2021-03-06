use super::image::Image;
use super::Color;

use ::math::size::Size;

use std::convert::From;

pub struct Screen {
    pub image: Image,
    pub colors: [Color; 4]
}

impl Screen {
    pub fn new() -> Screen {
        let image = Image::new((160, 144), 0u8);
        let colors = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ];

        Screen {
            image: image,
            colors: colors
        }
    }
}
