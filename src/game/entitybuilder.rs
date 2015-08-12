use std::rc::Rc;
use std::ops::DerefMut;

use ::game::world::*;

use ::math::Vector;

use ::gfx::image::ImageDelegate;

pub struct EntityBuilder<'a> {
    i: EntityID,
    w: &'a mut World
}

macro_rules! builder_gen_function {
    ($name:ident, $setter:ident, $t:ty) => (
        #[inline]
        pub fn $name(&mut self, v: $t) -> &mut Self {
            self.w.$setter(self.i, v);
            self
        }
    )
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World) -> EntityBuilder<'a> {
        EntityBuilder {
            i: world.create_entity(),
            w: world
        }
    }

    pub fn finish(&self) -> EntityID {
        self.i
    }

    builder_gen_function!(position, set_position, Vector);
    builder_gen_function!(velocity, set_velocity, Vector);
    builder_gen_function!(thinker, set_thinker, Rc<Thinker>);
    builder_gen_function!(drawer, set_drawer, Rc<Drawer>);
    builder_gen_function!(sprite, set_sprite, ImageDelegate);
}
