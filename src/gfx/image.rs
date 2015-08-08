use ::math::size::Size;

pub struct Image {
    buffer: Vec<u8>,
    size: Size
}

impl Image {
    pub fn new(size: Size) -> Self {
        let buffer = size.buffer(0 as u8);
        Image {
            buffer: buffer,
            size: size
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, position: (u32, u32), color: u8) -> Result<(), String> {
        let i = try!(self.index_of_position(position));

        self.buffer[i as usize] = color;
        Ok(())
    }

    #[inline]
    fn index_of_position(&self, position: (u32, u32)) -> Result<u32, String> {
        if (position.0 > self.size.width || position.1 > self.size.height) {
            return Err("error out of range".to_string());
        }
        Ok(position.1 * self.size.width + position.0)
    }
}
