use bracket_lib::prelude::GameState;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Entity,
};
use specs_derive::{Component, ConvertSaveload};

use crate::game::player::player_input;
use crate::generation::map::{draw_map, Map};
use crate::ui::gui;

use super::damage_system::{self, DamageSystem};
use super::inventory_system::ItemCollectionSystem;
use super::melee_combat_system::MeleeCombatSystem;
use super::{
    map_indexing_system::MapIndexingSystem, monster_ai_system::MonsterAI,
    view_systems::VisibilitySystem,
};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Component)]
pub struct Renderable {
    pub glyph: bracket_lib::prelude::FontCharType,
    pub fg: bracket_lib::prelude::RGB,
    pub bg: bracket_lib::prelude::RGB,
}
#[derive(Component, Debug)]
pub struct Player {}
#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct Potion {
    pub heal_amount: i32,
}

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

pub struct Steps {
    pub count: i32,
}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
}

pub struct State {
    pub ecs: World,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<bracket_lib::prelude::Point>,
    pub range: i32,
    pub dirty: bool,
}

impl State {
    pub fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                if gui::show_inventory(self, ctx) == gui::ItemMenuResult::Cancel {
                    newrunstate = RunState::AwaitingInput;
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        damage_system::DamageSystem::delete_the_dead(&mut self.ecs);
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            };
        }
        gui::draw_ui(&self.ecs, ctx);
    }
}
