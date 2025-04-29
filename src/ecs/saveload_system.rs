use std::fs::File;
use specs::saveload::SerializeComponents;
use specs::error::NoError;
use specs::{
    saveload::{MarkedBuilder, SimpleMarker},
    Builder, World, WorldExt,
};

use crate::{ecs::component::{BlocksTile, CombatStats, InBackpack, Item, Monster, Name, Player, Position, Renderable, SerializationHelper, SufferDamage, Viewshed, WantsToDropItem, WantsToMelee, WantsToPickupItem}, generation::map::Map};

use super::component::SerializeMe;

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


}
