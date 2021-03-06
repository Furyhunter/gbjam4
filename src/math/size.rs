use std::ops::{Add, Sub};
use std::convert::Into;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size {
        Size { width: self.width + rhs.width, height: self.height + rhs.height }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, rhs: Size) -> Size {
        Size {
            width: self.width.saturating_sub(rhs.width),
            height: self.height.saturating_sub(rhs.height)
        }
    }
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width: width, height: height }
    }

    pub fn buffer<T: Copy>(&self, initial: T) -> Vec<T> {
        let mut buffer = Vec::with_capacity(self.elements());
        for _ in 0..self.elements() {
            buffer.push(initial);
        }

        buffer
    }

    pub fn elements(&self) -> usize {
        (self.width * self.height) as usize
    }
}

impl Into<Size> for (u32, u32) {
    fn into(self) -> Size {
        Size { width: self.0, height: self.1 }
    }
}
