use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::convert::Into;

pub mod size;
pub mod rect;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Position {
        Position {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        self + -rhs
    }
}

impl Into<Vector> for Position {
    fn into(self) -> Vector {
        Vector {
            x: self.x as f32,
            y: self.y as f32
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        self + -rhs
    }
}

impl Into<Position> for Vector {
    fn into(self) -> Position {
        Position {
            x: self.x.round() as i32,
            y: self.y.round() as i32
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x: x, y: y }
    }
}
