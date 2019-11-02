extern crate specs;
#[macro_use]
extern crate specs_derive;

use specs::DispatcherBuilder;
use specs::RunNow;
use specs::{Builder, World};
use specs::{Component, VecStorage};
use specs::{Entities, LazyUpdate, NullStorage, Read, ReadStorage, System, WriteStorage};

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
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (delta, vel, mut pos) = data;

        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
            println!("{:?} and {:?}", &pos, &vel);
        }
    }
}

// struct UpdatePos2;
//
// impl<'a> System<'a> for UpdatePos2 {
//     type SystemData = (
//         Fetch<'a, DeltaTime>,
//         ReadStorage<'a, Velocity>,
//         WriteStorage<'a, Position>,
//     );
//
//     fn run(&mut self, data: Self::SystemData) {
//         use specs::Join;
//         let (delta, vel, mut pos) = data;
//
//         // `Fetch` implements `Deref`, so it
//         // coerces to `&DeltaTime`.
//         let delta = delta.0;
//
//         for (vel, pos) in (&vel, &mut pos).join() {
//             pos.x += vel.x * delta;
//             pos.y += vel.y * delta;
//             println!("{:?} and {:?}", &pos, &vel);
//         }
//     }
// }

// #[derive(Debug)]
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }

// #[derive(Debug)]
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

// impl Component for Velocity {
//     type Storage = VecStorage<Self>;
// }

#[derive(Debug, Default)]
struct DeltaTime(f32);
#[derive(Debug, Default)]
struct Stone;
impl Component for Stone {
    type Storage = NullStorage<Self>;
}

struct StoneCreator;
impl<'a> System<'a> for StoneCreator {
    type SystemData = (Entities<'a>, WriteStorage<'a, Stone>, Read<'a, LazyUpdate>);

    fn run(&mut self, (entities, mut stones, updater): Self::SystemData) {
        let stone = entities.create();

        // 1) Either we insert the component by writing to its storage
        stones.insert(stone, Stone).unwrap();

        // 2) or we can lazily insert it with `LazyUpdate`
        updater.insert(stone, Stone);
        println!("there is a stone",);
    }
}

struct Gravity;

fn main() {
    // initialize
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Stone>();

    // // create an entity with a position
    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    // // Resources
    world.add_resource(DeltaTime(0.05)); // Let's use some start value
                                         // {
                                         //     let mut delta = world.write_resource::<DeltaTime>();
                                         //     *delta = DeltaTime(0.04);
                                         // }
                                         //
    world.add_resource(Gravity);

    for _ in 0..5 {
        world.create_entity().with(Velocity).build();
    }

    // dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .with(StoneCreator, "make_a_stone", &[])
        .build();
    dispatcher.dispatch(&mut world.res);
    // and run
    world.maintain();
}
