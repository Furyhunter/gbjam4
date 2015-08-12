use std::path::PathBuf;
use std::error::Error;

use find_folder::Search;

use image;
use image::{DynamicImage, GenericImage, Rgba, Pixels, Pixel, GrayAlphaImage};

use ::gfx::image::Image;

pub fn load_image(path: PathBuf) -> Result<Image, String> {
    let (dims, buffer) = match image::open(path) {
        Ok(i) => (i.dimensions(), i.to_luma_alpha()),
        Err(e) => return Err(e.description().to_string())
    };

    // convert DynamicImage to Image
    let mut image = Image::new(dims, 0u8);
    for (i, ps) in buffer.chunks(2).enumerate() {
        if ps[1] > 240 {
            try!(image.set_index(i, ps[0] / 64));
        } else {
            try!(image.set_index(i, 4));
        }
    }

    Ok(image)
}
