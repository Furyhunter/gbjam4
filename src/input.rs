#[derive(Clone, Copy, Debug)]
pub struct InputState {
    pub left: PressedState,
    pub right: PressedState,
    pub up: PressedState,
    pub down: PressedState,
    pub a: PressedState,
    pub b: PressedState,
    pub start: PressedState,
    pub select: PressedState
}

impl InputState {
    pub fn new() -> InputState {
        use self::PressedState::*;
        InputState {
            left: Up,
            right: Up,
            up: Up,
            down: Up,
            a: Up,
            b: Up,
            start: Up,
            select: Up
        }
    }

    pub fn update(&mut self) {
        self.left.update();
        self.right.update();
        self.up.update();
        self.down.update();
        self.a.update();
        self.b.update();
        self.start.update();
        self.select.update();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PressedState {
    Up,
    Pressed,
    Held
}

impl PressedState {
    pub fn update(&mut self) {
        use self::PressedState::*;
        *self = match *self {
            Up => Up,
            Pressed => Held,
            Held => Held
        };
    }
}
