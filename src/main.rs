use bracket_lib::prelude::Point;
use ecs::component::{
    BlocksTile, CombatStats, Item, Monster, Name, Player, Position, Potion, Renderable, RunState, State, SufferDamage, Viewshed, WantsToMelee
};
use game::{gamelog, spawner};
use generation::map::{new_map_rooms_and_corridors, Map};
use specs::prelude::*;

mod ecs;
mod game;
mod generation;
mod ui;

fn main() -> bracket_lib::prelude::BError {
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();

    let map: Map = new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // player placdement
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);
    gs.ecs
        .insert(bracket_lib::prelude::RandomNumberGenerator::new());

    // monsters
    for room in map.rooms.iter().skip(1) {

        spawner::spawn_room(&mut gs.ecs, room);
    }

    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .with_fitscreen(true)
        .build()
        .unwrap();

    gs.ecs.insert(map);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_x, player_y)); // player position
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Halls of Lafferre".to_string()],
    });

    bracket_lib::prelude::main_loop(context, gs);

    // println!("Hello, world!");
    Ok(())
}
