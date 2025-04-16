use specs::prelude::*;
use crate::{Viewshed, Position, Map, Monster};
use bracket_lib::prelude::{console, Point, field_of_view};


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data : Self::SystemData) {
        let (player_pos, viewshed,monster) = data;

        for (viewshed, _monster) in (&viewshed, &monster).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("Monster shouts slurs"));
            }
        }
    }
}