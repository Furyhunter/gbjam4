pub mod image;
pub mod screen;
pub mod palettes;
pub mod blit;
pub mod sprite;

const SCREEN_TOTAL_PIXELS: isize = 160 * 144;

pub type Color = [u8; 4];
