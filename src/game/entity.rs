use ::math::Vector;
use ::game::Game;

use std::cell::RefCell;

pub struct Entity {
    pub pos: Vector
    //pub thinker: Box<(FnMut(&mut Entity) -> ())>
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            pos: Vector::new(0.0, 0.0)
            // thinker: Box::new(no_op_thinker)
        }
    }

    pub fn update(&mut self) {
        // let c: Box<FnMut(&mut Entity) -> ()> = self.thinker.clone();
        // c.call_mut((self,));
    }
}

// pub fn no_op_thinker(entity: &mut Entity) -> () {
//
// }
