// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use specs;

// Part of a bigger structure
// use in modules
#[derive(Debug)]
pub struct Partof {
    id: specs::world::Index,
}

impl Partof {
    pub fn new(id: specs::world::Index) -> Self {
        Partof { id }
    }

    pub fn modify(&mut self, id: specs::world::Index) {
        self.id = id;
    }
}

impl specs::Component for Partof {
    type Storage = specs::VecStorage<Partof>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::prelude::*;

    #[test]
    fn create_partof_component() {
        let mut world = specs::World::new();
        world.register::<Partof>();
        world.create_entity().with(Partof::new(1)).build();
    }

    #[test]
    fn modify_partof_component() {
        let mut part = Partof::new(1);
        part.modify(2);
        assert_eq!(part.id, 2);
    }
}
