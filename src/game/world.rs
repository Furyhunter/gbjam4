use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

use ::math::Vector;
use ::input::InputState;

type EntityID = u32;

type Thinker = Fn(Rc<RefCell<World>>, EntityID, InputState) -> ();

pub struct World {
    entities: HashSet<EntityID>,
    positions: HashMap<EntityID, Vector>,
    velocities: HashMap<EntityID, Vector>,
    thinkers: HashMap<EntityID, Rc<Thinker>>,

    entity_counter: EntityID
}

macro_rules! make_component_funcs {
    ($getter:ident, $setter:ident, $component_type:ty, $field:ident) => (
        #[inline]
        pub fn $getter(&self, entity: EntityID) -> Option<$component_type> {
            if let Some(s) = self.$field.get(&entity) {
                Some(s.clone())
            } else {
                None
            }
        }

        #[inline]
        pub fn $setter(&mut self, entity: EntityID, value: $component_type) -> () {
            self.$field.insert(entity, value);
        }
    );
}

impl World {
    pub fn new() -> Self {
        World {
            entities: HashSet::with_capacity(512),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            thinkers: HashMap::new(),

            entity_counter: 0
        }
    }

    /// Clone the entity set for iteration. Needed for safety. Probably could be
    /// more optimized.
    pub fn clone_entities(&self) -> HashSet<EntityID> {
        self.entities.clone()
    }

    /// Allocates an entity ID. Does not set any components.
    pub fn create_entity(&mut self) -> EntityID {
        let mut i = self.entity_counter;
        while self.entities.contains(&i) {
            i = i.wrapping_add(1);
        }

        if !self.entities.insert(i) {
            error!("Entity ID {} re-inserted? This shouldn't happen.", i);
        }

        self.entity_counter = i.wrapping_add(1);
        i
    }

    make_component_funcs!(position, set_position, Vector, positions);
    make_component_funcs!(velocity, set_velocity, Vector, velocities);
    make_component_funcs!(thinker, set_thinker, Rc<Thinker>, thinkers);
}