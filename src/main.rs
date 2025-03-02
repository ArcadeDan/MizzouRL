use specs::prelude::*;
use Ecs::component::{Player, Position, Renderable, State};
use Generation::map::new_map_rooms_and_corridors;

mod Ecs;
mod Generation;

fn main() {
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map_rooms_and_corridors());

    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .build()
        .unwrap();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('@'),
            fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::YELLOW),
            bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
        })
        .with(Player {})
        .build();

    bracket_lib::prelude::main_loop(context, gs);

    println!("Hello, world!");
}
