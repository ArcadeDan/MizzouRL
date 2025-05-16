use bracket_lib::prelude::Point;
use ecs::component::{
    BlocksTile, CombatStats, InBackpack, Item, Monster, Name, Player, Position, Potion, Renderable, RunState, SerializationHelper, SerializeMe, State, SufferDamage, Viewshed, WantsToDrinkPotion, WantsToDropItem, WantsToMelee, WantsToPickupItem
};
use game::{gamelog, spawner};
use generation::map::{new_map_rooms_and_corridors, Map};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod ecs;
mod game;
mod generation;
mod ui;

use ecs::saveload_system::new_game;

extern crate serde;

fn main() -> bracket_lib::prelude::BError {
    let mut gs = State { ecs: World::new() };

    ecs::saveload_system::new_game(&mut gs.ecs);

    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .with_fitscreen(true)
        .build()
        .unwrap();

    bracket_lib::prelude::main_loop(context, gs)
}

