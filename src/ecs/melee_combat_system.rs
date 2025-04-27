

use crate::{CombatStats, SufferDamage, WantsToMelee};
use bracket_lib::prelude::console;
use specs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage};

use super::component::Name;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut inflict_damage) = data;
        for (_entity, wants_melee, name, stats) in  (&entities, &wants_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    let damage = i32::max(0, stats.power - target_stats.defense);
                    if damage == 0 {
                        console::log(&format!(
                            "{} attacks {} but does no damage.",
                            &name.name, &target_name.name
                        ));
                    } else {
                        console::log(&format!(
                            "{} attacks {} for {} hit points.",
                            &name.name, &target_name.name, damage
                        ));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
