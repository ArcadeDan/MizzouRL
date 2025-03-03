use specs::prelude::*;
use ecs::component::{Player, Position, Renderable, State, Viewshed};
use generation::map::{new_map_rooms_and_corridors, Map};

mod ecs;
mod generation;


fn main() -> bracket_lib::prelude::BError {
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: Map = new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .with_fitscreen(true)
        .build()
        .unwrap();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('@'),
            fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::YELLOW),
            bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    bracket_lib::prelude::main_loop(context, gs);

    // println!("Hello, world!");
    Ok(())
}
