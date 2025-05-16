use std::fs::{self, File};
use std::path::Path;
use bracket_lib::prelude::Point;
use specs::saveload::{SerializeComponents, SimpleMarkerAllocator};
use specs::error::NoError;
use specs::{Entity, Join};
use specs::{
    saveload::{MarkedBuilder, SimpleMarker},
    Builder, World, WorldExt,
};

use crate::generation::map::MAPCOUNT;
use crate::{ecs::component::{BlocksTile, CombatStats, InBackpack, Item, Monster, Name, Player, Position, Renderable, SerializationHelper, SufferDamage, Viewshed, WantsToDropItem, WantsToMelee, WantsToPickupItem}, generation::map::Map};

use super::component::SerializeMe;
use specs::saveload::DeserializeComponents;

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
            SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
                &( $ecs.read_storage::<$type>(),),
                &$data.0,
                &$data.1,
                &mut $ser,
            )
            .unwrap();
        )*
    };
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}



pub fn save_game(ecs: &mut World) {
    let mapcopy = ecs.get_mut::<Map>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper { map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());

        let writer = File::create("./savegame.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(ecs, serializer, data, Position, Renderable, Player, Viewshed, Monster,
            Name, BlocksTile, CombatStats, WantsToMelee, SufferDamage, Item, InBackpack, WantsToPickupItem,
            WantsToDropItem, SerializationHelper 
        );
    }
    ecs.delete_entity(savehelper).expect("Crash on cleanup");


}

pub fn load_game(ecs: &mut World) {
    {
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }
    let data = fs::read_to_string("./savegame.json").unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);

    {
        let mut d = (&mut ecs.entities(), &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>());

        deserialize_individually!(ecs, de, d, Position, Renderable, Player, Viewshed, Monster,
            Name, BlocksTile, CombatStats, WantsToMelee, SufferDamage, Item, InBackpack, WantsToPickupItem,
            WantsToDropItem, SerializationHelper 
        );
    }

    let mut deleteme: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e,h) in (&entities, &helper).join() {
            let mut worldmap = ecs.write_resource::<Map>();
            *worldmap = h.map.clone();
            worldmap.tile_content = vec![Vec::new(); MAPCOUNT];
            deleteme = Some(e);
        }
        for (e,_p,pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<Point>();
            *ppos = Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    
    }
    ecs.delete_entity(deleteme.unwrap()).expect("Unable to delete helper");
}

pub fn delete_save() {
    if Path::new("./savegame.json").exists() { std::fs::remove_file("./savegame.json").expect("Unable to delete file"); } 
}


pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}

use specs::prelude::*;
use crate::ecs::component::*;
use crate::game::{gamelog, spawner};
use crate::generation::map::{new_map_rooms_and_corridors};

pub fn new_game(ecs: & mut World) {
    {
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }
    // Register all component types
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
    ecs.register::<Monster>();
    ecs.register::<Name>();
    ecs.register::<BlocksTile>();
    ecs.register::<CombatStats>();
    ecs.register::<WantsToMelee>();
    ecs.register::<SufferDamage>();
    ecs.register::<Item>();
    ecs.register::<Potion>();
    ecs.register::<InBackpack>();
    ecs.register::<WantsToPickupItem>();
    ecs.register::<WantsToDrinkPotion>();
    ecs.register::<WantsToDropItem>();
    ecs.register::<SimpleMarker<SerializeMe>>();
    ecs.register::<SerializationHelper>();

    ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    ecs.insert(bracket_lib::prelude::RandomNumberGenerator::new());

    // Create map and get starting position
    let map: Map = new_map_rooms_and_corridors(1);
    let (player_x, player_y) = map.rooms[0].center();

    // Spawn player
    let player_entity = spawner::player(ecs, player_x, player_y);

    // Spawn monsters/items in other rooms
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(ecs, room);
    }

    // Insert world resources
    ecs.insert(map);
    ecs.insert(player_entity);
    ecs.insert(Point::new(player_x, player_y)); // player position
    ecs.insert(RunState::PreRun);
    ecs.insert(gamelog::GameLog {
        entries: vec!["Halls of Lafferre".to_string()],
    });
}
