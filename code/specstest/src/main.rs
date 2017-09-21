extern crate specs;
// extern crate specs_derive;

// #[macro_use]

use specs::{Component, VecStorage};
use specs::World;
// use specs_derive;

#[derive(Debug)]
// #[derive(Component,Debug)]
// #[component(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
}
