extern crate specs;
// extern crate specs_derive;

// #[macro_use]

use specs::{Component, VecStorage};
use specs::World;
use specs::{WriteStorage, ReadStorage, System};
use specs::RunNow;
// use specs_derive;
use specs::DispatcherBuilder;

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;
    // type SystemVel = ReadStorage<'a, Velocity>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        use specs::Join;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
            println!("{:?} and {:?}", &pos, &vel);
        }
    }
}

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
    // initialize
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    // create an entity with a position
    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    world.create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    // dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .add(HelloWorld, "hello_world", &[])
        .add(UpdatePos, "update_pos", &["hello_world"])
        .build();
    dispatcher.dispatch(&mut world.res);

    // create a world and let it run
    // let mut hello_world = HelloWorld;
    // hello_world.run_now(&world.res);

}