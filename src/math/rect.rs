use super::size::Size;
use super::Position;

#[derive(Copy, Clone)]
pub struct Rect {
    position: Position,
    size: Size
}

impl Rect {
    /// Create a zero Rect.
    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Create a rect with the bounds.
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Rect {
            position: Position::new(x, y),
            size: Size::new(w, h)
        }
    }

    /// Clip a Rect into a bounds Rect.
    pub fn clip(&self, bounds: &Rect) -> Self {
        let b_max_x = bounds.max_x();
        let b_min_x = bounds.x();
        let b_max_y = bounds.max_y();
        let b_min_y = bounds.y();

        if self.x() > b_max_x || self.y() > b_max_y {
            return Self::zero();
        }

        let mut res = *self;

        if self.x() > b_max_x || self.y() > b_max_y {
            res.position.x = b_max_x;
        }

        Self::zero()
    }

    #[inline]
    pub fn x(&self) -> i32 { self.position.x }
    #[inline]
    pub fn y(&self) -> i32 { self.position.y }
    #[inline]
    pub fn w(&self) -> u32 { self.size.width }
    #[inline]
    pub fn h(&self) -> u32 { self.size.height }

    #[inline]
    pub fn max_x(&self) -> i32 { self.x() + self.w() as i32 }
    #[inline]
    pub fn max_y(&self) -> i32 { self.y() + self.h() as i32 }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.w() == 0 || self.h() == 0
    }
}
